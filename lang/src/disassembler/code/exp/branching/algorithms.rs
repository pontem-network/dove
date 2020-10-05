#[allow(dead_code)]
#[derive(Debug)]
pub enum Algorithm {
    If {
        true_branch: Branch,
    },
    IfElse {
        true_branch: Branch,
        false_branch: Branch,
    },
    Loop {
        body: Branch,
    },
    While {
        body: Branch,
    },
}

impl Algorithm {
    //pub fn select()
}

#[derive(Debug)]
pub struct Branch {
    start_offset: usize,
    end_offset: usize,
}
