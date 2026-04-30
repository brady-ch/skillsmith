//! CLI: strip caveman fluff (stdin, file, or `--hook` JSON).

use std::fs;
use std::io::{self, Read};

use clap::Parser;
use serde::Deserialize;
use serde_json::json;
use skillsmith::caveman_defluff::defluff;

#[derive(Parser, Debug)]
#[command(name = "caveman-defluff")]
struct Args {
    /// Optional UTF-8 text file (default: stdin).
    path: Option<std::path::PathBuf>,
    /// Do not strip a/an/the.
    #[arg(long)]
    no_articles: bool,
    /// Read JSON with `prompt` from stdin; print JSON with `continue` and defluffed `prompt`.
    #[arg(long)]
    hook: bool,
}

#[derive(Deserialize)]
struct HookInput {
    prompt: String,
}

fn main() {
    let args = Args::parse();
    let articles = !args.no_articles;

    if args.hook {
        let mut raw = String::new();
        if io::stdin().read_to_string(&mut raw).is_err() {
            eprintln!("failed to read stdin");
            std::process::exit(1);
        }
        let parsed: HookInput = match serde_json::from_str(raw.trim()) {
            Ok(v) => v,
            Err(e) => {
                println!(
                    "{}",
                    json!({"continue": false, "user_message": format!("invalid JSON: {e}")})
                );
                std::process::exit(2);
            }
        };
        let cleaned = defluff(&parsed.prompt, articles);
        println!("{}", json!({"continue": true, "prompt": cleaned}));
        return;
    }

    let text = match &args.path {
        Some(p) => match fs::read_to_string(p) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        },
        None => {
            let mut buf = String::new();
            if io::stdin().read_to_string(&mut buf).is_err() {
                eprintln!("failed to read stdin");
                std::process::exit(1);
            }
            buf
        }
    };
    print!("{}", defluff(&text, articles));
}
