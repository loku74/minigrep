use minigrep::{search, search_case_insensitive};

use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /// Builds a `Config` from an iterator of command-line arguments.
    ///
    /// The first item is skipped, as it is expected to be the program name.
    /// The next two items are used as the search query and the file path.
    /// Case-insensitive search is enabled when the `IGNORE_CASE`
    /// environment variable is set, regardless of its value.
    ///
    /// # Errors
    ///
    /// Returns an error if the query or the file path is missing.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let args = ["minigrep", "to", "poem.txt"].map(String::from);
    /// let config = Config::build(args.into_iter()).unwrap();
    ///
    /// assert_eq!(config.query, "to");
    /// assert_eq!(config.file_path, "poem.txt");
    /// ```
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
