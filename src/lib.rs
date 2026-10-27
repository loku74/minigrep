//! # minigrep
//!
//! A small version of `grep`: it finds the lines of a text that contain
//! a query.
//!
//! Use [`search`] for a case-sensitive search, or
//! [`search_case_insensitive`] to ignore case.

/// Returns every line of `contents` that contains `query`.
///
/// The search is case-sensitive. Lines are returned in the order they
/// appear, as slices borrowed from `contents`.
///
/// # Examples
///
/// ```
/// use minigrep::search;
///
/// let contents = "Rust:\nsafe, fast, productive.\nDuct tape.";
///
/// assert_eq!(search("duct", contents), vec!["safe, fast, productive."]);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Returns every line of `contents` that contains `query`, ignoring case.
///
/// Both `query` and each line are lowercased before comparing. Lines are
/// returned in the order they appear, as slices borrowed from `contents`,
/// with their original casing.
///
/// # Examples
///
/// ```
/// use minigrep::search_case_insensitive;
///
/// let contents = "Rust:\nsafe, fast, productive.\nTrust me.";
///
/// assert_eq!(
///     search_case_insensitive("rUsT", contents),
///     vec!["Rust:", "Trust me."]
/// );
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
