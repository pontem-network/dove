use move_ir_types::location::{Loc, Span};
use move_lang::parser::ast::{ModuleAccess, ModuleAccess_, ModuleName, Type, Type_, Use};
use move_lang::parser::syntax::spanned;
use move_lang::shared::Address;

use dsl::parser::parse;
use dsl::parser::types::{Ast, Call, Instruction, Struct, Value, Value_, Var};

mod common;
use common::*;

#[test]
pub fn parse_empty_dsl() {
    let ast = parse("{}", "dsl").unwrap();
    assert_eq!(
        ast,
        Ast {
            loc: loc(0, 2),
            instructions: vec![],
        }
    );

    let ast = parse(
        "        {\
    \
    \
    }   ",
        "dsl",
    )
    .unwrap();
    assert_eq!(
        ast,
        Ast {
            loc: Loc::new("dsl", Span::new(8, 10)),
            instructions: vec![],
        }
    );

    let err = parse("{", "dsl").unwrap_err();
    assert_eq!(
        err,
        vec![
            (loc(1, 1), "Unexpected end-of-file".to_owned()),
            (
                loc(1, 1),
                "Expected one of `use`, `let`, `address`, or `identifier`".to_owned()
            ),
        ]
    );

    let err = parse("}", "dsl").unwrap_err();
    assert_eq!(
        err,
        vec![
            (loc(0, 1), "Unexpected '}'".to_owned()),
            (loc(0, 1), "Expected '{'".to_owned()),
        ]
    );
}

#[test]
pub fn parse_use_dsl() {
    let ast = parse(
        "{\
        use 0x1::Block;
        use 0x2::Block::Block;
        use 0x4::Mod::{BAR, FOO, T as D};
        use 0x4::Mod::{T as D};



        use 0x2::Block::Block as Block;
        use 0x2::Block as Block;

    }",
        "dsl",
    )
    .unwrap();
    assert_eq!(
        ast,
        Ast {
            loc: Loc::new("dsl", Span::new(0, 204)),
            instructions: vec![
                use_(
                    1,
                    16,
                    Use::Module(module(5, 15, 10, 15, "0x1", "Block"), None),
                ),
                use_(
                    25,
                    47,
                    Use::Members(
                        module(29, 39, 34, 39, "0x2", "Block"),
                        vec![(name("Block"), None)],
                    ),
                ),
                use_(
                    56,
                    89,
                    Use::Members(
                        module(60, 68, 65, 68, "0x4", "Mod"),
                        vec![
                            (name("BAR"), None),
                            (name("FOO"), None),
                            (name("T"), Some(name("D")))
                        ],
                    ),
                ),
                use_(
                    98,
                    121,
                    Use::Members(
                        module(102, 110, 107, 110, "0x4", "Mod"),
                        vec![(name("T"), Some(name("D")))],
                    ),
                ),
                use_(
                    133,
                    164,
                    Use::Members(
                        module(137, 147, 142, 147, "0x2", "Block"),
                        vec![(name("Block"), Some(name("Block")))],
                    ),
                ),
                use_(
                    173,
                    197,
                    Use::Module(
                        module(177, 187, 182, 187, "0x2", "Block"),
                        Some(ModuleName(name("Block"))),
                    ),
                ),
            ],
        }
    );
}

#[test]
pub fn parse_use_dsl_err() {
    let err = parse(
        "{\


        use 0x1:Block;


    }",
        "dsl",
    )
    .unwrap_err();
    assert_eq!(
        err,
        vec![
            (loc(8, 9), "Unexpected ':'".to_owned()),
            (loc(8, 9), "Expected '::'".to_owned()),
        ]
    );
}

#[test]
pub fn test_fun_call() {
    let ast = parse(
        "{\
        init();
        Block::init();
        0x1::Block::init();

        test<>();
        test<u8>();
        test<0x1::Block::T<Block::T>, T<T, u8>>();

        test(0, true, foo, [], {value: 100}, 0x1);
        test(foo,);
    }",
        "dsl",
    )
    .unwrap();
    assert_eq!(
        ast,
        Ast {
            loc: Loc::new("dsl", Span::new(0, 227)),
            instructions: vec![
                func(
                    1,
                    8,
                    Call {
                        name: access_name(1, 8, "init"),
                        t_params: None,
                        params: vec![],
                    },
                ),
                func(
                    17,
                    31,
                    Call {
                        name: access_mod_name(1, 8, "Block", "init"),
                        t_params: None,
                        params: vec![],
                    },
                ),
                func(
                    40,
                    59,
                    Call {
                        name: access_addr_mod_name(
                            40, 59, 40, 50, 45, 50, "0x1", "Block", "init",
                        ),
                        t_params: None,
                        params: vec![],
                    },
                ),
                func(
                    69,
                    78,
                    Call {
                        name: access_name(1, 8, "test"),
                        t_params: Some(vec![]),
                        params: vec![],
                    },
                ),
                func(
                    87,
                    98,
                    Call {
                        name: access_name(1, 8, "test"),
                        t_params: Some(vec![tp("u8")]),
                        params: vec![],
                    },
                ),
                func(
                    107,
                    149,
                    Call {
                        name: access_name(1, 8, "test"),
                        t_params: Some(vec![
                            tp_mod_access(
                                access_addr_mod_name(
                                    107, 149, 112, 122, 117, 122, "0x1", "Block", "T",
                                ),
                                vec![tp_mod_access(access_mod_name(0, 0, "Block", "T"), vec![])],
                            ),
                            tp_mod_access(
                                access_name(107, 149, "T"),
                                vec![tp_mod_access(access_name(0, 0, "T"), vec![]), tp("u8")],
                            ),
                        ]),
                        params: vec![],
                    },
                ),
                func(
                    159,
                    201,
                    Call {
                        name: access_name(1, 8, "test"),
                        t_params: None,
                        params: vec![
                            val(Value_::Num(0)),
                            val(Value_::Bool(true)),
                            val(Value_::Var("foo".to_owned())),
                            val(Value_::Vec(vec![])),
                            val(Value_::Struct(Struct {
                                fields: vec![(name("value"), val(Value_::Num(100)))]
                            })),
                            val(Value_::Address(Address::DIEM_CORE))
                        ],
                    }
                ),
                func(
                    210,
                    221,
                    Call {
                        name: access_name(1, 8, "test"),
                        t_params: None,
                        params: vec![val(Value_::Var("foo".to_owned())),],
                    }
                )
            ],
        }
    );
}

#[test]
pub fn test_var() {
    let ast = parse(
        "{\
        let a = 1;
        a = true;
        a={val:1};
        _a=[];
        b=a;
    }",
        "dsl",
    )
    .unwrap();

    assert_eq!(
        ast,
        Ast {
            loc: loc(0, 82),
            instructions: vec![
                (var(1, 11, "a", val(Value_::Num(1)))),
                (var(20, 29, "a", val(Value_::Bool(true)))),
                (var(
                    38,
                    48,
                    "a",
                    val(Value_::Struct(Struct {
                        fields: vec![(name("val"), val(Value_::Num(1)))]
                    }))
                )),
                (var(57, 63, "_a", val(Value_::Vec(vec![])))),
                (var(72, 76, "b", val(Value_::Var("a".to_owned())))),
            ],
        }
    );
}

fn use_(start: u32, end: u32, use_: Use) -> (Loc, Instruction) {
    (loc(start, end), Instruction::Use(use_))
}

fn func(start: u32, end: u32, call: Call) -> (Loc, Instruction) {
    (loc(start, end), Instruction::Call(call))
}

fn var(start: u32, end: u32, n: &str, val: Value) -> (Loc, Instruction) {
    (
        loc(start, end),
        Instruction::Var(Var {
            name: name(n),
            value: val,
        }),
    )
}

fn access_name(start: u32, end: u32, n: &str) -> ModuleAccess {
    spanned(
        "dsl",
        start as usize,
        end as usize,
        ModuleAccess_::Name(name(n)),
    )
}

fn access_mod_name(start: u32, end: u32, m: &str, n: &str) -> ModuleAccess {
    spanned(
        "dsl",
        start as usize,
        end as usize,
        ModuleAccess_::ModuleAccess(ModuleName(name(m)), name(n)),
    )
}

fn tp(name: &str) -> Type {
    spanned(
        "dsl",
        0,
        0,
        Type_::Apply(Box::new(access_name(0, 0, name)), vec![]),
    )
}

fn val(val: Value_) -> Value {
    Value::new(loc(0, 0), val)
}
