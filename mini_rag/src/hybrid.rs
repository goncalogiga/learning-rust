// ============================================================================
//  Mission 11: hybrid search, a struct generic over two retrievers (Chapter 10.1 and 10.2)
// ============================================================================

use std::collections::HashMap;

use crate::corpus::Chunk;
use crate::retriever::{Hit, Retriever, top_k};

// The `k` in Reciprocal Rank Fusion. 60 is the value from the original paper.
pub const RRF_K: f32 = 60.0;

// Combines any two retrievers. `A` and `B` can be different types:
// `Hybrid<Bm25Index, VectorIndex<CharNgramEmbedder>>` is the one `main.rs` uses.
pub struct Hybrid<A, B> {
    first: A,
    second: B,
}

impl<A: Retriever, B: Retriever> Hybrid<A, B> {
    pub fn new(first: A, second: B) -> Hybrid<A, B> {
        Hybrid { first, second }
    }
}

impl<A: Retriever, B: Retriever> Retriever for Hybrid<A, B> {
    // MISSION 11a: "hybrid(<first name>+<second name>)", e.g. "hybrid(bm25+vector)".
    fn name(&self) -> String {
        todo!("Mission 11")
    }

    // MISSION 11b: Reciprocal Rank Fusion.
    //
    // 1. Ask each retriever for `k.max(20)` candidates (more than `k`, so that a chunk ranked
    //    25th by one retriever but 1st by the other still gets a chance).
    // 2. A chunk at rank r (1 for the first hit, 2 for the second, ...) in a list earns
    //    1.0 / (RRF_K + r). Its fused score is the sum over both lists. The original scores
    //    are ignored: only the ranks matter.
    // 3. Return the best `k` with `top_k`, as `Hit`s whose score is the fused score.
    //
    // The same chunk usually appears in both lists, so you need to find it again by `chunk.id`.
    // Keep a `Vec<(Chunk, f32)>` of fused scores, plus a `HashMap<usize, usize>` that maps a
    // chunk id to its position in that Vec. (Why not put the scores straight into a HashMap?
    // A HashMap's iteration order is random, so ties would come out in a different order on
    // every run. The Vec keeps the order in which chunks were first seen.)
    fn search(&self, query: &str, k: usize) -> Vec<Hit<'_>> {
        todo!("Mission 11")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{FixedRetriever, doc};

    #[test]
    fn mission_11_name() {
        let a = FixedRetriever {
            name: String::from("bm25"),
            chunks: Vec::new(),
        };
        let b = FixedRetriever {
            name: String::from("vector"),
            chunks: Vec::new(),
        };
        assert_eq!(Hybrid::new(a, b).name(), "hybrid(bm25+vector)");
    }

    #[test]
    fn mission_11_reciprocal_rank_fusion() {
        let d = doc("d", "");
        let c = |id: usize| Chunk {
            id,
            doc: &d,
            text: "",
        };
        // first:  0, 1, 2
        // second: 2, 3, 1
        let a = FixedRetriever {
            name: String::from("a"),
            chunks: vec![c(0), c(1), c(2)],
        };
        let b = FixedRetriever {
            name: String::from("b"),
            chunks: vec![c(2), c(3), c(1)],
        };
        let hybrid = Hybrid::new(a, b);

        let hits = hybrid.search("anything", 10);
        let mut ids = Vec::new();
        for hit in &hits {
            ids.push(hit.chunk.id);
        }
        // chunk 2: 1/63 + 1/61, chunk 1: 1/62 + 1/63, chunk 0: 1/61, chunk 3: 1/62
        assert_eq!(
            ids,
            vec![2, 1, 0, 3],
            "each chunk once, ranked by fused score"
        );
        assert!((hits[0].score - (1.0 / 63.0 + 1.0 / 61.0)).abs() < 1e-6);
        assert!((hits[3].score - 1.0 / 62.0).abs() < 1e-6);

        assert_eq!(hybrid.search("anything", 2).len(), 2, "respect k");
    }

    #[test]
    fn mission_11_ties_are_stable() {
        let d = doc("d", "");
        let c = |id: usize| Chunk {
            id,
            doc: &d,
            text: "",
        };
        // Chunks 5 and 9 are both first in one list only: same fused score.
        let a = FixedRetriever {
            name: String::from("a"),
            chunks: vec![c(5)],
        };
        let b = FixedRetriever {
            name: String::from("b"),
            chunks: vec![c(9)],
        };
        let hybrid = Hybrid::new(a, b);
        for _ in 0..20 {
            let hits = hybrid.search("q", 2);
            assert_eq!(
                hits[0].chunk.id, 5,
                "on a tie, the first retriever's chunk comes first"
            );
            assert_eq!(hits[1].chunk.id, 9);
        }
    }
}
