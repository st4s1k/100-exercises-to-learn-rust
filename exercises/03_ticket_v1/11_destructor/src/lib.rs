// We need some more machinery to write a proper exercise for destructors.
// We'll pick the concept up again in a later chapter after covering traits and
// interior mutability.
fn outro() -> &'static str {
    // let x = String::from("dummy");
    // drop(x); // value moved here
    // println!("{x}"); // value borrowed here after move, so it will fail
    "I have a basic understanding of destructors!"
    // value dropped here (out of scope) if it wasn't moved or dropped before
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a basic understanding of destructors!");
    }
}
