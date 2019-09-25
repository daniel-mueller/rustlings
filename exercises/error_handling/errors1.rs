pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.len() > 0 {
        Ok(format!("Hi! My name is {}", name))
    } else {
        Err(format!("`name` was empty; it must be nonempty."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}




















// `Err` is one of the variants of `Result`, so what the 2nd test is saying
// is that `generate_nametag_text` should return a `Result` instead of an
// `Option`.

// To make this change, you'll need to:
// - update the return type in the function signature to be a Result<String, String> that
//   could be the variants `Ok(String)` and `Err(String)`
// - change the body of the function to return `Ok(stuff)` where it currently
//   returns `Some(stuff)`
// - change the body of the function to return `Err(error message)` where it
//   currently returns `None`
// - change the first test to expect `Ok(stuff)` where it currently expects
//   `Some(stuff)`.
