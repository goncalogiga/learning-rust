// ============================================================================
//  Missions 2, 3 and 5: documents, queries and chunks
// ============================================================================

use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use crate::error::RagError;
use crate::text::paragraphs;

// One markdown file. The first line is `# Title`, everything after it is the body.
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    pub id: String, // the file name without `.md`, e.g. "attention"
    pub title: String,
    pub body: String,
}

// One line of `queries.tsv`: a question, and the id of the document that answers it.
#[derive(Debug, Clone, PartialEq)]
pub struct LabeledQuery {
    pub query: String,
    pub relevant: String,
}

#[derive(Debug)]
pub struct Corpus {
    pub docs: Vec<Document>,
}

// A chunk is one paragraph of one document. It owns nothing: both fields are references
// into a `Document` that lives in the `Corpus`.
//
// The `'a` says: "a Chunk<'a> cannot outlive the data it points to". Read Chapter 10.3,
// "Lifetime Annotations in Struct Definitions", then look at how `Corpus::chunks` uses it.
//
// `Copy` is fine here because copying a Chunk only copies two references and a number.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Chunk<'a> {
    pub id: usize, // position in the list returned by `Corpus::chunks`, unique per chunk
    pub doc: &'a Document,
    pub text: &'a str,
}

impl Chunk<'_> {
    // The text we index: the document title plus the paragraph. The title gives every chunk
    // a bit of context ("this paragraph is about Attention").
    pub fn searchable_text(&self) -> String {
        format!("{} {}", self.doc.title, self.text)
    }
}

impl Document {
    // MISSION 2a: build a Document from the text of a markdown file.
    //
    //   "# Attention\n\nAttention lets every token..."  ->  Ok(Document { id, title: "Attention", body: "Attention lets..." })
    //
    // - The first line must start with "# " and have a non-empty title after it. If not, return
    //   RagError::Parse { file: "<id>.md", line: 1, reason: "expected a '# Title' line" }.
    // - The body is everything after the first line, with `.trim()` applied. If the body is
    //   empty, return RagError::Parse { file: "<id>.md", line: 2, reason: "document has no body" }.
    pub fn parse(id: &str, text: &str) -> Result<Document, RagError> {
        todo!("Mission 2")
    }
}

// MISSION 2b: parse the contents of a queries file. `file` is only used in error messages.
//
// - Each line is `<query><TAB><doc id>`. Trim both parts.
// - Skip blank lines and lines starting with '#'.
// - A line without a tab, or with an empty query or doc id, is an error:
//   RagError::Parse { file, line: <1-based line number>, reason: "expected <query><TAB><doc id>" }
pub fn parse_queries(file: &str, text: &str) -> Result<Vec<LabeledQuery>, RagError> {
    todo!("Mission 2")
}

// MISSION 3b: read a queries file from disk and parse it with `parse_queries`.
// Use `path.display().to_string()` as the file name for error messages.
pub fn load_queries(path: &Path) -> Result<Vec<LabeledQuery>, RagError> {
    todo!("Mission 3")
}

impl Corpus {
    // MISSION 3a: load every `.md` file in `dir`.
    //
    // - `fs::read_dir(dir)?` gives you an iterator of `Result<DirEntry, io::Error>`. Loop over it
    //   with `for entry in ...`, and use `?` on each entry too.
    // - Skip files whose extension is not "md". `path.extension()` returns an `Option<&OsStr>`,
    //   and you can compare it with `Some(OsStr::new("md"))`.
    // - The id is the file name without extension: `path.file_stem()`. Turn it into a String
    //   with `.to_string_lossy().to_string()`.
    // - Docs must be sorted by id (directory order is random): `docs.sort_by(...)`, see the hint.
    // - No `.md` files at all: return RagError::EmptyCorpus(dir.to_path_buf()).
    pub fn load(dir: &Path) -> Result<Corpus, RagError> {
        todo!("Mission 3")
    }

    // MISSION 5b: cut every document into chunks, one per paragraph (use `paragraphs`).
    //
    // - Go through `self.docs` in order, and each document's paragraphs in order.
    // - Chunk ids count up from 0 across the whole corpus: 0, 1, 2, ...
    //
    // The return type is `Vec<Chunk<'_>>`: the chunks borrow from `self`. Elision rule 1 gives
    // `&self` a lifetime, and rule 3 hands it to the output.
    pub fn chunks(&self) -> Vec<Chunk<'_>> {
        todo!("Mission 5")
    }

    // The document with this id, if there is one. (`|doc| ...` is a closure, Chapter 13.)
    pub fn get(&self, id: &str) -> Option<&Document> {
        self.docs.iter().find(|doc| doc.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{data_dir, doc};

    #[test]
    fn mission_02_parse_document() {
        let d = Document::parse("cats", "# All About Cats\n\nCats purr.\n\nCats nap.\n").unwrap();
        assert_eq!(d.id, "cats");
        assert_eq!(d.title, "All About Cats");
        assert_eq!(d.body, "Cats purr.\n\nCats nap.");
    }

    #[test]
    fn mission_02_parse_document_errors() {
        match Document::parse("cats", "Cats purr.\n") {
            Err(RagError::Parse { file, line, .. }) => {
                assert_eq!(file, "cats.md");
                assert_eq!(line, 1);
            }
            other => panic!("expected a parse error on line 1, got {other:?}"),
        }
        assert!(
            Document::parse("cats", "#  \nbody").is_err(),
            "an empty title is an error"
        );
        match Document::parse("cats", "# Cats\n\n   \n") {
            Err(RagError::Parse { line: 2, .. }) => {}
            other => panic!("expected a parse error on line 2, got {other:?}"),
        }
    }

    #[test]
    fn mission_02_parse_queries() {
        let text = "# a comment\n\nwhat is a cat\tcats\n  why dogs bark \t dogs \n";
        let queries = parse_queries("q.tsv", text).unwrap();
        assert_eq!(
            queries,
            vec![
                LabeledQuery {
                    query: String::from("what is a cat"),
                    relevant: String::from("cats"),
                },
                LabeledQuery {
                    query: String::from("why dogs bark"),
                    relevant: String::from("dogs"),
                },
            ]
        );
    }

    #[test]
    fn mission_02_parse_queries_reports_the_line() {
        let text = "# header\nok query\tcats\nno tab on this line\n";
        match parse_queries("q.tsv", text) {
            Err(e) => assert_eq!(e.to_string(), "q.tsv:3: expected <query><TAB><doc id>"),
            Ok(q) => panic!("expected an error, got {q:?}"),
        }
        assert!(parse_queries("q.tsv", "\tcats\n").is_err(), "empty query");
        assert!(
            parse_queries("q.tsv", "a query\t \n").is_err(),
            "empty doc id"
        );
    }

    #[test]
    fn mission_03_load_corpus() {
        let corpus = Corpus::load(&data_dir().join("docs")).unwrap();
        assert_eq!(corpus.docs.len(), 18);
        assert_eq!(corpus.docs[0].id, "attention", "docs must be sorted by id");
        assert_eq!(corpus.docs[17].id, "vector_databases");
        assert_eq!(
            corpus.get("rag").unwrap().title,
            "Retrieval-Augmented Generation"
        );
    }

    #[test]
    fn mission_03_load_errors() {
        match Corpus::load(Path::new("no/such/dir")) {
            Err(RagError::Io(_)) => {}
            other => panic!("expected RagError::Io, got {other:?}"),
        }
        // `src/` exists but has no .md files in it.
        let src = data_dir().join("..").join("src");
        match Corpus::load(&src) {
            Err(RagError::EmptyCorpus(dir)) => assert_eq!(dir, src),
            other => panic!("expected RagError::EmptyCorpus, got {other:?}"),
        }
    }

    #[test]
    fn mission_03_load_queries() {
        let queries = load_queries(&data_dir().join("queries.tsv")).unwrap();
        assert_eq!(queries.len(), 42);
        assert_eq!(queries[0].relevant, "attention");
    }

    #[test]
    fn mission_05_chunks() {
        let corpus = Corpus {
            docs: vec![doc("a", "A one.\n\nA two."), doc("b", "B one.")],
        };
        let chunks = corpus.chunks();
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].text, "A one.");
        assert_eq!(chunks[1].text, "A two.");
        assert_eq!(chunks[2].text, "B one.");
        assert_eq!(chunks[2].doc.id, "b");
        for (i, chunk) in chunks.iter().enumerate() {
            assert_eq!(chunk.id, i, "chunk ids count up from 0");
        }
    }
}
