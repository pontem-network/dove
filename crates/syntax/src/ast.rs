use core::fmt;

use tree_sitter::Node;
//
// pub trait AstNode {
//     fn can_cast(kind: &str) -> bool
//     where
//         Self: Sized;
//
//     fn cast(node: tree_sitter::Node) -> Option<Self>
//     where
//         Self: Sized,
//     {
//
//     }
//
//     fn node(&self) -> &tree_sitter::Node;
// }

macro_rules! define_ast_node {
    ($struct_ident: ident, [$($field_name: ident),*]) => {
        #[allow(dead_code)]
        pub struct $struct_ident<'a> {
            source: &'a str,
            pub node: Node<'a>,
        }

        impl<'a> fmt::Debug for $struct_ident<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let start_pos = self.node.start_position();
                let end_pos = self.node.end_position();
                let span = format!(
                    "[({}, {}), ({}, {})]",
                    start_pos.row, start_pos.column, end_pos.row, end_pos.column
                );
                f.debug_struct(stringify!($struct_ident))
                    .field("span", &span)
                    $(
                        .field(stringify!($field_name), &self.$field_name())
                    )*
                    .finish()
            }
        }

        impl<'a> $struct_ident<'a> {
            pub fn new(source: &'a str, node: Node<'a>) -> Self {
                Self { source, node }
            }

            pub fn is_position_inside(&self, pos: (usize, usize)) -> bool {
                let point = tree_sitter::Point::new(pos.0, pos.1);
                let node_range = self.node.range();
                point > node_range.start_point && point < node_range.end_point
            }
        }
    };
}

macro_rules! define_ident_literal_from_first_child {
    ($ident_name: ident) => {
        pub fn $ident_name(&self) -> Option<&str> {
            self.node
                .named_child(0)
                .map(|node| node.utf8_text(self.source.as_bytes()).unwrap())
        }
    };
}

macro_rules! define_ident_literal_from_last_child {
    ($ident_name: ident) => {
        pub fn $ident_name(&self) -> Option<&str> {
            self.node
                .named_child(self.node.named_child_count() - 1)
                .map(|node| node.utf8_text(self.source.as_bytes()).unwrap())
        }
    };
}

macro_rules! define_named_ident_literal {
    ($ident_name: ident) => {
        pub fn $ident_name(&self) -> Option<&str> {
            self.node
                .child_by_field_name(stringify!($ident_name))
                .map(|node| node.utf8_text(self.source.as_bytes()).unwrap())
        }
    };
}

macro_rules! define_named_field {
    ($ident_name: ident, $ast_type: ident) => {
        pub fn $ident_name(&self) -> Option<$ast_type> {
            self.node
                .child_by_field_name(stringify!($ident_name))
                .map(|node| $ast_type::new(self.source, node))
        }
    };
}

macro_rules! define_field_from_first_child {
    ($ident_name: ident, $ast_type: ident) => {
        pub fn $ident_name(&self) -> Option<$ast_type> {
            self.node
                .named_child(0)
                .map(|node| $ast_type::new(self.source, node))
        }
    };
}

macro_rules! define_proxy_array_named_field {
    ($field_name: ident, $ast_type: ident) => {
        pub fn $field_name(&self) -> Option<Vec<$ast_type>> {
            self.node
                .child_by_field_name(stringify!($field_name))
                .map(|node| {
                    let mut cursor = self.node.walk();
                    node.named_children(&mut cursor)
                        .map(|node| $ast_type::new(self.source, node))
                        .collect()
                })
        }
    };
}

macro_rules! define_type_field {
    ($field_name: ident) => {
        pub fn $field_name(&self) -> Option<Type> {
            self.node
                .child_by_field_name("type")
                .map(|node| Type::new(self.source, node))
        }
    };
}

macro_rules! define_enum {
    ($enum_name: ident, {$($node_ident: ident => $ast_type: ident),*}) => {
        #[derive(Debug)]
        pub enum $enum_name<'a> {
            $(
                $ast_type($ast_type<'a>),
            )*
        }

        impl<'a> $enum_name<'a> {
            pub fn new(source: &'a str, node: Node<'a>) -> Self {
                match node.kind() {
                    $(
                        stringify!($node_ident) => $enum_name::$ast_type($ast_type::new(source, node)),
                    )*
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct SourceFile {
    source: String,
    pub tree: tree_sitter::Tree,
}

impl fmt::Debug for SourceFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let start_pos = self.tree.root_node().start_position();
        let end_pos = self.tree.root_node().end_position();
        let span = format!(
            "[({}, {}), ({}, {})]",
            start_pos.row, start_pos.column, end_pos.row, end_pos.column
        );
        f.debug_struct("SourceFile")
            .field("span", &span)
            .field("definition", &self.definition())
            .finish()
    }
}
impl SourceFile {
    pub fn new(source: String) -> Self {
        let tree = crate::parser().parse(&source, None).unwrap();
        Self { tree, source }
    }

    pub fn definition(&self) -> Option<Definition> {
        self.tree
            .root_node()
            .named_child(0)
            .filter(|node| node.kind() != "ERROR")
            .map(|node| Definition::new(&self.source, node))
    }
}

#[derive(Debug)]
pub enum Definition<'a> {
    ScriptBlock(ScriptBlock<'a>),
    ModuleBlock(Module<'a>),
    AddressBlock(AddressBlock<'a>),
}

impl<'a> Definition<'a> {
    pub fn new(source: &'a str, node: Node<'a>) -> Self {
        match node.kind() {
            "script_block" => Definition::ScriptBlock(ScriptBlock::new(source, node)),
            "module_definition" => Definition::ModuleBlock(Module::new(source, node)),
            "address_block" => Definition::AddressBlock(AddressBlock::new(source, node)),
            _ => unreachable!(),
        }
    }
}

define_ast_node!(ScriptBlock, [main_function]);

impl<'a> ScriptBlock<'a> {
    pub fn main_function(&self) -> Option<FuncDef> {
        self.node
            .named_children(&mut self.node.walk())
            .find(|&node| node.kind() == "usual_function_definition")
            .map(|node| FuncDef::new(self.source, node))
    }
}

define_ast_node!(AddressBlock, [address, modules]);

impl<'a> AddressBlock<'a> {
    define_named_ident_literal!(address);

    pub fn modules(&self) -> Vec<Module> {
        let mut cursor = self.node.walk();
        self.node
            .named_children(&mut cursor)
            .filter(|&node| node.kind() == "module_definition")
            .map(|node| Module::new(self.source, node))
            .collect()
    }
}

define_ast_node!(Module, [name, body]);

impl<'a> Module<'a> {
    define_named_ident_literal!(name);
    define_proxy_array_named_field!(body, ModuleItem);
}

define_ast_node!(UseDecl, [address, module]);

impl<'a> UseDecl<'a> {
    define_named_ident_literal!(address);
    define_named_ident_literal!(module);
}

define_ast_node!(FieldDef, [field, typ]);

impl<'a> FieldDef<'a> {
    define_named_ident_literal!(field);
    define_type_field!(typ);
}

define_ast_node!(TypeParam, [name]);

impl<'a> TypeParam<'a> {
    define_ident_literal_from_first_child!(name);
}

define_ast_node!(StructDef, [name, type_parameters, fields]);

impl<'a> StructDef<'a> {
    define_named_ident_literal!(name);
    define_proxy_array_named_field!(type_parameters, TypeParam);
    define_proxy_array_named_field!(fields, FieldDef);
}

define_ast_node!(NativeStructDef, [name]);

impl<'a> NativeStructDef<'a> {
    define_named_ident_literal!(name);
}

#[derive(Debug)]
pub enum ModuleItem<'a> {
    Use(UseDecl<'a>),
    FuncDef(FuncDef<'a>),
    NativeFuncDef(NativeFuncDef<'a>),
    Struct(StructDef<'a>),
    NativeStruct(NativeStructDef<'a>),
}

impl<'a> ModuleItem<'a> {
    pub fn new(source: &'a str, node: Node<'a>) -> Self {
        match node.kind() {
            "use_decl" => ModuleItem::Use(UseDecl::new(source, node)),
            "usual_function_definition" => ModuleItem::FuncDef(FuncDef::new(source, node)),
            "native_function_definition" => {
                ModuleItem::NativeFuncDef(NativeFuncDef::new(source, node))
            }
            "struct_definition" => ModuleItem::Struct(StructDef::new(source, node)),
            "native_struct_definition" => {
                ModuleItem::NativeStruct(NativeStructDef::new(source, node))
            }
            _ => unreachable!(),
        }
    }
}

define_ast_node!(
    FuncDef,
    [name, type_parameters, params, return_type, acquires, body]
);

impl<'a> FuncDef<'a> {
    define_named_ident_literal!(name);
    define_proxy_array_named_field!(type_parameters, TypeParam);
    define_proxy_array_named_field!(params, FuncParam);
    define_named_field!(return_type, Type);
    define_proxy_array_named_field!(acquires, ModuleAccess);
    define_named_field!(body, Block);
}

define_ast_node!(NativeFuncDef, [name, type_parameters, params]);

impl<'a> NativeFuncDef<'a> {
    define_named_ident_literal!(name);
    define_proxy_array_named_field!(type_parameters, TypeParam);
    define_proxy_array_named_field!(params, FuncParam);
    define_proxy_array_named_field!(acquires, ModuleAccess);
}

define_ast_node!(FuncParam, [name, typ]);

impl<'a> FuncParam<'a> {
    define_named_ident_literal!(name);
    define_type_field!(typ);
}

define_ast_node!(Block, [items]);

impl<'a> Block<'a> {
    pub fn items(&self) -> Vec<BlockItem> {
        let mut cursor = self.node.walk();
        self.node
            .named_children(&mut cursor)
            .map(|node| BlockItem::new(self.source, node))
            .collect()
    }
}

#[derive(Debug)]
pub enum BlockItem<'a> {
    LetStatement(LetStatement<'a>),
    Expr(Expr<'a>),
}

impl<'a> BlockItem<'a> {
    pub fn new(source: &'a str, node: Node<'a>) -> Self {
        match node.kind() {
            "let_statement" => BlockItem::LetStatement(LetStatement::new(source, node)),
            _ => BlockItem::Expr(Expr::new(source, node)),
        }
    }
}

define_ast_node!(LetStatement, [binds, typ, exp]);

impl<'a> LetStatement<'a> {
    define_proxy_array_named_field!(binds, Bind);
    define_type_field!(typ);
    define_named_field!(exp, Expr);
}

// Binds
// **********************************************************************************
define_enum!(Bind, { bind_var => BindVar, bind_unpack => BindUnpack });

define_ast_node!(BindVar, [name]);

impl<'a> BindVar<'a> {
    define_ident_literal_from_first_child!(name);
}

define_ast_node!(BindUnpack, [module_access, bind_fields]);

impl<'a> BindUnpack<'a> {
    define_field_from_first_child!(module_access, ModuleAccess);
    define_proxy_array_named_field!(bind_fields, BindField);
}

define_ast_node!(BindField, [field, bind]);

impl<'a> BindField<'a> {
    define_named_ident_literal!(field);
    define_named_field!(bind, BindVar);
}

// Types
// **********************************************************************************
define_enum!(Type, { apply_type => ApplyType,
                     ref_type => RefType,
                     tuple_type => TupleType,
                     function_type => FunctionType });

define_ast_node!(ApplyType, [module_access, type_arguments]);

impl<'a> ApplyType<'a> {
    define_field_from_first_child!(module_access, ModuleAccess);
    define_proxy_array_named_field!(type_arguments, Type);
}

define_ast_node!(RefType, [typ]);

impl<'a> RefType<'a> {
    define_field_from_first_child!(typ, Type);
}

define_ast_node!(TupleType, [items]);

impl<'a> TupleType<'a> {
    pub fn items(&self) -> Vec<Type> {
        let mut cursor = self.node.walk();
        self.node
            .named_children(&mut cursor)
            .map(|node| Type::new(self.source, node))
            .collect()
    }
}

define_ast_node!(FunctionType, [param_types, return_type]);

impl<'a> FunctionType<'a> {
    define_proxy_array_named_field!(param_types, Type);
    define_named_field!(return_type, Type);
}

define_ast_node!(ModuleAccess, [address, module, name]);

impl<'a> ModuleAccess<'a> {
    define_named_ident_literal!(address);
    define_named_ident_literal!(module);
    define_ident_literal_from_last_child!(name);
}

// Expressions
// **********************************************************************************
#[derive(Debug)]
pub enum Expr<'a> {
    Lambda(LambdaExpr<'a>),
    Loop(LoopExpr<'a>),
    While(WhileExpr<'a>),
    If(IfExpr<'a>),
    Return(ReturnExpr<'a>),
    Abort(AbortExpr<'a>),
    Assign(AssignExpr<'a>),
    Binary(BinaryExpr<'a>),
    Unary(UnaryExpr<'a>),
}

impl<'a> Expr<'a> {
    pub fn new(source: &'a str, node: Node<'a>) -> Self {
        match node.kind() {
            "lambda_expression" => Expr::Lambda(LambdaExpr::new(source, node)),
            "loop_expression" => Expr::Loop(LoopExpr::new(source, node)),
            "if_expression" => Expr::If(IfExpr::new(source, node)),
            "while_expression" => Expr::While(WhileExpr::new(source, node)),
            "return_expression" => Expr::Return(ReturnExpr::new(source, node)),
            "abort_expression" => Expr::Abort(AbortExpr::new(source, node)),
            "assign_expression" => Expr::Assign(AssignExpr::new(source, node)),
            "binary_expression" => Expr::Binary(BinaryExpr::new(source, node)),
            _ => Expr::Unary(UnaryExpr::new(source, node)),
        }
    }
}

define_ast_node!(LambdaExpr, [bindings, exp]);

impl<'a> LambdaExpr<'a> {
    define_proxy_array_named_field!(bindings, Bind);
    define_named_field!(exp, Expr);
}

define_ast_node!(IfExpr, []);

define_ast_node!(WhileExpr, []);

define_ast_node!(ReturnExpr, []);

define_ast_node!(AbortExpr, []);

define_ast_node!(AssignExpr, []);

define_ast_node!(BinaryExpr, [lhs, operator, rhs]);

impl<'a> BinaryExpr<'a> {
    define_named_field!(lhs, BinaryOperand);
    define_named_ident_literal!(operator);
    define_named_field!(rhs, BinaryOperand);
}

#[derive(Debug)]
pub enum BinaryOperand<'a> {
    BinaryExpr(BinaryExpr<'a>),
    UnaryExpr(UnaryExpr<'a>),
}
impl<'a> BinaryOperand<'a> {
    pub fn new(source: &'a str, node: Node<'a>) -> Self {
        match node.kind() {
            "binary_expression" => BinaryOperand::BinaryExpr(BinaryExpr::new(source, node)),
            _ => BinaryOperand::UnaryExpr(UnaryExpr::new(source, node)),
        }
    }
}

define_ast_node!(LoopExpr, [body]);

impl<'a> LoopExpr<'a> {
    define_named_field!(body, Block);
}

// Unary Expression
// **********************************************************************************
#[derive(Debug)]
pub enum UnaryExpr<'a> {
    Not(NotExpr<'a>),
    Borrow(BorrowExpr<'a>),
    Deref(DerefExpr<'a>),
    MoveOrCopy(MoveOrCopyExpr<'a>),
    Term(Term<'a>),
}

impl<'a> UnaryExpr<'a> {
    pub fn new(source: &'a str, node: Node<'a>) -> Self {
        match node.kind() {
            "unary_expression" => UnaryExpr::Not(NotExpr::new(source, node)),
            "borrow_expression" => UnaryExpr::Borrow(BorrowExpr::new(source, node)),
            "dereference_expression" => UnaryExpr::Deref(DerefExpr::new(source, node)),
            "move_or_copy_expression" => UnaryExpr::MoveOrCopy(MoveOrCopyExpr::new(source, node)),
            _ => UnaryExpr::Term(Term::new(source, node)),
        }
    }
}

define_ast_node!(NotExpr, [exp]);

impl<'a> NotExpr<'a> {
    define_named_field!(exp, UnaryExpr);
}

define_ast_node!(BorrowExpr, [exp]);

impl<'a> BorrowExpr<'a> {
    define_named_field!(exp, UnaryExpr);
}

define_ast_node!(DerefExpr, [exp]);

impl<'a> DerefExpr<'a> {
    define_named_field!(exp, UnaryExpr);
}

define_ast_node!(MoveOrCopyExpr, [exp]);

impl<'a> MoveOrCopyExpr<'a> {
    define_named_ident_literal!(exp);
}

// Terminals
// **********************************************************************************
#[derive(Debug)]
pub enum Term<'a> {
    Break(BreakExpr<'a>),
    Continue(ContinueExpr<'a>),
    Name(NameExpr<'a>),
    Pack(PackExpr<'a>),
    Call(CallExpr<'a>),
    Literal(Literal<'a>),
    Unit(UnitExpr<'a>),
    ExprList(ExprList<'a>),
    Block(Block<'a>),
}

impl<'a> Term<'a> {
    pub fn new(source: &'a str, node: Node<'a>) -> Self {
        match node.kind() {
            "break_expression" => Term::Break(BreakExpr::new(source, node)),
            "continue_expression" => Term::Continue(ContinueExpr::new(source, node)),
            "name_expression" => Term::Name(NameExpr::new(source, node)),
            "pack_expression" => Term::Pack(PackExpr::new(source, node)),
            "call_expression" => Term::Call(CallExpr::new(source, node)),
            "unit_expression" => Term::Unit(UnitExpr::new(source, node)),
            "expression_list" => Term::ExprList(ExprList::new(source, node)),
            "block" => Term::Block(Block::new(source, node)),
            kind if kind.ends_with("literal") => Term::Literal(Literal::new(source, node)),
            _ => unreachable!("{}", node.kind()),
        }
    }
}

define_ast_node!(BreakExpr, []);
define_ast_node!(ContinueExpr, []);
define_ast_node!(ExprList, [items]);
define_ast_node!(UnitExpr, []);

impl<'a> ExprList<'a> {
    pub fn items(&self) -> Vec<Expr> {
        let mut cursor = self.node.walk();
        self.node
            .named_children(&mut cursor)
            .map(|node| Expr::new(self.source, node))
            .collect()
    }
}

define_ast_node!(NameExpr, [fully_qual_name, type_arguments]);

impl<'a> NameExpr<'a> {
    define_field_from_first_child!(fully_qual_name, ModuleAccess);
    define_proxy_array_named_field!(type_arguments, Type);
}

define_ast_node!(CallExpr, [name, args]);

impl<'a> CallExpr<'a> {
    define_field_from_first_child!(name, NameExpr);
    define_proxy_array_named_field!(args, Expr);
}

define_ast_node!(PackExpr, [name, body]);

impl<'a> PackExpr<'a> {
    define_field_from_first_child!(name, NameExpr);
    define_proxy_array_named_field!(body, FieldAssignment);
}

define_ast_node!(FieldAssignment, [field, exp]);

impl<'a> FieldAssignment<'a> {
    define_named_ident_literal!(field);
    define_named_field!(exp, Expr);
}

// Literals
// **********************************************************************************
#[derive(Debug)]
pub enum Literal<'a> {
    Address(&'a str),
    Num(&'a str),
    Bool(&'a str),
    ByteString(&'a str),
}

impl<'a> Literal<'a> {
    pub fn new(source: &'a str, node: Node<'a>) -> Self {
        let val = node.utf8_text(source.as_bytes()).unwrap();
        match node.kind() {
            "address_literal" => Literal::Address(val),
            "num_literal" => Literal::Num(val),
            "bool_literal" => Literal::Bool(val),
            "bytestring_literal" => Literal::ByteString(val),
            _ => unreachable!(),
        }
    }
}
