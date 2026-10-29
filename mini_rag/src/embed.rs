// ============================================================================
//  Mission 8: embeddings (Chapter 10.2, implementing a trait)
// ============================================================================
//
// A real RAG system turns text into vectors with a neural network, so that texts with similar
// meaning get similar vectors. We don't have a neural network, so we fake it with hashing.
// It's a stand-in: the point is the `Embedder` trait. Later you can implement the same trait
// with a real model, and nothing else in the code has to change.

use crate::text::content_words;
use std::collections::HashMap;

pub trait Embedder {
    // Turn text into a vector. Every call must return a vector of the same length.
    fn embed(&self, text: &str) -> Vec<f32>;
}

// FNV-1a: a tiny, fast hash function. Same input, same output, on every machine and every run.
// We use it to pick a position in the vector for each word (the "hashing trick").
pub fn fnv1a(s: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in s.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

// Scale `v` in place so its length is 1 (a "unit vector"). A zero vector is left alone.
pub fn normalize(v: &mut [f32]) {
    let mut sum = 0.0;
    for x in v.iter() {
        sum += x * x;
    }
    let length = f32::sqrt(sum);
    if length > 0.0 {
        for x in v.iter_mut() {
            *x /= length;
        }
    }
}

// ============================================================================
//  Worked example: read this before writing CharNgramEmbedder
// ============================================================================

// Bag of words: each word adds 1.0 at position `hash(word) % dims`.
// "cat sat" and "sat cat" get the same vector; "cat" and "cats" share nothing.
pub struct WordEmbedder {
    pub dims: usize,
}

impl Embedder for WordEmbedder {
    fn embed(&self, text: &str) -> Vec<f32> {
        let mut v = vec![0.0; self.dims];
        for word in content_words(text) {
            let index = (fnv1a(&word) % self.dims as u64) as usize;
            v[index] += 1.0;
        }
        normalize(&mut v);
        v
    }
}

// ============================================================================
//  Mission 8
// ============================================================================

// Character n-grams: instead of whole words, use every run of `n` characters.
// With n = 3, the word "token" becomes "#to", "tok", "oke", "ken", "en#" (the `#` marks the
// start and end of a word). "tokenization" and "tokenisation" now share most of their
// n-grams, so they get similar vectors even though they are different words.
pub struct CharNgramEmbedder {
    pub dims: usize,
    pub n: usize,
}

// `Default` is a standard trait for "a sensible starting value". `main.rs` calls
// `CharNgramEmbedder::default()`.
impl Default for CharNgramEmbedder {
    fn default() -> CharNgramEmbedder {
        CharNgramEmbedder { dims: 4096, n: 3 }
    }
}

impl CharNgramEmbedder {
    fn ngram_word(&self, word: &str) -> Vec<f32> {
        todo!()
    }
}

// MISSION 8a: implement `Embedder` for `CharNgramEmbedder`. Copy the shape of WordEmbedder.
//
// For each word from `content_words(text)`:
//   1. Wrap it in markers: format!("#{word}#").
//   2. Collect its characters into a Vec<char> (use `chars()` and a loop, or `.collect()`).
//      Index by character, not by byte: "été" is 3 characters but 5 bytes.
//   3. For every start position i where i + n <= length, build the n-gram as a String and
//      add 1.0 at position `fnv1a(&ngram) % dims`.
//   (If the marked word is shorter than n, it has no n-grams; that's fine.)
// Finally, normalize the vector.
impl Embedder for CharNgramEmbedder {
    fn embed(&self, text: &str) -> Vec<f32> {
        let mut embedding: Vec<f32> = vec![0.0; self.dims];

        for word in content_words(text) {
            let chars: Vec<char> = format!("#{word}#").chars().collect();

            for window in chars.windows(self.n) {
                let ngram: String = window.iter().collect();
                let slot = (fnv1a(&ngram) % self.dims as u64) as usize;
                embedding[slot] += 1.0;
            }
        }

        let norm_factor = embedding.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
        let norm_factor = if norm_factor == 0.0 { 1.0 } else { norm_factor };

        embedding.into_iter().map(|x| x / norm_factor).collect()
    }
}

fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

// MISSION 8b: cosine similarity between two vectors of the same length.
//
//   cosine(a, b) = dot(a, b) / (length(a) * length(b))
//
// where dot(a, b) is the sum of a[i] * b[i], and length(a) is the square root of dot(a, a).
// It is 1.0 for vectors pointing the same way, and 0.0 for vectors that share nothing.
// If either vector has length 0, return 0.0 (dividing by zero would give NaN).
pub fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let length = |x: &[f32]| dot(x, x).sqrt();

    if length(a) * length(b) == 0.0 {
        0.0
    } else {
        dot(a, b) / (length(a) * length(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-5
    }

    #[test]
    fn mission_08_cosine() {
        assert!(close(cosine(&[1.0, 0.0], &[1.0, 0.0]), 1.0));
        assert!(close(cosine(&[1.0, 0.0], &[0.0, 1.0]), 0.0));
        assert!(
            close(cosine(&[1.0, 2.0], &[2.0, 4.0]), 1.0),
            "length doesn't matter"
        );
        assert!(close(cosine(&[1.0, 1.0], &[1.0, 0.0]), f32::sqrt(0.5)));
        assert!(close(cosine(&[1.0, 0.0], &[-1.0, 0.0]), -1.0));
        assert_eq!(
            cosine(&[0.0, 0.0], &[1.0, 1.0]),
            0.0,
            "zero vector gives 0, not NaN"
        );
    }

    #[test]
    fn mission_08_char_ngram_basics() {
        let e = CharNgramEmbedder { dims: 64, n: 3 };
        let v = e.embed("tokenization");
        assert_eq!(v.len(), 64);
        let mut length = 0.0;
        for x in &v {
            length += x * x;
        }
        assert!(close(length, 1.0), "the vector must be normalized");
        assert_eq!(
            e.embed("same text"),
            e.embed("same text"),
            "embedding must be deterministic"
        );
        assert!(
            e.embed("").iter().all(|x| *x == 0.0),
            "empty text gives a zero vector"
        );
    }

    #[test]
    fn mission_08_char_ngram_counts() {
        // A huge `dims` makes collisions very unlikely, so we can count n-grams exactly.
        let e = CharNgramEmbedder {
            dims: 1 << 20,
            n: 3,
        };
        let v = e.embed("token");
        // "#token#" has 7 characters, so 5 trigrams, each once: every non-zero entry is 1/sqrt(5).
        let mut nonzero = Vec::new();
        for x in &v {
            if *x != 0.0 {
                nonzero.push(*x);
            }
        }
        assert_eq!(nonzero.len(), 5);
        assert!(close(nonzero[0], 1.0 / f32::sqrt(5.0)));
        // Characters, not bytes: "été" -> "#été#" -> 3 trigrams.
        let mut count = 0;
        for x in e.embed("été") {
            if x != 0.0 {
                count += 1;
            }
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn mission_08_char_ngrams_handle_spelling_variants() {
        let e = CharNgramEmbedder::default();
        let british = e.embed("tokenisation");
        let american = e.embed("tokenization");
        let other = e.embed("quantization");
        assert!(cosine(&british, &american) > cosine(&british, &other));
        assert!(cosine(&british, &american) > 0.5);

        // The word embedder can't see this at all: different words share nothing.
        let w = WordEmbedder { dims: 4096 };
        assert_eq!(
            cosine(&w.embed("tokenisation"), &w.embed("tokenization")),
            0.0
        );
    }
}
