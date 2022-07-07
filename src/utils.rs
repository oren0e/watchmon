use anyhow::Result as AnyhowResult;
use std::{fs, path::Path};

pub fn look_for_term_in_file(term: &str, filepath: &dyn AsRef<Path>) -> AnyhowResult<bool> {
    let data = fs::read_to_string(filepath)?;
    if data.contains(term) {
        return Ok(true);
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_for_item_in_file() {
        assert!(look_for_term_in_file("car", &"data/example_file").unwrap());
        assert!(!look_for_term_in_file("obligation", &"data/example_file").unwrap());
    }
}
