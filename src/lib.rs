mod moindex;
pub use moindex::*;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn indices() {
        use moindex::*;
        let p = gen("p");
        let a = vir("a");
        let i = occ("i");

        println!("{p}{a}{i}");
    }
}
