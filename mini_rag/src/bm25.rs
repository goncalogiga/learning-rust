// ============================================================================
//  Mission 10: keyword search with BM25 (Chapter 8.3, hash maps)
// ============================================================================

use std::collections::HashMap;

use crate::corpus::Chunk;
use crate::retriever::{Hit, Retriever, top_k};
use crate::text::content_words;

// The two standard BM25 constants. The README explains what they do.
pub const K1: f32 = 1.2;
pub const B: f32 = 0.75;

pub struct Bm25Index<'a> {
    chunks: Vec<Chunk<'a>>,
    // term_counts[i]: how many times each word appears in chunks[i]
    term_counts: Vec<HashMap<String, usize>>,
    // lengths[i]: the number of words in chunks[i]
    lengths: Vec<usize>,
    // doc_freq["attention"]: how many chunks contain "attention" at least once
    doc_freq: HashMap<String, usize>,
    avg_len: f32,
}

impl<'a> Bm25Index<'a> {
    // MISSION 10a: count words. For each chunk, take `content_words(&chunk.searchable_text())` and fill in
    // `term_counts`, `lengths` and `doc_freq`. Then compute `avg_len`, the average of `lengths`
    // (0.0 if there are no chunks).
    //
    // Careful with `doc_freq`: a word that appears 3 times in one chunk only adds 1.
    // Tip: fill the chunk's `term_counts` map first, then loop over its keys.
    pub fn build(chunks: &[Chunk<'a>]) -> Bm25Index<'a> {
        todo!("Mission 10")
    }

    // MISSION 10b: how many chunks contain `term` (0 if none).
    pub fn doc_freq(&self, term: &str) -> usize {
        todo!("Mission 10")
    }

    pub fn avg_len(&self) -> f32 {
        self.avg_len
    }

    // MISSION 10b: inverse document frequency. Rare words get a high weight, common words a
    // low one.
    //
    //   idf = ln( (N - df + 0.5) / (df + 0.5) + 1 )
    //
    // where N is the number of chunks and df is `self.doc_freq(term)`.
    // Convert the counts with `as f32`, and use `f32::ln(x)` (or `x.ln()`).
    pub fn idf(&self, term: &str) -> f32 {
        todo!("Mission 10")
    }
}

impl Retriever for Bm25Index<'_> {
    fn name(&self) -> String {
        String::from("bm25")
    }

    // MISSION 10c: score every chunk against the query, then keep the best `k`.
    //
    // For chunk i, add up, over every word of `content_words(query)`:
    //
    //   tf = how many times the word appears in chunk i (skip the word if tf is 0)
    //   idf(word) * tf * (K1 + 1) / (tf + K1 * (1 - B + B * len_i / avg_len))
    //
    // Keep only chunks with a score above 0.0, and use `top_k` like in Mission 9.
    fn search(&self, query: &str, k: usize) -> Vec<Hit<'_>> {
        todo!("Mission 10")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::corpus::Corpus;
    use crate::testing::doc;

    fn corpus() -> Corpus {
        // `doc` sets the title to the id in uppercase ("X", "Y", "Z"), and titles are indexed too.
        Corpus {
            docs: vec![
                doc("x", "the cat sat on the mat"),
                doc("y", "the dog sat on the log"),
                doc("z", "the cat chased the dog and the cat won"),
            ],
        }
    }

    #[test]
    fn mission_10_build() {
        let corpus = corpus();
        let chunks = corpus.chunks();
        let index = Bm25Index::build(&chunks);
        assert_eq!(index.doc_freq("sat"), 2);
        assert_eq!(
            index.doc_freq("cat"),
            2,
            "'cat' twice in chunk z still counts once"
        );
        assert_eq!(index.doc_freq("won"), 1);
        assert_eq!(index.doc_freq("zebra"), 0);
        assert_eq!(index.doc_freq("the"), 0, "stopwords are not indexed");
        // content words, title included: [x cat sat mat] [y dog sat log] [z cat chased dog cat won]
        assert!((index.avg_len() - 14.0 / 3.0).abs() < 1e-5);
    }

    #[test]
    fn mission_10_idf() {
        let corpus = corpus();
        let chunks = corpus.chunks();
        let index = Bm25Index::build(&chunks);
        assert!(index.idf("won") > index.idf("cat"), "rare words weigh more");
        // N = 3, df = 1: ln((3 - 1 + 0.5) / (1 + 0.5) + 1) = ln(2.6667)
        assert!((index.idf("won") - 0.980829).abs() < 1e-4);
        // N = 3, df = 2: ln(1.5 / 2.5 + 1) = ln(1.6)
        assert!((index.idf("cat") - 0.470004).abs() < 1e-4);
    }

    #[test]
    fn mission_10_search() {
        let corpus = corpus();
        let chunks = corpus.chunks();
        let index = Bm25Index::build(&chunks);

        let hits = index.search("cat", 10);
        assert_eq!(hits.len(), 2, "only chunks that contain the word");
        assert_eq!(hits[0].chunk.doc.id, "z", "two cats beat one cat");
        assert_eq!(hits[1].chunk.doc.id, "x");

        let hits = index.search("dog log", 10);
        assert_eq!(hits[0].chunk.doc.id, "y");
        assert!(
            (hits[0].score - 1.5409).abs() < 1e-3,
            "check your formula, got {}",
            hits[0].score
        );

        assert!(index.search("zebra", 10).is_empty());
        assert!(
            index.search("the", 10).is_empty(),
            "stopwords match nothing"
        );
        assert_eq!(index.search("sat", 1).len(), 1, "respect k");
    }
}
