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
        let mut term_counts = Vec::with_capacity(chunks.len());
        let mut lengths = Vec::with_capacity(chunks.len());
        let mut doc_freq = HashMap::new();

        for chunk in chunks {
            let mut chunk_term_counts = HashMap::new();
            let words = content_words(&chunk.searchable_text());
            let length = words.len();

            for word in words {
                *chunk_term_counts.entry(word).or_insert(0) += 1;
            }

            for term in chunk_term_counts.keys() {
                *doc_freq.entry(term.clone()).or_insert(0) += 1;
            }

            term_counts.push(chunk_term_counts);
            lengths.push(length);
        }

        let avg_len = if lengths.is_empty() {
            0.0
        } else {
            lengths.iter().sum::<usize>() as f32 / lengths.len() as f32
        };

        Bm25Index {
            chunks: Vec::from(chunks),
            term_counts,
            lengths,
            doc_freq,
            avg_len,
        }
    }

    // MISSION 10b: how many chunks contain `term` (0 if none).
    pub fn doc_freq(&self, term: &str) -> usize {
        self.doc_freq.get(term).copied().unwrap_or(0)
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
        let df = self.doc_freq(term) as f32;
        let n = self.chunks.len() as f32;

        f32::ln((n - df + 0.5) / (df + 0.5) + 1.0)
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
        let query_words = content_words(query);
        let mut scores = Vec::new();

        for (i, chunk) in self.chunks.iter().enumerate() {
            let len_i = self.lengths[i] as f32;
            let mut score = 0.0;

            for word in &query_words {
                let Some(&tf) = self.term_counts[i].get(word) else {
                    continue;
                };
                let tf = tf as f32;

                score += self.idf(word) * tf * (K1 + 1.0)
                    / (tf + K1 * (1.0 - B + B * len_i / self.avg_len))
            }

            if score > 0.0 {
                scores.push((*chunk, score));
            }
        }

        top_k(scores, k)
            .into_iter()
            .map(|(chunk, score)| Hit { chunk, score })
            .collect()
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
