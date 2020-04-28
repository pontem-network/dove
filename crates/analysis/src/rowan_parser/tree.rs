use crate::rowan_parser::syntax_kind::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MoveLanguage;

impl rowan::Language for MoveLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
        SyntaxKind::from(raw.0)
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.into())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<MoveLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<MoveLanguage>;
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

#[derive(Debug)]
pub struct File(SyntaxNode);

#[allow(dead_code)]
impl File {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::SOURCE_FILE {
            Some(Self(node))
        } else {
            None
        }
    }
}
