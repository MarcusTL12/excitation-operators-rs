use once_cell::sync::Lazy;

use super::*;

pub const I: Lazy<MOIndex> = Lazy::new(|| occ("i"));
pub const J: Lazy<MOIndex> = Lazy::new(|| occ("j"));
pub const K: Lazy<MOIndex> = Lazy::new(|| occ("k"));
pub const L: Lazy<MOIndex> = Lazy::new(|| occ("l"));

pub const A: Lazy<MOIndex> = Lazy::new(|| vir("a"));
pub const B: Lazy<MOIndex> = Lazy::new(|| vir("b"));
pub const C: Lazy<MOIndex> = Lazy::new(|| vir("c"));
pub const D: Lazy<MOIndex> = Lazy::new(|| vir("d"));

pub const P: Lazy<MOIndex> = Lazy::new(|| occ("p"));
pub const Q: Lazy<MOIndex> = Lazy::new(|| occ("q"));
pub const R: Lazy<MOIndex> = Lazy::new(|| occ("r"));
pub const S: Lazy<MOIndex> = Lazy::new(|| occ("s"));
