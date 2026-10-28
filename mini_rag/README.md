# 🔎 mini_rag: Chapters 7 to 10 practice

You'll build the **retrieval half of a RAG pipeline**: the part that finds the right passages to
paste into an LLM's prompt. It loads a folder of markdown docs, cuts them into chunks, indexes
them two ways, fuses the results, and **measures** how good the search is:

```
$ cargo run -- search "what does a higher temprature do" --k 2
2 results from hybrid(bm25+vector)

1. Sampling and Temperature (sampling.md)  score 0.0328
   > For brainstorming and creative writing, higher temperature gives more variety.

2. Sampling and Temperature (sampling.md)  score 0.0161
   > Temperature rescales the logits before the softmax.

$ cargo run -- eval
42 queries, 18 documents, 72 chunks

bm25                  recall@3 0.952  MRR 0.952
vector                recall@3 1.000  MRR 0.940
hybrid(bm25+vector)   recall@3 1.000  MRR 0.984
```

No external crates: everything is the standard library and your code. The corpus in
`data/docs/` is 18 short notes on LLM topics (attention, quantization, RAG, ...), and
`data/queries.tsv` is a labelled eval set of 42 deliberately sloppy queries (typos, British
spellings, paraphrases).

### What's real and what's a toy

| Piece | Here | In production |
|---|---|---|
| Keyword search | BM25, the real formula | the same (Elasticsearch, Tantivy, ...) |
| Fusion | Reciprocal Rank Fusion, the real algorithm | the same |
| Eval | recall@k and MRR on labelled queries | the same, with more queries |
| Embeddings | hashed character n-grams (a toy) | a neural embedding model |
| Vector search | brute-force cosine | brute force, or an ANN index like HNSW |

The toy embedder sits behind an `Embedder` trait. Once you've reached the async chapter,
you can swap in a real model by implementing that one trait. The index, the hybrid search and
the eval don't change at all. That's the main point of Chapter 10.

## What you'll practise

| Book chapter | Where you'll use it |
|---|---|
| 7 Modules | the crate is split into 9 modules (see the map below); `mod`, `pub`, `use crate::...`, `pub use` |
| 8.1 / 8.2 Vectors, Strings | tokenizing, splitting paragraphs, `&str` slices, `chars()` vs bytes |
| 8.3 Hash Maps | word counts and document frequencies for BM25, deduplication in hybrid search |
| 9.2 `Result` and `?` | loading files, a custom error type, `From<io::Error>` |
| 10.1 Generics | `top_k<T>`, `VectorIndex<E>`, `Hybrid<A, B>`, `evaluate<R>` |
| 10.2 Traits | implementing `Display`, `Embedder`, `Retriever`; a default method; trait bounds; a blanket impl |
| 10.3 Lifetimes | `Chunk<'a>` borrowing from the corpus, elision, writing `'a` yourself |

## How to work through it

Each mission is a `todo!("Mission N")` in `src/`. The comment above each `todo!` tells you
exactly what to do; this README adds context, hints, and experiments.

1. Read the mission below, then the comment in the code.
2. Write the code.
3. Run only that mission's tests. Note the **two-digit** number, so `mission_01` doesn't
   also match `mission_10`:
   ```bash
   cargo test mission_01
   ```
4. Green? Move on.

Do the missions **in order**, because later ones use earlier ones. If a test panics with
`not yet implemented: Mission 4`, the code you're testing calls a mission you haven't done yet.

Budget **about 5 hours**, over several sessions. Parts A and B are warm-ups for Chapters 7 to 9;
Parts C and D are the Chapter 10 workout.

### Map of the crate (Chapter 7)

This project is a **library crate** (`src/lib.rs`) with a **binary crate** on top (`src/main.rs`).
Both live in the same package, as described in 7.1. `lib.rs` declares the modules; each one is
its own file (7.5).

```
src/
  lib.rs        mod declarations + `pub use` re-exports            (read it first)
  error.rs      RagError                                          Mission 1
  corpus.rs     Document, LabeledQuery, Corpus, Chunk<'a>          Missions 2, 3, 5
  text.rs       tokenize, content_words, paragraphs, best_sentence Missions 4, 5, 6
  retriever.rs  Hit<'a>, trait Retriever, top_k                    Mission 7
  embed.rs      trait Embedder, WordEmbedder, CharNgramEmbedder    Mission 8
  vector.rs     VectorIndex<'a, E>                                 Mission 9
  bm25.rs       Bm25Index<'a>                                      Mission 10
  hybrid.rs     Hybrid<A, B>                                       Mission 11
  eval.rs       evaluate, EvalReport                               Mission 12
  main.rs       the CLI (given)
  testing.rs    test helpers (given)
```

👉 Before you start, open `lib.rs` and `main.rs`. Notice that `main.rs` says
`use mini_rag::Corpus`, not `crate::Corpus`: to the binary, the library is an external crate,
so it only sees what's `pub`. Try removing `pub` from `pub mod corpus;` in `lib.rs` and running
`cargo build`. Then put it back.

---

# Part A: loading data (Chapters 7 to 9)

## Mission 1: An error type (`Display` and `From`)

**Book: 9.2, "Propagating Errors" and "A Shortcut for Propagating Errors: the ? Operator"**

Every function that can fail in this library returns `Result<_, RagError>`. Fill in `error.rs`:

1. **`impl fmt::Display for RagError`**. This is the first time you implement a standard
   library trait yourself. `Display` is what `{}` and `.to_string()` use.
2. **`impl From<io::Error> for RagError`**. When `?` sees an error of the wrong type, it calls
   `From::from` to convert it. With this impl, any function returning `Result<_, RagError>`
   can use `?` on file operations.

Also read the empty `impl std::error::Error for RagError {}` under them. Why can it be empty?
(Look up "Default Implementations" in 10.2.)

```bash
cargo test mission_01
```

<details><summary>Hint</summary>

`write!` works like `format!`, but writes into the formatter and returns the `fmt::Result` you
need:

```rust
match self {
    RagError::Io(e) => write!(f, "io error: {e}"),
    // ...
}
```
</details>

## Mission 2: Parsing documents and queries

**Book: 8.2 (string slices and methods), 9.2 (returning `Err`)**

Fill in `Document::parse` and `parse_queries` in `corpus.rs`. The comments specify the format and
the errors. Bad input should never panic, only return an `Err` that says **where** the problem is.
(When the project is done, delete the tab on one line of `queries.tsv` and run
`cargo run -- eval` to see your error message at work.)

Useful `&str` methods (`rustup doc --std` opens the standard library docs, offline):

| Method | Does |
|---|---|
| `s.split_once('\n')` | `Some((before, after))` at the first `\n`, or `None` |
| `s.strip_prefix("# ")` | `Some(rest)` if `s` starts with `"# "`, or `None` |
| `s.trim()` | removes surrounding whitespace |
| `s.lines()` | iterates over lines |

```bash
cargo test mission_02
```

<details><summary>Hint: line numbers</summary>

`for (i, line) in text.lines().enumerate()` gives you `i = 0, 1, 2, ...`. Humans count lines from 1.
</details>

<details><summary>Hint: let...else from Chapter 6</summary>

```rust
let Some(title) = first_line.strip_prefix("# ") else {
    return Err(RagError::Parse { /* ... */ });
};
```
</details>

## Mission 3: Loading from disk (`?` everywhere)

**Book: 9.2, "A Shortcut for Propagating Errors: the ? Operator"**

Fill in `Corpus::load` and `load_queries`. This is where Mission 1 pays off: `fs::read_dir`,
each directory entry, and `fs::read_to_string` all return `io::Error`, and `Document::parse`
returns `RagError`. With `?` and your `From` impl, they all flow into the same return type.

The tests load the real `data/` folder (18 documents, 42 queries).

```bash
cargo test mission_03
```

<details><summary>Hint: sorting</summary>

Directory order is not guaranteed, so sort the documents by id:

```rust
docs.sort_by(|a, b| a.id.cmp(&b.id));
```

The `|a, b| ...` part is a *closure*, a small inline function. It's Chapter 13; for now read it
as "to compare two docs `a` and `b`, compare their ids".
</details>

---

# Part B: text and lifetimes (Chapters 8 and 10.3)

## Mission 4: Tokenizing

**Book: 8.2, "Iterating Over Strings"**

Fill in `tokenize` and `content_words` in `text.rs`. Search engines don't compare raw text, they
compare *tokens*. Ours are simple: lowercase runs of letters and digits.

Notice the test with `"Ünïcode ÉTÉ"`. A Rust `String` is UTF-8, so a "character" can take 1 to 4
bytes. Always iterate with `.chars()` when you mean characters.

`content_words` then removes stopwords like "how" and "the". Without that, a query like
"how is tokenisation done" would match every chunk containing "how".

```bash
cargo test mission_04
```

<details><summary>Hint: the loop for `tokenize`</summary>

Keep a `current: String`. For each character, if it's alphanumeric, push its lowercase form onto
`current`. Otherwise, if `current` isn't empty, push it onto the result and start a new
`String`. Don't forget the last word after the loop.
</details>

## Mission 5: Paragraphs and chunks (references that point into other data)

**Book: 10.3, "Lifetime Annotations in Struct Definitions" and "Lifetime Elision"**

A RAG system never sends whole documents to the model, only the relevant **chunks**. Ours are
paragraphs.

1. `paragraphs(text: &str) -> Vec<&str>` returns **slices of the input**: no copying. One of
   the tests even checks the memory address. The signature has no lifetime annotations; which
   elision rule lets the compiler work it out?
2. `Corpus::chunks(&self) -> Vec<Chunk<'_>>`. Read the definition of `Chunk<'a>` first: it
   holds a `&'a Document` and a `&'a str`. So a chunk is a few pointers, whatever the length of
   its text, and it can never outlive the `Corpus` it points into.

```bash
cargo test mission_05
```

👉 Once this works, open `main.rs` and add `drop(corpus);` right after `let chunks = corpus.chunks();`.
Run `cargo build` and read the **first** error. In a language without a borrow checker, this
bug would be a use-after-free, or a garbage collector quietly keeping all the documents alive.
Remove the line again.

<details><summary>The three elision rules (10.3), as a reminder</summary>

1. Each reference parameter gets its own lifetime.
2. If there's exactly one input lifetime, it's used for all outputs.
3. If there's a `&self` or `&mut self`, its lifetime is used for all outputs.

`paragraphs` uses rule 2. `chunks` uses rule 3.
</details>

## Mission 6: The best sentence (writing `'a` yourself)

**Book: 10.3, "Lifetime Annotations in Function Signatures"**

The CLI shows the best sentence of each hit as a snippet. **Part 1:** fill in `best_sentence`
so that `cargo test mission_06` passes.

**Part 2:** this signature has two input references and no `self`, so elision can't decide
which input the output borrows from. You must write it. The signature you were given is the
"`longest`" signature from the book:

```rust
pub fn best_sentence<'a>(chunk: &'a str, query: &'a str) -> &'a str
```

It compiles, and your tests pass. Now go to the bottom of `text.rs`, uncomment the test
`mission_06_snippet_outlives_query`, and run `cargo test mission_06`. Read the error.
The snippet comes **only from `chunk`**, never from `query`, so why does the compiler complain
that `query` doesn't live long enough? Fix the signature so that the test compiles, without
changing the test.

<details><summary>Hint</summary>

`'a` on both inputs promises "the output lives no longer than the *shorter* of the two". Which
input actually needs `'a`?
</details>

---

# Part C: generics and traits (Chapters 10.1 and 10.2)

## Mission 7: `top_k<T>` and a default method

**Book: 10.1, "In Function Definitions"; 10.2, "Default Implementations"**

Every retriever ends the same way: "here are scores, keep the best `k`". Write it once, for any `T`.

👉 Before you start, try this: write the body as `scored.sort(); scored.truncate(k); scored`
and run `cargo build`. You'll get **two** errors: `T: Ord` and `f32: Ord` are not satisfied.
- `T: Ord`: to sort the tuples, Rust would compare the `T`s as well, and nothing says it can.
- `f32: Ord`: floats are only `PartialOrd`, because `NaN` is not equal to, smaller than, or
  bigger than anything (not even itself). So "sort these floats" has no total answer.
  That's a real concern when your scores come out of an ML model.

The working version needs **no trait bound on `T`**: it only compares the `f32`s.

Then fill in the default method `Retriever::best` in the trait. The test calls `best` on a
`FixedRetriever` (in `testing.rs`), which never wrote a `best` of its own.

```bash
cargo test mission_07
```

<details><summary>Hint: two ways to write top_k</summary>

With loops: repeatedly find the index of the highest score, `scored.remove(i)` it, push it to
the result, until you have `k` items or `scored` is empty. Use `>` (not `>=`) so that on a tie
the earlier item wins.

With a closure (Chapter 13 preview): `sort_by` is stable (ties keep their order), and
`total_cmp` gives floats a total order:

```rust
scored.sort_by(|a, b| b.1.total_cmp(&a.1)); // b before a: descending
scored.truncate(k);
scored
```
</details>

## Mission 8: An embedder (implementing a trait)

**Book: 10.2, "Implementing a Trait on a Type"**

Read the worked example `WordEmbedder` first. Then implement `Embedder` for `CharNgramEmbedder`
and write `cosine`.

Why character n-grams? Look at `data/queries.tsv`: "tokenisation", "quantised", "embedings",
"temprature". BM25 matches whole words, so it scores those at zero. "tokenisation" and
"tokenization" share most of their character n-grams, so they still match. The last test shows
both cases.

```bash
cargo test mission_08
```

<details><summary>Hint: the n-grams of one word</summary>

```rust
let chars: Vec<char> = format!("#{word}#").chars().collect();
let mut i = 0;
while i + self.n <= chars.len() {
    // build a String from chars[i..i + self.n], hash it, add 1.0 to that position
    i += 1;
}
```
</details>

## Mission 9: `VectorIndex<'a, E>` (generic struct, trait bound)

**Book: 10.1, "In Struct Definitions" and "In Method Definitions"; 10.2, "Trait Bound Syntax"**

Fill in `build` and `search` in `vector.rs`. `VectorIndex` works with *any* embedder: the
tests use `WordEmbedder` and `CharNgramEmbedder`, and a real neural model would work the same.

```bash
cargo test mission_09
```

👉 Once it works, try this: change `impl<'a, E: Embedder> Retriever for VectorIndex<'a, E>` to
`impl<'a, E> Retriever for VectorIndex<'a, E>` and run `cargo build`. Read the `help:` line.
Inside a generic function, `E` can only do what its bounds promise. That's the main difference
from Python duck typing: the check happens once, at compile time, for every possible `E`.
Put the bound back.

Something to think about: the struct itself (`pub struct VectorIndex<'a, E>`) has no bound,
only the `impl` blocks do. Why is that enough?

---

# Part D: search quality

## Mission 10: BM25 (hash maps)

**Book: 8.3, "Updating a Value Based on the Old Value"**

BM25 is the keyword ranking used by most search engines. For a query, a chunk's score is the
sum, over each query word, of:

```
              tf × (K1 + 1)
idf(word) × ─────────────────────────────────────
             tf + K1 × (1 − B + B × len / avg_len)
```

- `tf`: how many times the word appears in this chunk. More is better, but with diminishing
  returns: going from 1 to 2 helps a lot, from 10 to 11 barely. `K1 = 1.2` controls how fast
  it saturates.
- `idf`: how rare the word is across all chunks. A match on "quantization" says much more than
  a match on "model".
- `len / avg_len`: long chunks get a penalty, otherwise they would win just by containing more
  words. `B = 0.75` sets how strong the penalty is.

Fill in `build`, `doc_freq`, `idf` and `search` in `bm25.rs`.

```bash
cargo test mission_10
```

<details><summary>Hint: counting with a HashMap</summary>

```rust
let mut counts: HashMap<String, usize> = HashMap::new();
for word in words {
    *counts.entry(word).or_insert(0) += 1;
}
```

`entry(...).or_insert(0)` returns a `&mut usize` to the existing count, or to a fresh 0, and
`*` writes through it.
</details>

## Mission 11: Hybrid search, `Hybrid<A, B>`

**Book: 10.1 (several type parameters), 10.2 (trait bounds on `impl`)**

Keyword search is good at exact terms ("GPTQ", "HNSW"), vector search at fuzzy matches
("quantised"). Hybrid search runs both and merges the two ranked lists.

BM25 scores look like `7.18`, cosine scores like `0.41`, so you can't add them.
**Reciprocal Rank Fusion** ignores the scores and only uses ranks:

```
fused(chunk) = Σ over both lists of  1 / (60 + rank of the chunk in that list)
```

A chunk ranked 1st by both lists gets `2/61`; one ranked 1st by only one list gets `1/61`.

`Hybrid<A, B>` takes **two different types**. `main.rs` builds a
`Hybrid<&Bm25Index, &VectorIndex<CharNgramEmbedder>>`. The `&` works because of the
*blanket implementation* at the bottom of `retriever.rs`: read it, it's the last idea of 10.2.

And since `Hybrid` is itself a `Retriever`, you could pass a `Hybrid` into another `Hybrid`.

```bash
cargo test mission_11
```

## Mission 12: Evaluation

**Book: 10.2, "Traits as Parameters" (and `Display` once more)**

You don't improve a RAG system without measuring retrieval. Fill in `evaluate` and
`impl Display for EvalReport` in `eval.rs`:

- **recall@k**: for what fraction of queries is the right document somewhere in the top k?
- **MRR** (mean reciprocal rank): rank 1 scores 1, rank 2 scores ½, rank 3 scores ⅓, not
  found scores 0; average over all queries. It rewards putting the answer *first*, which
  matters when only the top chunks fit in the prompt.

`evaluate<R: Retriever>` is written once and works for all three retrievers, and for any you
add later.

```bash
cargo test mission_12
```

<details><summary>Hint: format specs</summary>

`{:<21}` pads to 21 characters, left-aligned. `{:.3}` prints 3 decimals.
`write!(f, "{:<21} recall@{} {:.3}  MRR {:.3}", self.name, self.k, self.recall, self.mrr)`
</details>

---

# 🏁 Finish line

```bash
cargo test           # all 36 tests (including the one you uncommented in Mission 6)
cargo run -- eval
```

You should see the numbers at the top of this file. (Everything is deterministic: the hash is
FNV-1a and every tie-break is specified. If you add things up in a different order, the last
digit may differ.)

Now use what you built, like you would at work:

1. **Error analysis.** `cargo run -- eval --misses`, and `cargo run -- eval --k 1 --misses`.
   Which queries does BM25 miss, and what do they have in common? Which does vector search
   miss? Why does hybrid miss almost none?
2. **Compare search modes.** `cargo run -- search "quantised models" --retriever bm25`, then
   `--retriever vector`, then without the flag (hybrid).
3. **Swap the embedder.** In `main.rs`, replace `CharNgramEmbedder::default()` with
   `mini_rag::WordEmbedder { dims: 4096 }` and run the eval. Then try
   `CharNgramEmbedder { dims: 4096, n: 4 }`. It's a one-line change because `VectorIndex` is
   generic. With one of them, hybrid becomes **worse than BM25 alone**. Why can fusing in a
   weak retriever hurt?
4. **Your own notes.** `cargo run -- search "..." --dir ~/path/to/notes`. Each `.md` file needs
   a `# Title` first line; if one doesn't, the error from Missions 1 and 2 tells you which file
   and which line.

## Where this goes next

These pick up chapters you haven't read yet, so come back to them later:

- **Ch. 11 (tests):** add an integration test in `tests/` that fails if hybrid MRR drops below
  0.95: a *retrieval regression test*, the kind that catches a "harmless" chunking change
  before it reaches users.
- **Ch. 13 (closures, iterators):** rewrite `tokenize`, `top_k` and the BM25 loops with iterator
  chains, and check that the eval numbers don't change.
- **Ch. 17 (async) + crates:** implement `Embedder` with a real model. The `fastembed` crate
  runs embedding models locally; an HTTP embeddings API works too. Then add the "G" of RAG:
  send the top chunks and the question to an LLM API and print its answer with citations
  (`chunk.doc.id`).
- **Ch. 18 (trait objects):** replace the `match` in `main.rs` with a `Box<dyn Retriever>`, and
  compare the two approaches.
- **Any time:** a `Reranker` trait that rescores the top 20 fused hits; weights for each
  retriever in `Hybrid`; recall at several values of k in one report.
