use std::fmt::Display;

pub use Occupation::*;

#[derive(Debug, Clone, Copy)]
pub enum Occupation {
    Gen = 1,
    Vir,
    Occ,
}

pub struct MOIndex {
    o: Occupation,
    n: String,
}

impl MOIndex {
    pub fn new(n: &str, o: Occupation) -> Self {
        MOIndex { o, n: n.to_owned() }
    }

    pub fn isgen(&self) -> bool {
        matches!(self.o, Gen)
    }

    pub fn isvir(&self) -> bool {
        matches!(self.o, Vir)
    }

    pub fn isocc(&self) -> bool {
        matches!(self.o, Occ)
    }
}

pub fn gen(n: &str) -> MOIndex {
    MOIndex::new(n, Gen)
}

pub fn vir(n: &str) -> MOIndex {
    MOIndex::new(n, Vir)
}

pub fn occ(n: &str) -> MOIndex {
    MOIndex::new(n, Occ)
}

impl Display for MOIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let col = match self.o {
            Gen => "",
            Vir => "\x1b[91m",
            Occ => "\x1b[92m",
        };
        write!(f, "{}{}\x1b[39m", col, self.n)?;
        Ok(())
    }
}
