use std::fmt::Display;

use super::*;

pub struct ExcitationOperator {
    pub p: MOIndex,
    pub q: MOIndex,
}

// pub fn exop(p: MOIndex, q: MOIndex) -> ExcitationOperator {
//     ExcitationOperator { p, q }
// }

impl Display for ExcitationOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "E_{}{}", self.p, self.q)?;
        Ok(())
    }
}
