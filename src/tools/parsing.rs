use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// A parsed grammar: mapping single-character keys to token names
/// and a list of moves (each move has a name and a sequence of chars).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveDef {
    pub name: String,
    pub sequence: Vec<char>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grammar {
    /// key -> token name (e.g. 'i' -> "[BK]")
    pub mappings: BTreeMap<char, String>,
    /// moves found in the grammar (may be empty)
    pub moves: Vec<MoveDef>,
}

impl Grammar {
    pub fn new() -> Self {
        Self {
            mappings: BTreeMap::new(),
            moves: Vec::new(),
        }
    }

    /// Display key mappings in the format shown in README examples
    pub fn display_key_mappings(&self) {
        println!("Key mappings:");
        for (key, token_name) in &self.mappings {
            println!("{} -> {}", key, token_name);
        }
        println!("----------------------");
    }

    /// Get the keyboard character for a given token name
    pub fn get_key_for_token(&self, token_name: &str) -> Option<char> {
        self.mappings.iter()
            .find(|(_, name)| name.as_str() == token_name)
            .map(|(key, _)| *key)
    }

    /// Get the token name for a keyboard character
    pub fn get_token_for_key(&self, key: char) -> Option<&str> {
        self.mappings.get(&key).map(|s| s.as_str())
    }
}

/// Parse a `.gmr` grammar file. Currently supports two simple line formats:
///
/// 1) key, Name
///    - maps a single-character `key` to a token name
/// 2) Name: k k k
///    - (optional) move definitions where tokens are single-character keys
///
/// Lines starting with `#` or empty lines are ignored.
pub fn parse_grammar_file<P: AsRef<Path>>(path: P) -> Result<Grammar, String> {
    let file = File::open(&path).map_err(|e| format!("open {}: {}", path.as_ref().display(), e))?;
    let reader = BufReader::new(file);
    let mut grammar = Grammar::new();

    for (lineno, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| format!("read {}: {}", path.as_ref().display(), e))?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // mapping: single_char, Name
        if let Some(pos) = line.find(',') {
            let (left, right) = line.split_at(pos);
            let left = left.trim();
            let right = right[1..].trim(); // skip comma
            if left.len() != 1 {
                return Err(format!("{}:{}: left side must be a single char key", path.as_ref().display(), lineno + 1));
            }
            let key = left.chars().next().unwrap();
            grammar.mappings.insert(key, right.to_string());
            continue;
        }

        // move: Name: keys...
        if let Some(pos) = line.find(':') {
            let (name, seq) = line.split_at(pos);
            let name = name.trim();
            let seq = seq[1..].trim();
            // tokens separated by whitespace
            let mut sequence = Vec::new();
            for tok in seq.split_whitespace() {
                if tok.len() != 1 {
                    return Err(format!("{}:{}: move token must be a single character key: '{}'", path.as_ref().display(), lineno + 1, tok));
                }
                sequence.push(tok.chars().next().unwrap());
            }
            grammar.moves.push(MoveDef { name: name.to_string(), sequence });
            continue;
        }

        // Unknown line format — be lenient and try `key name` (two parts)
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 && parts[0].len() == 1 {
            grammar.mappings.insert(parts[0].chars().next().unwrap(), parts[1].to_string());
            continue;
        }

        return Err(format!("{}:{}: unrecognized line: '{}'", path.as_ref().display(), lineno + 1, line));
    }

    Ok(grammar)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::current_dir;

    #[test]
    fn parse_mk9_mappings() {
        // relative to project root
        let mut path = current_dir().unwrap();
        path.push("grammars/mk9.gmr");
        let g = parse_grammar_file(path).expect("parse mk9");
        // file contains at least these mappings
        assert_eq!(g.mappings.get(&'w').map(|s| s.as_str()), Some("Up"));
        assert_eq!(g.mappings.get(&'i').map(|s| s.as_str()), Some("[BK]"));
        // no moves in this simple grammar file
        assert!(g.moves.is_empty());
    }
}