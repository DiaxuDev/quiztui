use std::{
    env::current_dir,
    fs::File,
    io::{self, BufRead, BufReader},
};

use deunicode::deunicode;

#[derive(Debug)]
pub struct Flashcard {
    pub definition: String,
    pub explanation: String,
}

pub fn parse_cards() -> io::Result<Vec<Flashcard>> {
    let path = current_dir().map(|p| p.join("input.csv"))?;
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();

    let mut entries = Vec::new();

    for line in lines {
        let line = line?;
        let (definition, explanation) = line
            .split_once(',')
            .ok_or(io::Error::other("invalid format"))?;

        entries.push(Flashcard {
            definition: definition.to_owned(),
            explanation: explanation.to_owned(),
        });
    }

    Ok(entries)
}

impl Flashcard {
    pub fn check(&self, input: &str) -> bool {
        let word = deunicode(&self.definition);

        if let Some((root, suffix)) = word.split_once('/') {
            if input == root {
                return true;
            }

            input.strip_prefix(&root[..root.len() - suffix.len()]) == Some(suffix)
        } else {
            word == input
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn slash_pattern() {
        let card = Flashcard {
            definition: "bajo/a".into(),
            explanation: "".into(),
        };

        assert!(card.check("bajo"));
        assert!(card.check("baja"));
        assert!(!card.check("foo"));
        assert!(!card.check("bajos"));
    }
}
