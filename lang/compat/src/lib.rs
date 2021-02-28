#[macro_use]
extern crate anyhow;

use std::io::Cursor;
use anyhow::Result;
use diem_types::account_address::AccountAddress;
use move_core_types::vm_status::StatusCode;
use vm::errors::{BinaryLoaderResult, PartialVMError};
use vm::file_format_common::TableType;
use vm::file_format::SignatureToken;
use vm::deserializer::{check_binary, load_signature_token, load_constant_size};

mod context;
mod mutator;

use context::*;
use mutator::Mutator;

const DFIN_ADDR_LEN: usize = AccountAddress::LENGTH;
const LIBRA_ADDR_LEN: usize = 16;

pub fn adapt(bytes: &mut Vec<u8>) -> Result<()> {
    let mut cur = Cursor::new(bytes.as_slice());
    let mut mutator = Mutator::new();

    check_binary(&mut cur).map_err(|err| anyhow!("{:?}", err))?;
    make_diff(&mut cur, &mut mutator)?;
    mutator.mutate(bytes);
    Ok(())
}

fn make_diff(cur: &mut Cursor<&[u8]>, mutator: &mut Mutator) -> Result<()> {
    let table_len = read_uleb128_as_u64(cur)?;

    let header_len = cur.position() as u32;
    let header_size = calc_header_size(cur, table_len)?;

    let mut additional_offset: u32 = 0;
    for _ in 0..table_len {
        let kind = read_u8(cur)?;

        let offset = if additional_offset > 0 {
            let start_pos = cur.position();
            let offset = read_uleb128_as_u64(cur)? as u32;
            make_uleb128_diff(
                start_pos,
                cur.position(),
                offset + additional_offset,
                mutator,
            )?;
            offset
        } else {
            read_uleb128_as_u64(cur)? as u32
        };

        let t_len_start_pos = cur.position();
        let t_len = read_uleb128_as_u64(cur)? as u32;
        let t_len_end_pos = cur.position();

        let offset_diff = if kind == TableType::ADDRESS_IDENTIFIERS as u8 {
            handle_address_identifiers(
                TableContext::new(cur, offset + header_size + header_len, t_len),
                mutator,
            )
        } else if kind == TableType::CONSTANT_POOL as u8 {
            handle_const_pool(
                TableContext::new(cur, offset + header_size + header_len, t_len),
                mutator,
            )
            .map_err(|err| anyhow!("{:?}", err))?
        } else {
            0
        };

        if offset_diff > 0 {
            make_uleb128_diff(t_len_start_pos, t_len_end_pos, t_len + offset_diff, mutator)?;
        }

        additional_offset += offset_diff;
    }

    Ok(())
}

fn calc_header_size(cur: &mut Cursor<&[u8]>, table_len: u64) -> Result<u32> {
    let start = cur.position() as u32;

    for _ in 0..table_len {
        read_u8(cur)?;
        read_uleb128_as_u64(cur)?;
        read_uleb128_as_u64(cur)?;
    }

    let end = cur.position() as u32;
    cur.set_position(start as u64);
    Ok(end - start)
}

fn make_uleb128_diff(
    start_pos: u64,
    end_pos: u64,
    new_offset: u32,
    mutator: &mut Mutator,
) -> Result<()> {
    let mut binary = BinaryData::new();
    write_u64_as_uleb128(&mut binary, new_offset as u64)?;
    mutator.make_diff(start_pos as usize, end_pos as usize, binary.into_inner());
    Ok(())
}

fn handle_address_identifiers(ctx: TableContext, mutator: &mut Mutator) -> u32 {
    if ctx.len % (LIBRA_ADDR_LEN as u32) == 0 {
        for idx in (0..ctx.len).step_by(LIBRA_ADDR_LEN) {
            let index = ctx.position() + idx as usize;
            mutator.make_diff(index, index, vec![0x0, 0x0, 0x0, 0x0]);
        }
        ctx.len / (LIBRA_ADDR_LEN as u32) * (DFIN_ADDR_LEN - LIBRA_ADDR_LEN) as u32
    } else {
        0
    }
}

fn handle_const_pool(ctx: TableContext, mutator: &mut Mutator) -> BinaryLoaderResult<u32> {
    let end_offset = ctx.cursor.position() + ctx.len as u64;
    let mut additional_offset = 0;
    while ctx.cursor.position() < end_offset {
        let type_ = load_signature_token(ctx.cursor)?;

        let size_start_offset = ctx.cursor.position();
        let size = load_constant_size(ctx.cursor)? as u32;
        let size_end_offset = ctx.cursor.position();

        if SignatureToken::Address == type_ {
            let diff_size = (DFIN_ADDR_LEN - LIBRA_ADDR_LEN) as u32;
            make_uleb128_diff(
                size_start_offset,
                size_end_offset,
                size + diff_size,
                mutator,
            )
            .map_err(|err| {
                PartialVMError::new(StatusCode::MALFORMED).with_message(format!("{:?}", err))
            })?;
            additional_offset += diff_size;
            let index = ctx.cursor.position() as usize;
            mutator.make_diff(index, index, vec![0x0, 0x0, 0x0, 0x0]);
            ctx.cursor.set_position(ctx.cursor.position() + size as u64);
        } else {
            ctx.cursor.set_position(ctx.cursor.position() + size as u64);
        }
    }

    Ok(additional_offset)
}
