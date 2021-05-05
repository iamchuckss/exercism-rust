#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref DNARNA: HashMap<char, char> = {
        let mut m = HashMap::new();
        m.insert('G', 'C');
        m.insert('C', 'G');
        m.insert('T', 'A');
        m.insert('A', 'U');
        m
    };

    static ref RNADNA: HashMap<char, char> = {
        let mut m = HashMap::new();
        for (key, val) in DNARNA.iter() {
            m.insert(*val, *key);
        }
        m
    };
}

#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

impl DNA {
    pub fn new(s: &str) -> Result<DNA, &str> {
        if s.chars().all(|c| DNARNA.contains_key(&c)) {
            Ok(DNA(s.to_string()))
        } else {
            Err("invalid dna string")
        }
    }

    pub fn to_rna(&self) -> RNA {
        RNA(self.0.chars().map(|c| DNARNA.get(&c).unwrap()).collect())
    }
}

impl RNA {
    pub fn new(s: &str) -> Result<RNA, &str> {
        if s.chars().all(|c| RNADNA.contains_key(&c)) {
            Ok(RNA(s.to_string()))
        } else {
            Err("invalid rna string")
        }
    }

    pub fn to_dna(&self) -> DNA {
        DNA(self.0.chars().map(|c| DNARNA.get(&c).unwrap()).collect())
    }
}