// 🔎 mini_rag: the retrieval half of a RAG pipeline, for practising Chapters 7 to 10.
//
// The library (this crate) loads markdown documents, cuts them into chunks, indexes them two
// ways (keywords with BM25, and vectors with a toy embedding) and fuses the results. The binary
// in `main.rs` is a small CLI on top of it.
//
// Search for `todo!` to find your missions. The README explains each one.

// Unfinished missions leave unused variables around; this silences the noise.
#![allow(unused)]

// Chapter 7: each `mod` line tells Rust to load `src/<name>.rs` as a module.
// `pub` makes the module visible to users of the library, such as `main.rs`.
pub mod bm25;
pub mod corpus;
pub mod embed;
pub mod error;
pub mod eval;
pub mod hybrid;
pub mod retriever;
pub mod text;
pub mod vector;

// Helpers shared by the tests. `#[cfg(test)]` means it only exists when running `cargo test`.
#[cfg(test)]
mod testing;

// Re-exports, so callers can write `mini_rag::Corpus` instead of `mini_rag::corpus::Corpus`.
pub use bm25::Bm25Index;
pub use corpus::{Chunk, Corpus, Document, LabeledQuery};
pub use embed::{CharNgramEmbedder, Embedder, WordEmbedder};
pub use error::RagError;
pub use eval::{EvalReport, evaluate};
pub use hybrid::Hybrid;
pub use retriever::{Hit, Retriever};
pub use vector::VectorIndex;
