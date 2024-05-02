use std::fs::File;
use std::io::{Error, Read};

use super::todo;
use todo::Todo;

pub fn parse_database(mut file: File) -> Result<Vec<Todo>, Error> {
    let mut results: Vec<Todo> = vec![];
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents.lines().for_each(|line| {
        let words: Vec<&str> = line.split("|").collect();

        if words.len() != 5 {
            return;
        }

        let name = words[1].to_string();
        let category: Option<String> = if words[2].is_empty() {
            None
        } else {
            Some(words[2].to_string())
        };

        let completed = words[3].parse().unwrap_or_else(|_| false);

        let todo = Todo::new(
            name,
            category,
            completed,
        );

        results.push(todo);
    });

    Ok(
        results
    )
}