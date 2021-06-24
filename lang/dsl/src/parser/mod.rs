use move_ir_types::location::{Loc, Span};
use move_lang::errors::Error;
use move_lang::parser::lexer::{Lexer, Tok};
use move_lang::parser::syntax::{consume_token, make_loc, parse_use_decl, unexpected_token_error};

use crate::parser::func::parse_call;
use crate::parser::types::{Ast, Instruction};
use crate::parser::var::parse_var;
use crate::parser::store::{MOVE_TO, DROP, parse_move_to, parse_drop};

pub mod func;
pub mod store;
pub mod types;
pub mod value;
pub mod var;

pub fn parse(content: &str, name: &'static str) -> Result<Ast, Error> {
    let mut lexer = Lexer::new(content, name, Default::default());

    match lexer.advance() {
        Err(err) => Err(err),
        Ok(..) => Ok(()),
    }?;

    let start_loc = lexer.start_loc();

    consume_token(&mut lexer, Tok::LBrace)?;

    let instructions = parse_instructions(&mut lexer)?;

    consume_token(&mut lexer, Tok::RBrace)?;

    Ok(Ast {
        loc: make_loc(lexer.file_name(), start_loc, lexer.previous_end_loc()),
        instructions,
    })
}

pub fn map_error(global_offset: usize, err: &mut Error) {
    for (loc, _) in err.iter_mut() {
        loc.span = Span::new(
            (loc.span.start().to_usize() + global_offset) as u32,
            (loc.span.end().to_usize() + global_offset) as u32,
        );
    }
}

fn parse_instructions(tokens: &mut Lexer) -> Result<Vec<(Loc, Instruction)>, Error> {
    let mut instructions = vec![];

    while tokens.peek() != Tok::RBrace {
        let start_loc = tokens.start_loc();

        let inst = match tokens.peek() {
            Tok::Use => Instruction::Use(parse_use_decl(tokens)?),
            Tok::AddressValue => Instruction::Call(parse_call(tokens)?),
            Tok::Let => {
                tokens.advance()?;
                Instruction::Var(parse_var(tokens)?)
            }
            Tok::IdentifierValue => {
                if tokens.lookahead()? == Tok::Equal {
                    Instruction::Var(parse_var(tokens)?)
                } else {
                    match tokens.content() {
                        MOVE_TO => Instruction::Store(parse_move_to(tokens)?),
                        DROP => Instruction::Store(parse_drop(tokens)?),
                        _ => Instruction::Call(parse_call(tokens)?),
                    }
                }
            }
            _ => {
                return Err(unexpected_token_error(
                    tokens,
                    "one of `use`, `let`, `address`, or `identifier`",
                ));
            }
        };
        instructions.push((
            make_loc(tokens.file_name(), start_loc, tokens.previous_end_loc()),
            inst,
        ));
    }

    Ok(instructions)
}
