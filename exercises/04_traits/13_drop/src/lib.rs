// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

#[cfg(test)]
mod tests {

    struct DropBomb(bool);

    impl DropBomb {
        fn new() -> DropBomb {
            DropBomb(true)
        }
        fn defuse(&mut self) {
            self.0 = false
        }
    }

    impl Drop for DropBomb {
        fn drop(&mut self) {
            if self.0 { panic!("BOOM!") }
        }
    }

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
