use crate::*;
use std::fs::File;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OpeningTree {
    root: Option<Vec<(ChessMove, Vec<(ChessMove, OpeningTree)>)>>
}

impl OpeningTree {
    pub fn new() -> Self {
        OpeningTree {root: None}
    }

    pub fn save(&self, file_name: &str) {
        let mut file = File::create(file_name).unwrap();
        let serialized = serde_json::to_string(&self).unwrap();
        file.write_all(serialized.as_bytes());
    }

    pub fn load(file_name: &str) -> OpeningTree {
        let mut file = File::open(file_name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        serde_json::from_str(&contents).unwrap()
    }

    pub fn add_d4(&mut self) {
        self.root = Some(vec!((ChessMove::new(PAWN(WHITE), "d2", "d4"), vec!())));
    }
}