//! Core logic for template-rust-cli. Keep I/O and argument parsing in the CLI crate so this crate stays
//! easy to test and to reuse as a library.

/// Errors the core can return.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("name must not be empty")]
    EmptyName,
}

/// Build the greeting for `name`.
pub fn greet(name: &str) -> Result<String, Error> {
    let name = name.trim();
    if name.is_empty() {
        return Err(Error::EmptyName);
    }
    Ok(format!("Hello, {name}!"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets() {
        assert_eq!(greet(" Allie ").as_deref(), Ok("Hello, Allie!"));
    }

    #[test]
    fn rejects_empty() {
        assert_eq!(greet("  "), Err(Error::EmptyName));
    }
}
