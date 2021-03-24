//! CLI program to generate random text.

mod options;
mod randstr;

use std::io::{self, Write};

use unicode_width::UnicodeWidthStr;

use crate::options::{CountType, Source};

fn main() {
    let options = match options::get() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    // Build words generator.
    let chain_storage;
    let mut chain = lipsum::MarkovChain::new();

    match options.source {
        Source::Lipsum => {
            chain.learn(lipsum::LOREM_IPSUM);
            chain.learn(lipsum::LIBER_PRIMUS);
        }

        Source::Random => {
            chain_storage = randstr::generate();
            chain.learn(&chain_storage);
        }

        _ => todo!(),
    }

    // Compute width to wrap content.
    let wrap_width = match options.wrap {
        Some(w) => w,
        None => match terminal_size::terminal_size() {
            Some((terminal_size::Width(w), _)) => w as usize,
            None => 80,
        },
    };

    // Use buffered output.
    let stdout_handle = io::stdout();
    let mut stdout = io::BufWriter::new(stdout_handle.lock());

    macro_rules! w {
        ($($tt:tt)*) => {
            if write!(&mut stdout, $($tt)*).is_err() {
                return;
            }
        }
    }

    let mut column = 0;
    let mut count = 0;

    macro_rules! newline {
        () => {
            column = 0;
            w!("\n");
        };
    }

    macro_rules! space {
        () => {
            column += 1;
            if column >= wrap_width {
                newline!();
            } else {
                w!(" ");
            }
        };
    }

    let mut sentence_start = true;

    for word in chain.iter() {
        let num_chars = word.chars().count();
        count += match options.count_type {
            CountType::Chars => num_chars,
            CountType::Words => 1,
        };

        if count > options.count {
            break;
        }

        // New line if the new word exceeds wrap_width.
        let word_width = UnicodeWidthStr::width(word);
        if column + word_width >= wrap_width {
            newline!();
        }

        if sentence_start {
            // Convert the first letter in the sentence to uppercase.
            let mut chars = word.chars();
            if let Some(c) = chars.next() {
                w!("{}", c.to_uppercase());
                for c in chars {
                    w!("{}", c);
                }
            }
        } else {
            if column > 0 {
                space!();
            }

            w!("{}", word);
        }

        sentence_start = word.ends_with('.');
        if sentence_start {
            space!();
        }

        column += word_width;
    }

    w!("\n");
}
