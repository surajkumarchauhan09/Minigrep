//! # MiniGrep
//! 
//! `my_crate` is a collection of utilities to filtring lines based on that are containing certain words

/// Filter lines containing certain word using ``search()`` method .
/// # Example
/// 
/// ```
/// use minigrep::search;
/// let query = "duct";
///        let contents = "\
/// Rust:
/// safe, fast, productive.
/// pick three.
/// Duct tape.";
///
/// assert_eq!(vec!["safe, fast, productive."], search(query, contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

/// Filter lines containing certain word using ``search_case_insensitive()`` method .
/// # Example
/// ```
/// use minigrep::search_case_insensitive;
/// let query = "rUst";
///         let contents = "\
/// Rust:
/// safe, fast, productive.
/// pick three.
/// Trust me.";
/// 
/// assert_eq!(
///     vec!["Rust:", "Trust me."],
///     search_case_insensitive(query, contents)
/// );
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
