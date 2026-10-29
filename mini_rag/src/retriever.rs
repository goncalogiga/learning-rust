// ============================================================================
//  Mission 7: the Retriever trait and a generic top-k (Chapter 10.1 and 10.2)
// ============================================================================

use crate::corpus::Chunk;

// One search result: a chunk and how well it matched (higher is better).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hit<'a> {
    pub chunk: Chunk<'a>,
    pub score: f32,
}

// Anything that can answer "which chunks best match this query?".
//
// BM25, vector search and hybrid search all implement this trait, so code like the evaluation
// in Mission 12 can be written once and work with all three.
pub trait Retriever {
    // A short name for reports, e.g. "bm25".
    fn name(&self) -> String;

    // The best `k` chunks for `query`, best first. May return fewer than `k`.
    //
    // `Hit<'_>` means the hits borrow from `self` (elision rule 3: `&self` methods give their
    // lifetime to the output).
    fn search(&self, query: &str, k: usize) -> Vec<Hit<'_>>;

    // MISSION 7b: a *default method*. Every type that implements `Retriever` gets it for free,
    // without writing it. Return the single best hit, or `None` if there are no hits.
    // Call `self.search(...)`: a default method can use the trait's other methods.
    fn best(&self, query: &str) -> Option<Hit<'_>> {
        self.search(query, 1).into_iter().next()
    }
}

// Worked example: a *blanket implementation* (Chapter 10.2, end of "Using Trait Bounds to
// Conditionally Implement Methods").
//
// It reads: "for every type R that is a Retriever, a reference `&R` is a Retriever too".
// Thanks to it, `Hybrid::new(&bm25, &vector)` in `main.rs` can borrow the two indexes
// instead of taking ownership of them.
impl<R: Retriever> Retriever for &R {
    fn name(&self) -> String {
        (**self).name()
    }

    fn search(&self, query: &str, k: usize) -> Vec<Hit<'_>> {
        (**self).search(query, k)
    }
}

// MISSION 7a: keep the `k` items with the highest scores, highest first.
//
//   top_k(vec![("a", 0.1), ("b", 0.9), ("c", 0.5)], 2)  ->  [("b", 0.9), ("c", 0.5)]
//
// - If two items have the same score, keep their original order.
// - If `k` is bigger than the number of items, return them all (sorted).
//
// `T` can be anything: this function never looks at it, so it needs no trait bounds.
// Before you start, read the README's "try this" about `.sort()`.
pub fn top_k<T>(mut scored: Vec<(T, f32)>, k: usize) -> Vec<(T, f32)> {
    scored.sort_by(|a, b| b.1.total_cmp(&a.1)); // Reversed order a <-> b
    scored.truncate(k);
    scored
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{FixedRetriever, doc};

    #[test]
    fn mission_07_top_k() {
        let scored = vec![("a", 0.1), ("b", 0.9), ("c", 0.5), ("d", 0.7)];
        assert_eq!(top_k(scored, 2), vec![("b", 0.9), ("d", 0.7)]);
    }

    #[test]
    fn mission_07_top_k_edge_cases() {
        // k bigger than the input: everything, sorted
        assert_eq!(
            top_k(vec![(1, 0.2), (2, 0.8)], 10),
            vec![(2, 0.8), (1, 0.2)]
        );
        // k = 0 and empty input
        assert!(top_k(vec![(1, 0.2)], 0).is_empty());
        assert!(top_k(Vec::<(u8, f32)>::new(), 3).is_empty());
        // ties keep their original order
        assert_eq!(
            top_k(vec![("x", 0.5), ("y", 0.9), ("z", 0.5)], 3),
            vec![("y", 0.9), ("x", 0.5), ("z", 0.5)]
        );
        // negative scores are fine
        assert_eq!(top_k(vec![("p", -1.0), ("q", -0.5)], 1), vec![("q", -0.5)]);
    }

    #[test]
    fn mission_07_top_k_works_for_any_type() {
        let words = vec![(String::from("long"), 3.0), (String::from("hi"), 7.0)];
        assert_eq!(top_k(words, 1), vec![(String::from("hi"), 7.0)]);
    }

    #[test]
    fn mission_07_default_method_best() {
        let d = doc("d", "First.\n\nSecond.");
        let chunks = vec![
            Chunk {
                id: 0,
                doc: &d,
                text: "First.",
            },
            Chunk {
                id: 1,
                doc: &d,
                text: "Second.",
            },
        ];
        let retriever = FixedRetriever {
            name: String::from("fixed"),
            chunks,
        };
        // `FixedRetriever` never wrote `best`: it comes from the trait's default.
        assert_eq!(retriever.best("anything").unwrap().chunk.text, "First.");

        let empty = FixedRetriever {
            name: String::from("empty"),
            chunks: Vec::new(),
        };
        assert!(empty.best("anything").is_none());
    }
}
