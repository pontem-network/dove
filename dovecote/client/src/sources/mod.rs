pub mod highlight;

pub type Error = Vec<(Loc, String)>;
pub type Errors = Vec<Error>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Loc {
    pub start: u32,
    pub end: u32,
}

pub fn make_loc(start: usize, end: usize) -> Loc {
    Loc {
        start: start as u32,
        end: end as u32,
    }
}
