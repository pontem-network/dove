use move_ir_types::location::{Loc, Span};
use move_lang::shared::{Address, Name};
use move_lang::parser::syntax::spanned;
use move_lang::parser::ast::{ModuleAccess, ModuleAccess_, Type, Type_, ModuleIdent};

pub fn loc(start: u32, end: u32) -> Loc {
    Loc::new("dsl", Span::new(start, end))
}

pub fn addr(addr: &str) -> Address {
    Address::parse_str(addr).unwrap()
}

pub fn name(name: &str) -> Name {
    Name::new(loc(0, 0), name.to_owned())
}

pub fn access_addr_mod_name(
    start: u32,
    end: u32,
    start_1: u32,
    end_1: u32,
    start_2: u32,
    end_2: u32,
    a: &str,
    m: &str,
    n: &str,
) -> ModuleAccess {
    spanned(
        "dsl",
        start as usize,
        end as usize,
        ModuleAccess_::QualifiedModuleAccess(
            module(start_1, end_1, start_2, end_2, a, m),
            name(n),
        ),
    )
}

pub fn tp_mod_access(access: ModuleAccess, tps: Vec<Type>) -> Type {
    spanned("dsl", 0, 0, Type_::Apply(Box::new(access), tps))
}

pub fn module(
    start_1: u32,
    end_1: u32,
    start_2: u32,
    end_2: u32,
    adr: &str,
    name: &str,
) -> ModuleIdent {
    ModuleIdent {
        locs: (loc(start_1, end_1), loc(start_2, end_2)),
        value: (addr(adr), name.to_owned()),
    }
}
