use move_ir_types::location::{Loc, Span};
use move_lang::errors::Error;
use move_lang::parser::lexer::{Lexer, Tok};
use move_lang::parser::syntax::{consume_token, make_loc, parse_use_decl, unexpected_token_error};

use crate::parser::types::{Ast, Instruction};
use crate::parser::func::parse_call;

pub mod types;
pub mod func;
pub mod value;


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
            Tok::Use => {
                Instruction::Use(parse_use_decl(tokens)?)
            }
            Tok::AddressValue => {
                if tokens.lookahead()? == Tok::ColonColon {
                    Instruction::Call(parse_call(tokens)?)
                } else {
                    // resource declaration.
                    todo!()
                }
            }
            Tok::Let => {
                // variable declaration.
                todo!()
            }
            Tok::IdentifierValue => {
                Instruction::Call(parse_call(tokens)?)
            }
            _ => return Err(unexpected_token_error(tokens, "one of `use`, `let`, `address`, or `identifier`")),
        };
        instructions.push(
            (make_loc(tokens.file_name(), start_loc, tokens.previous_end_loc()),
             inst)
        );
    }

    Ok(instructions)
}

