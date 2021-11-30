use anyhow::Result;
use move_core_types::account_address::AccountAddress;
use rust_base58::{ToBase58, FromBase58};

const SS58_PREFIX: &[u8] = b"SS58PRE";
const PUB_KEY_LENGTH: usize = 32;
const CHECK_SUM_LEN: usize = 2;

/// Convert address to ss58
/// 0xD43593C715FDD31C61141ABD04A99FD6822C8558854CCDE39A5684E7A56DA27D => 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
pub fn address_to_ss58(account: &AccountAddress) -> String {
    let mut ss58_address = [0; 35];
    ss58_address[0] = 42;
    ss58_address[1..33].copy_from_slice(&account.to_u8());
    let hash = ss58hash(&ss58_address[0..33]);
    ss58_address[33..35].copy_from_slice(&hash.as_bytes()[0..2]);
    ss58_address.to_base58()
}

fn ss58hash(data: &[u8]) -> blake2_rfc::blake2b::Blake2bResult {
    let mut context = blake2_rfc::blake2b::Blake2b::new(64);
    context.update(SS58_PREFIX);
    context.update(data);
    context.finalize()
}

pub fn ss58_to_address(ss58: &str) -> Result<AccountAddress> {
    let bs58 = match ss58.from_base58() {
        Ok(bs58) => bs58,
        Err(err) => return Err(anyhow!("Wrong base58:{}", err)),
    };
    ensure!(
        bs58.len() > PUB_KEY_LENGTH + CHECK_SUM_LEN,
        format!(
            "Address length must be equal or greater than {} bytes",
            PUB_KEY_LENGTH + CHECK_SUM_LEN
        )
    );
    let check_sum = &bs58[bs58.len() - CHECK_SUM_LEN..];
    let address = &bs58[bs58.len() - PUB_KEY_LENGTH - CHECK_SUM_LEN..bs58.len() - CHECK_SUM_LEN];

    if check_sum != &ss58hash(&bs58[0..bs58.len() - CHECK_SUM_LEN]).as_bytes()[0..CHECK_SUM_LEN] {
        return Err(anyhow!("Wrong address checksum"));
    }
    let mut addr = [0; PUB_KEY_LENGTH];
    addr.copy_from_slice(address);
    Ok(AccountAddress::new(addr))
}
