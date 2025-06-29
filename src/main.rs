use std::{collections::HashMap, fs};

struct Arguments {
    filepath: String,
    vocabulary_size: u16,
}

impl Arguments {
    fn parse() -> Self {
        let mut args = std::env::args();
        let mut vocabulary_size = None;
        let mut filepath = None;

        let program_name = args.next().unwrap();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" | "--help" => {
                    usage(&program_name);
                    std::process::exit(0)
                }
                "-n" | "--vocabulary-size" => {
                    let Some(size_str) = args.next() else {
                        usage(&program_name);
                        eprintln!("Error: -n flag requires a vocabulary size argument");
                        std::process::exit(1);
                    };

                    if let Ok(value) = size_str.parse::<u16>() {
                        vocabulary_size = Some(value)
                    } else {
                        usage(&program_name);
                        eprintln!("Error: Invalid vocabulary size '{}'", size_str);
                        std::process::exit(1);
                    };
                }
                _ if filepath.is_none() => filepath = Some(arg),
                _ => {
                    usage(&program_name);
                    eprintln!("Error: Multiple file paths provided");
                    std::process::exit(1);
                }
            }
        }

        let Some(filepath) = filepath else {
            usage(&program_name);
            eprintln!("Error: Missing filepath");
            std::process::exit(1);
        };

        let Some(vocabulary_size) = vocabulary_size else {
            usage(&program_name);
            eprintln!("Error: Missing vocabulary_size");
            std::process::exit(1);
        };

        Arguments {
            filepath,
            vocabulary_size,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Token {
    Byte(u8),
    Pair(u32, u32),
}

fn main() {
    let Arguments {
        filepath,
        vocabulary_size,
    } = Arguments::parse();

    let text = fs::read(filepath).unwrap();
    let mut vocabulary = Vec::with_capacity(vocabulary_size as usize);
    for byte in 0..=255u8 {
        vocabulary.push(Token::Byte(byte));
    }

    let mut tokens: Vec<u32> = text.into_iter().map(|ch| ch as u32).collect();

    let mut frequencies = HashMap::with_capacity(vocabulary_size as usize);
    for _ in 0..vocabulary_size {
        let pair = most_frequent_token_pair(&tokens, &mut frequencies);
        frequencies.clear();
        replace_token_pair(&mut tokens, pair, vocabulary.len() as u32);
        vocabulary.push(pair);
    }

    for token_id in tokens.iter() {
        let str = render_token(&vocabulary, *token_id);
        print!("[{str}]");
    }
}

fn usage(program_name: &str) {
    eprintln!("Usage: {} -n <vocabulary_size> <filepath>", program_name);
    eprintln!();
    eprintln!("Performs Byte Pair Encoding (BPE) tokenization on a text file.");
    eprintln!();
    eprintln!("Arguments:");
    eprintln!("  <filepath>              Path to the text file to tokenize");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  -n <vocabulary_size>    Number of token pairs to learn");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {} -n 1000 input.txt", program_name);
}

fn most_frequent_token_pair(tokens: &[u32], frequencies: &mut HashMap<Token, u32>) -> Token {
    for i in 1..tokens.len() {
        let pair = Token::Pair(tokens[i - 1], tokens[i]);
        *frequencies.entry(pair).or_insert(0) += 1;
    }

    frequencies
        .iter()
        .max_by_key(|(_key, count)| *count)
        .map(|(token, _count)| *token)
        .unwrap()
}

fn render_token(vocabulary: &[Token], token_id: u32) -> String {
    let mut str = String::with_capacity(32);

    fn inner_render_token(vocabulary: &[Token], token_id: u32, str: &mut String) {
        match vocabulary[token_id as usize] {
            Token::Byte(byte) => str.push(byte as char),
            Token::Pair(left, right) => {
                inner_render_token(vocabulary, left, str);
                inner_render_token(vocabulary, right, str);
            }
        }
    }
    inner_render_token(vocabulary, token_id, &mut str);
    str
}

fn replace_token_pair(tokens: &mut Vec<u32>, pair: Token, pair_id: u32) {
    let mut read_index = 0;
    let mut write_index = 0;

    while read_index < tokens.len() {
        if read_index < tokens.len() - 1 {
            let cur_pair = Token::Pair(tokens[read_index], tokens[read_index + 1]);

            if cur_pair == pair {
                tokens[write_index] = pair_id;
                write_index += 1;
                read_index += 2;
                continue;
            }
        }

        tokens[write_index] = tokens[read_index];
        write_index += 1;
        read_index += 1;
    }

    tokens.truncate(write_index);
}
