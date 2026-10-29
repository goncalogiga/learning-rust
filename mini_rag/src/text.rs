// ============================================================================
//  Missions 4, 5 and 6: working with text (Chapter 8.2 and Chapter 10.3)
// ============================================================================

use std::collections::HashSet;

// MISSION 4: split text into lowercase words.
//
//   "Self-attention costs O(n²)!"  ->  ["self", "attention", "costs", "o", "n²"]
//
// A word is a run of characters for which `char::is_alphanumeric` is true. Anything else
// (spaces, punctuation, hyphens) separates words. Lowercase with `c.to_lowercase()`, which
// returns an iterator because some characters lowercase to more than one character, so push
// them all with a `for` loop.
pub fn tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut word = String::new();

    for c in text.chars() {
        if c.is_alphanumeric() {
            word.extend(c.to_lowercase());
        } else if !word.is_empty() {
            tokens.push(std::mem::take(&mut word));
        }
    }

    if !word.is_empty() {
        tokens.push(word)
    }

    tokens
}

// Very common words that carry no meaning on their own. Without this list, a query like
// "how is tokenisation done" matches every chunk that contains "how" or "is".
pub const STOPWORDS: [&str; 57] = [
    "a", "about", "after", "all", "an", "and", "are", "as", "at", "be", "but", "by", "can", "do",
    "does", "done", "for", "from", "get", "has", "have", "how", "i", "if", "in", "into", "is",
    "it", "its", "me", "my", "not", "of", "on", "or", "s", "should", "so", "t", "than", "that",
    "the", "their", "them", "then", "there", "these", "they", "this", "to", "was", "what", "when",
    "which", "why", "with", "you",
];

pub fn is_stopword(word: &str) -> bool {
    STOPWORDS.contains(&word)
}

// MISSION 4b: like `tokenize`, but without the stopwords. Use `tokenize` and `is_stopword`.
//
//   "How is tokenisation done?"  ->  ["tokenisation"]
//
// BM25, the embedders and `best_sentence` all use this.
pub fn content_words(text: &str) -> Vec<String> {
    tokenize(text)
        .into_iter()
        .filter(|t| !is_stopword(t))
        .collect()
}

// MISSION 5a: split a document body into paragraphs.
//
// Paragraphs are separated by a blank line, i.e. "\n\n". Trim each one and skip empty ones.
// The result holds `&str` slices pointing into `text`: no copying, no new Strings.
//
// There are no lifetime annotations on this signature, yet the compiler knows the output
// borrows from `text`. Which elision rule is that?
pub fn paragraphs(text: &str) -> Vec<&str> {
    text.split("\n\n")
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .collect()
}

// MISSION 6: find the sentence in `chunk` that best matches `query`, to show as a snippet.
//
// - Sentences: `chunk.split_inclusive(". ")`, then `.trim()` each one.
// - A sentence's score is how many *different* query words appear among the sentence's
//   words. Use `content_words` for both.
// - Return the sentence with the highest score. On a tie, keep the earlier one. If no sentence
//   matches at all, return the first sentence.
//
// Part 2 of this mission is about the lifetimes in this signature. See the README.
pub fn best_sentence<'a>(chunk: &'a str, query: &'a str) -> &'a str {
    let words_set = |text| content_words(text).into_iter().collect::<HashSet<_>>();

    let sentences: Vec<&str> = chunk.split_inclusive(". ").map(str::trim).collect();
    let query_words = words_set(query);

    let mut best_idx = 0;
    let mut best_score = 0;

    for (idx, sentence) in sentences.iter().enumerate() {
        let sentence_words = words_set(sentence);
        let score = query_words.intersection(&sentence_words).count();

        if score > best_score {
            best_score = score;
            best_idx = idx;
        }
    }

    sentences[best_idx]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mission_04_tokenize() {
        assert_eq!(tokenize("Hello, World!"), vec!["hello", "world"]);
        assert_eq!(
            tokenize("Self-attention costs O(n²)!"),
            vec!["self", "attention", "costs", "o", "n²"]
        );
        assert_eq!(
            tokenize("  GPT-4 and LoRA  "),
            vec!["gpt", "4", "and", "lora"]
        );
        assert_eq!(tokenize("Ünïcode ÉTÉ"), vec!["ünïcode", "été"]);
        assert!(tokenize("").is_empty());
        assert!(tokenize("... --- !!!").is_empty());
    }

    #[test]
    fn mission_04_content_words() {
        assert_eq!(
            content_words("How is tokenisation done?"),
            vec!["tokenisation"]
        );
        assert_eq!(
            content_words("The KV cache and the weights"),
            vec!["kv", "cache", "weights"]
        );
        assert!(content_words("what is it").is_empty());
    }

    #[test]
    fn mission_05_paragraphs() {
        let body = "First paragraph,\nstill first.\n\n  Second.  \n\n\n\nThird.";
        assert_eq!(
            paragraphs(body),
            vec!["First paragraph,\nstill first.", "Second.", "Third."]
        );
        assert!(paragraphs("\n\n  \n\n").is_empty());
    }

    #[test]
    fn mission_05_paragraphs_borrow_from_the_input() {
        let body = String::from("One.\n\nTwo.");
        let parts = paragraphs(&body);
        // Same address: the slice points into `body`, nothing was copied.
        assert_eq!(parts[1].as_ptr(), body[6..].as_ptr());
    }

    #[test]
    fn mission_06_best_sentence() {
        let chunk = "Cats purr when happy. Dogs bark at strangers. Birds sing at dawn.";
        assert_eq!(
            best_sentence(chunk, "why do dogs bark"),
            "Dogs bark at strangers."
        );
        assert_eq!(
            best_sentence(chunk, "SINGING birds?"),
            "Birds sing at dawn."
        );
        // Repeating a query word doesn't count twice: "cats cats cats bark" scores 1 for
        // both the first and the second sentence, so the earlier one wins.
        assert_eq!(
            best_sentence(chunk, "cats cats cats bark"),
            "Cats purr when happy."
        );
        // Stopwords don't count: "at" is in two sentences but means nothing.
        assert_eq!(best_sentence(chunk, "at dawn"), "Birds sing at dawn.");
        // Nothing matches: fall back to the first sentence.
        assert_eq!(
            best_sentence(chunk, "quantum physics"),
            "Cats purr when happy."
        );
        assert_eq!(best_sentence("Only one", "nothing"), "Only one");
    }

    // MISSION 6, part 2: uncomment this test (select it and remove the `//`), run
    // `cargo test mission_06`, and read the compiler error. Then fix the signature of
    // `best_sentence` so it compiles. The README explains what's going on.
    //
    // #[test]
    // fn mission_06_snippet_outlives_query() {
    //     let chunk = String::from("Cats purr. Dogs bark.");
    //     let snippet;
    //     {
    //         let query = String::from("dogs");
    //         snippet = best_sentence(&chunk, &query);
    //     } // `query` is dropped here...
    //     assert_eq!(snippet, "Dogs bark."); // ...but `snippet` is still used here.
    // }
}
