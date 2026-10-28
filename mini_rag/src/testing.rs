// Helpers for the tests. You don't need to change anything here.

use std::path::PathBuf;

use crate::corpus::{Chunk, Document};
use crate::retriever::{Hit, Retriever};

// `data/`, found from the crate root so tests work from any directory.
pub fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data")
}

pub fn doc(id: &str, body: &str) -> Document {
    Document {
        id: id.to_string(),
        title: id.to_uppercase(),
        body: body.to_string(),
    }
}

// A fake retriever that always returns the same chunks, in the same order, whatever the query.
// Handy to test code that is generic over `Retriever` (Missions 7, 11 and 12).
pub struct FixedRetriever<'a> {
    pub name: String,
    pub chunks: Vec<Chunk<'a>>,
}

impl Retriever for FixedRetriever<'_> {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn search(&self, _query: &str, k: usize) -> Vec<Hit<'_>> {
        let mut hits = Vec::new();
        for (i, chunk) in self.chunks.iter().enumerate() {
            if i == k {
                break;
            }
            hits.push(Hit {
                chunk: *chunk,
                score: 1.0 / (i + 1) as f32,
            });
        }
        hits
    }
}
