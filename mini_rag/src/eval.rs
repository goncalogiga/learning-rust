// ============================================================================
//  Mission 12: measuring retrieval quality (a generic function + `Display`)
// ============================================================================

use std::fmt;

use crate::corpus::LabeledQuery;
use crate::retriever::Retriever;

#[derive(Debug, PartialEq)]
pub struct EvalReport {
    pub name: String,
    pub k: usize,
    // Fraction of queries whose relevant document is somewhere in the top k (0.0 to 1.0).
    pub recall: f32,
    // Mean reciprocal rank: the average of 1/rank of the first relevant hit (0 if not found).
    pub mrr: f32,
    // The queries whose relevant document was not in the top k, in input order.
    pub misses: Vec<String>,
}

// MISSION 12a: run every query through `retriever` and compute an EvalReport.
//
// For each query, call `retriever.search(&q.query, k)` and find the rank (1-based) of the first
// hit whose `chunk.doc.id` equals `q.relevant`.
//   - found at rank r:  counts as a hit for recall, and adds 1/r to the MRR sum
//   - not found:        push the query text to `misses`
// Then divide both sums by the number of queries (if there are no queries, both are 0.0).
//
// This one function works for BM25, vectors, hybrid, and any retriever you write later,
// because it only relies on the `Retriever` trait.
pub fn evaluate<R: Retriever>(retriever: &R, queries: &[LabeledQuery], k: usize) -> EvalReport {
    let mut hits = 0;
    let mut reciprocal_rank_sum = 0.0;
    let mut misses = Vec::new();

    for q in queries {
        let results = retriever.search(&q.query, k);

        if let Some(pos) = results
            .iter()
            .position(|hit| hit.chunk.doc.id == q.relevant)
        {
            hits += 1;
            reciprocal_rank_sum += 1.0 / (pos + 1) as f32; // rank = pos + 1
        } else {
            misses.push(q.query.clone());
        }
    }

    let (recall, mrr) = if queries.is_empty() {
        (0.0, 0.0)
    } else {
        let n = queries.len() as f32;
        (hits as f32 / n, reciprocal_rank_sum / n)
    };

    EvalReport {
        name: retriever.name(),
        k,
        recall,
        mrr,
        misses,
    }
}

// MISSION 12b: one line per report, so several reports line up as a table:
//
//   "bm25                  recall@5 0.886  MRR 0.771"
//
// The name is padded to 21 characters (`{:<21}`), and both numbers have 3 decimals (`{:.3}`).
impl fmt::Display for EvalReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:<21} recall@{} {:.3}  MRR {:.3}",
            self.name, self.k, self.recall, self.mrr
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::corpus::Chunk;
    use crate::testing::{FixedRetriever, doc};

    fn query(text: &str, relevant: &str) -> LabeledQuery {
        LabeledQuery {
            query: text.to_string(),
            relevant: relevant.to_string(),
        }
    }

    #[test]
    fn mission_12_evaluate() {
        let (a, b, c) = (doc("a", ""), doc("b", ""), doc("c", ""));
        // Always returns: a, b, c (in that order)
        let retriever = FixedRetriever {
            name: String::from("fixed"),
            chunks: vec![
                Chunk {
                    id: 0,
                    doc: &a,
                    text: "",
                },
                Chunk {
                    id: 1,
                    doc: &b,
                    text: "",
                },
                Chunk {
                    id: 2,
                    doc: &c,
                    text: "",
                },
            ],
        };
        let queries = vec![
            query("wants a", "a"), // rank 1 -> 1
            query("wants b", "b"), // rank 2 -> 0.5
            query("wants c", "c"), // rank 3, outside the top 2 -> miss
            query("wants z", "z"), // not there at all -> miss
        ];

        let report = evaluate(&retriever, &queries, 2);
        assert_eq!(report.name, "fixed");
        assert_eq!(report.k, 2);
        assert!((report.recall - 0.5).abs() < 1e-6, "2 of 4 found");
        assert!((report.mrr - 0.375).abs() < 1e-6, "(1 + 0.5 + 0 + 0) / 4");
        assert_eq!(report.misses, vec!["wants c", "wants z"]);

        let report = evaluate(&retriever, &queries, 3);
        assert!((report.recall - 0.75).abs() < 1e-6);

        let empty = evaluate(&retriever, &[], 3);
        assert_eq!((empty.recall, empty.mrr), (0.0, 0.0));
    }

    #[test]
    fn mission_12_only_the_first_relevant_hit_counts() {
        let a = doc("a", "");
        let retriever = FixedRetriever {
            name: String::from("fixed"),
            chunks: vec![
                Chunk {
                    id: 0,
                    doc: &a,
                    text: "first chunk of a",
                },
                Chunk {
                    id: 1,
                    doc: &a,
                    text: "second chunk of a",
                },
            ],
        };
        let report = evaluate(&retriever, &[query("q", "a")], 5);
        assert!((report.mrr - 1.0).abs() < 1e-6);
    }

    #[test]
    fn mission_12_display() {
        let report = EvalReport {
            name: String::from("bm25"),
            k: 5,
            recall: 0.88571,
            mrr: 0.7714,
            misses: Vec::new(),
        };
        assert_eq!(
            report.to_string(),
            "bm25                  recall@5 0.886  MRR 0.771"
        );
    }
}
