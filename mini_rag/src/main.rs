// The command-line tool. Nothing to do here: it only works once your missions are done.
//
//   cargo run -- search "how does attention work"
//   cargo run -- search "how does attention work" --retriever bm25 --k 3
//   cargo run -- search "what is in my notes about X" --dir ~/notes
//   cargo run -- eval
//   cargo run -- eval --k 3 --misses
//
// It is still worth reading: it's the "user" of your library, and it shows how the pieces fit.

use std::env;
use std::path::PathBuf;
use std::process;

use mini_rag::corpus::load_queries;
use mini_rag::text::best_sentence;
use mini_rag::{
    Bm25Index, CharNgramEmbedder, Corpus, Hybrid, RagError, Retriever, VectorIndex, evaluate,
};

const USAGE: &str = "\
usage:
  mini_rag search <query> [--retriever bm25|vector|hybrid] [--k N] [--dir DOCS]
  mini_rag eval [--k N] [--misses] [--dir DOCS] [--queries FILE]";

struct Options {
    command: String,
    query: String,
    retriever: String,
    k: usize,
    dir: PathBuf,
    queries: PathBuf,
    show_misses: bool,
}

fn main() {
    let options = match parse_args(env::args().skip(1).collect()) {
        Ok(options) => options,
        Err(message) => {
            eprintln!("{message}\n\n{USAGE}");
            process::exit(2);
        }
    };
    // `run` returns a Result. This is the one place where we turn an error into a message
    // for the user, using the `Display` impl from Mission 1.
    if let Err(e) = run(&options) {
        eprintln!("error: {e}");
        process::exit(1);
    }
}

fn run(options: &Options) -> Result<(), RagError> {
    let corpus = Corpus::load(&options.dir)?;
    // `chunks` borrows from `corpus`, and the indexes below borrow from `chunks`' documents.
    // That's why `corpus` has to stay alive until the end of this function.
    let chunks = corpus.chunks();
    let bm25 = Bm25Index::build(&chunks);
    let vector = VectorIndex::build(CharNgramEmbedder::default(), &chunks);
    // `&bm25` and `&vector` are Retrievers too, thanks to the blanket impl in retriever.rs.
    let hybrid = Hybrid::new(&bm25, &vector);

    if options.command == "eval" {
        let queries = load_queries(&options.queries)?;
        println!(
            "{} queries, {} documents, {} chunks\n",
            queries.len(),
            corpus.docs.len(),
            chunks.len()
        );
        // Three different types, one generic function each time.
        let reports = vec![
            evaluate(&bm25, &queries, options.k),
            evaluate(&vector, &queries, options.k),
            evaluate(&hybrid, &queries, options.k),
        ];
        for report in &reports {
            println!("{report}");
        }
        if options.show_misses {
            for report in &reports {
                println!("\nmissed by {}:", report.name);
                for query in &report.misses {
                    println!("  - {query}");
                }
            }
        }
        return Ok(());
    }

    // `bm25`, `vector` and `hybrid` have different types, so we can't put them in one variable
    // (that needs trait objects, Chapter 18). Instead, each arm calls the same generic function.
    match options.retriever.as_str() {
        "bm25" => print_results(&bm25, &options.query, options.k),
        "vector" => print_results(&vector, &options.query, options.k),
        _ => print_results(&hybrid, &options.query, options.k),
    }
    Ok(())
}

fn print_results<R: Retriever>(retriever: &R, query: &str, k: usize) {
    let hits = retriever.search(query, k);
    println!("{} results from {}\n", hits.len(), retriever.name());
    for (i, hit) in hits.iter().enumerate() {
        println!(
            "{}. {} ({}.md)  score {:.4}",
            i + 1,
            hit.chunk.doc.title,
            hit.chunk.doc.id,
            hit.score
        );
        println!("   > {}\n", best_sentence(hit.chunk.text, query));
    }
}

fn parse_args(args: Vec<String>) -> Result<Options, String> {
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
    let mut options = Options {
        command: String::new(),
        query: String::new(),
        retriever: String::from("hybrid"),
        k: 3,
        dir: data.join("docs"),
        queries: data.join("queries.tsv"),
        show_misses: false,
    };

    let mut words = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let arg = args[i].as_str();
        // Flags that take a value read the next argument too.
        let needs_value = matches!(arg, "--retriever" | "--k" | "--dir" | "--queries");
        let value = if needs_value {
            i += 1;
            match args.get(i) {
                Some(v) => v.clone(),
                None => return Err(format!("{arg} needs a value")),
            }
        } else {
            String::new()
        };
        match arg {
            "--retriever" => match value.as_str() {
                "bm25" | "vector" | "hybrid" => options.retriever = value,
                other => return Err(format!("unknown retriever '{other}'")),
            },
            "--k" => match value.parse() {
                Ok(k) if k > 0 => options.k = k,
                _ => return Err(format!("--k needs a positive number, got '{value}'")),
            },
            "--dir" => options.dir = PathBuf::from(value),
            "--queries" => options.queries = PathBuf::from(value),
            "--misses" => options.show_misses = true,
            "--help" | "-h" => return Err(String::from("mini_rag: search your documents")),
            _ if arg.starts_with("--") => return Err(format!("unknown flag '{arg}'")),
            _ => words.push(arg.to_string()),
        }
        i += 1;
    }

    if words.is_empty() {
        return Err(String::from("missing command"));
    }
    options.command = words.remove(0);
    match options.command.as_str() {
        "search" => {
            if words.is_empty() {
                return Err(String::from("search needs a query"));
            }
            options.query = words.join(" ");
        }
        "eval" => {}
        other => return Err(format!("unknown command '{other}'")),
    }
    Ok(options)
}
