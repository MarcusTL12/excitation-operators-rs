mod moindex;
pub use moindex::*;

mod exop;
pub use exop::*;

mod delta;
pub use delta::*;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_indices() {
        let p = gen("p");
        let a = vir("a");
        let i = occ("i");

        println!("{p}{a}{i}");
    }

    #[test]
    fn test_exop() {
        let o = ExcitationOperator {
            p: vir("a"),
            q: occ("i"),
        };
        println!("{o}");
    }

    #[test]
    fn test_delta() {
        let d = KroeneckerDelta {
            p: vir("a"),
            q: occ("i"),
        };
        println!("{d}");
    }
}
