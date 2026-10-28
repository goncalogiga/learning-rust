// ============================================================================
//  Mission 9: vector search, generic over the embedder (Chapter 10.1 and 10.2)
// ============================================================================

use crate::corpus::Chunk;
use crate::embed::{Embedder, cosine};
use crate::retriever::{Hit, Retriever, top_k};

// Two parameters:
// - `'a`: the chunks borrow from a Corpus, so the index can't outlive that Corpus.
// - `E`: the embedder. `VectorIndex<WordEmbedder>` and `VectorIndex<CharNgramEmbedder>` are two
//   different types, and the compiler generates code for each (monomorphization, Chapter 10.1).
pub struct VectorIndex<'a, E> {
    embedder: E,
    chunks: Vec<Chunk<'a>>,
    vectors: Vec<Vec<f32>>, // vectors[i] is the embedding of chunks[i]
}

impl<'a, E: Embedder> VectorIndex<'a, E> {
    // MISSION 9a: embed every chunk (use `chunk.searchable_text()`) and store the results.
    // `chunks` is a slice; `chunks.to_vec()` copies it into a Vec (cheap, Chunk is Copy).
    pub fn build(embedder: E, chunks: &[Chunk<'a>]) -> VectorIndex<'a, E> {
        todo!("Mission 9")
    }
}

// Read this line out loud: "for any lifetime 'a and any type E that implements Embedder,
// VectorIndex<'a, E> implements Retriever".
impl<'a, E: Embedder> Retriever for VectorIndex<'a, E> {
    fn name(&self) -> String {
        String::from("vector")
    }

    // MISSION 9b:
    // 1. Embed the query with `self.embedder`.
    // 2. Score every chunk with `cosine(&query_vector, &self.vectors[i])`.
    //    Keep only chunks with a score above 0.0.
    // 3. Use `top_k` to keep the best `k`, then turn each `(chunk, score)` into a `Hit`.
    fn search(&self, query: &str, k: usize) -> Vec<Hit<'_>> {
        todo!("Mission 9")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::corpus::Corpus;
    use crate::embed::{CharNgramEmbedder, WordEmbedder};
    use crate::testing::doc;

    fn pets() -> Corpus {
        Corpus {
            docs: vec![
                doc(
                    "cats",
                    "Cats purr and nap in the sun.\n\nKittens chase string.",
                ),
                doc("dogs", "Dogs bark and fetch sticks."),
                doc("fish", "Fish swim in tanks and ponds."),
            ],
        }
    }

    #[test]
    fn mission_09_vector_search() {
        let corpus = pets();
        let chunks = corpus.chunks();
        let index = VectorIndex::build(WordEmbedder { dims: 4096 }, &chunks);

        let hits = index.search("which animal will fetch a stick and bark", 2);
        assert_eq!(hits[0].chunk.doc.id, "dogs");
        assert!(hits.len() <= 2);
        for pair in hits.windows(2) {
            assert!(
                pair[0].score >= pair[1].score,
                "hits must be sorted, best first"
            );
        }
        assert!(
            index.search("zzz", 3).is_empty(),
            "no overlap at all: no hits"
        );
    }

    #[test]
    fn mission_09_works_with_any_embedder() {
        // Same code, different embedder: the char n-gram version is fine with "kitten".
        let corpus = pets();
        let chunks = corpus.chunks();
        let index = VectorIndex::build(CharNgramEmbedder::default(), &chunks);
        let best = index.best("kitten").unwrap();
        assert_eq!(best.chunk.text, "Kittens chase string.");
    }
}
