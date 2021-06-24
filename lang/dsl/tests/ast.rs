use move_ir_types::location::{Loc, Span};
use move_lang::parser::ast::{ModuleAccess, ModuleAccess_, ModuleIdent, ModuleName, Type, Type_, Use};
use move_lang::parser::syntax::spanned;
use move_lang::shared::{Address, Name};

use dsl::parser::parse;
use dsl::parser::types::{Ast, Call, Instruction};

#[test]
pub fn parse_empty_dsl() {
    let ast = parse("{}", "dsl").unwrap();
    assert_eq!(ast, Ast {
        loc: loc(0, 2),
        instructions: vec![],
    });

    let ast = parse(
        "        {\
    \
    \
    }   ",
        "dsl").unwrap();
    assert_eq!(ast, Ast {
        loc: Loc::new("dsl", Span::new(8, 10)),
        instructions: vec![],
    });

    let err = parse("{", "dsl").unwrap_err();
    assert_eq!(err,
               vec![
                   (loc(1, 1), "Unexpected end-of-file".to_owned()),
                   (loc(1, 1), "Expected one of `use`, `let`, `address`, or `identifier`".to_owned()),
               ]
    );

    let err = parse("}", "dsl").unwrap_err();
    assert_eq!(err,
               vec![
                   (loc(0, 1), "Unexpected '}'".to_owned()),
                   (loc(0, 1), "Expected '{'".to_owned()),
               ]
    );
}

#[test]
pub fn parse_use_dsl() {
    let ast = parse("{\
        use 0x1::Block;
        use 0x2::Block::Block;
        use 0x4::Mod::{BAR, FOO, T as D};
        use 0x4::Mod::{T as D};



        use 0x2::Block::Block as Block;
        use 0x2::Block as Block;

    }", "dsl").unwrap();
    assert_eq!(ast, Ast {
        loc: Loc::new("dsl", Span::new(0, 204)),
        instructions: vec![
            use_(1, 16, Use::Module(module(5, 15, 10, 15, "0x1", "Block"), None)),
            use_(25, 47, Use::Members(module(29, 39, 34, 39, "0x2", "Block"), vec![(name("Block"), None)])),
            use_(56, 89, Use::Members(module(60, 68, 65, 68, "0x4", "Mod"), vec![(name("BAR"), None), (name("FOO"), None), (name("T"), Some(name("D")))])),
            use_(98, 121, Use::Members(module(102, 110, 107, 110, "0x4", "Mod"), vec![(name("T"), Some(name("D")))])),
            use_(133, 164, Use::Members(module(137, 147, 142, 147, "0x2", "Block"), vec![(name("Block"), Some(name("Block")))])),
            use_(173, 197, Use::Module(module(177, 187, 182, 187, "0x2", "Block"), Some(ModuleName(name("Block"))))),
        ],
    });
}

#[test]
pub fn parse_use_dsl_err() {
    let err = parse("{\


        use 0x1:Block;


    }", "dsl").unwrap_err();
    assert_eq!(err,
               vec![
                   (loc(8, 9), "Unexpected ':'".to_owned()),
                   (loc(8, 9), "Expected '::'".to_owned()),
               ]
    );
}

#[test]
pub fn test_fun_call() {
    let ast = parse("{\
        init();
        Block::init();
        0x1::Block::init();

        test<>();
        test<u8>();
        test<0x1::Block::T<Block::T>, T<T, u8>>();
    }", "dsl").unwrap();
    assert_eq!(ast, Ast {
        loc: Loc::new("dsl", Span::new(0, 155)),
        instructions: vec![
            func(1, 8, Call {
                name: access_name(1, 8, "init"),
                t_params: None,
                params: vec![],
            }),
            func(17, 31, Call {
                name: access_mod_name(1, 8, "Block", "init"),
                t_params: None,
                params: vec![],
            }),
            func(40, 59, Call {
                name: access_addr_mod_name(40, 59, 40, 50, 45, 50, "0x1", "Block", "init"),
                t_params: None,
                params: vec![],
            }),
            func(69, 78, Call {
                name: access_name(1, 8, "test"),
                t_params: Some(vec![]),
                params: vec![],
            }),
            func(87, 98, Call {
                name: access_name(1, 8, "test"),
                t_params: Some(vec![tp("u8")]),
                params: vec![],
            }),
            func(107, 149, Call {
                name: access_name(1, 8, "test"),
                t_params: Some(vec![
                    tp_mod_access(access_addr_mod_name(107, 149,112, 122, 117, 122, "0x1", "Block", "T"), vec![tp_mod_access(access_mod_name(0, 0, "Block", "T"), vec![])]),
                    tp_mod_access(access_name(107, 149, "T"), vec![tp_mod_access(access_name(0, 0, "T"), vec![]), tp("u8")]),
                ]),
                params: vec![],
            }),
        ],
    });
}

fn loc(start: u32, end: u32) -> Loc {
    Loc::new("dsl", Span::new(start, end))
}

fn use_(start: u32, end: u32, use_: Use) -> (Loc, Instruction) {
    (loc(start, end), Instruction::Use(use_))
}

fn func(start: u32, end: u32, call: Call) -> (Loc, Instruction) {
    (loc(start, end), Instruction::Call(call))
}

fn addr(addr: &str) -> Address {
    Address::parse_str(addr).unwrap()
}

fn name(name: &str) -> Name {
    Name::new(loc(0, 0), name.to_owned())
}

fn module(start_1: u32, end_1: u32, start_2: u32, end_2: u32, adr: &str, name: &str) -> ModuleIdent {
    ModuleIdent {
        locs: (loc(start_1, end_1), loc(start_2, end_2)),
        value: (addr(adr), name.to_owned()),
    }
}

fn access_name(start: u32, end: u32, n: &str) -> ModuleAccess {
    spanned("dsl", start as usize, end as usize, ModuleAccess_::Name(name(n)))
}

fn access_mod_name(start: u32, end: u32, m: &str, n: &str) -> ModuleAccess {
    spanned("dsl", start as usize, end as usize, ModuleAccess_::ModuleAccess(ModuleName(name(m)), name(n)))
}

fn access_addr_mod_name(start: u32, end: u32, start_1: u32, end_1: u32, start_2: u32, end_2: u32, a: &str, m: &str, n: &str) -> ModuleAccess {
    spanned("dsl", start as usize, end as usize, ModuleAccess_::QualifiedModuleAccess(module(start_1, end_1, start_2, end_2, a, m), name(n)))
}

fn tp(name: &str) -> Type {
    spanned("dsl", 0, 0, Type_::Apply(Box::new(access_name(0, 0, name)), vec![]))
}

fn tp_mod_access(access: ModuleAccess, tps: Vec<Type>) -> Type {
    spanned("dsl", 0, 0, Type_::Apply(Box::new(access), tps))
}