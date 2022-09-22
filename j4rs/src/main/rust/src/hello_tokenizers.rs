use tokenizers::tokenizer::{Result, Tokenizer};
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::RwLock;

//static GLOBAL_DATA: RwLock<Option<HashMap<u32, Tokenizer>>> = RwLock::new(None);
static GLOBAL_DATA: Mutex<Option<HashMap<u32, &Tokenizer>>> = Mutex::new(None);

fn main() -> Result<()> {
    // let global_data = *GLOBAL_DATA.lock().unwrap();
    // if global_data.is_none() {
        // global_data = Some(HashSet::new());
    // }
    // *global_data..unl



    let tokenizer = Tokenizer::from_pretrained("distilbert-base-cased", None).unwrap(); // .expect("hello");
    // let tokenizer = Tokenizer::from_pretrained("xlm-roberta-base", None).expect("hello");

    // RUST_TOKENIZER_MAP.insert(RUST_TOKENIZER_MAP.len() as u32, tokenizer);


    let words = ["EU", "rejects", "German", "call", "to", "boycott", "British", "lamb", "."];
    let encoding = tokenizer.encode(&words[..], true).unwrap(); // .expect("nothing");
    let token_ids = encoding.get_ids();
    let word_id_opts = encoding.get_word_ids();
    let tokens = encoding.get_tokens();
    // let tokens = tokenizer.decode(&token_ids[..], false);

    for token_id in token_ids {
        print!("{} ", token_id);
    }
    println!();

    let word_id_ints = word_id_opts
        .iter()
        .map(|&word_id_opt| {
            if word_id_opt.is_some() {
                word_id_opt.unwrap() as i32
           } else {
                -1
           }
        })
        .collect::<Vec<_>>();

    for word_id_int in word_id_ints {
        print!("{} ", word_id_int);
    }
    println!();
    
    for word_id_opt in word_id_opts {
        let word_id_str = if word_id_opt.is_some() {
            format!("Some({})", word_id_opt.unwrap())
        } else {
            format!("{}", "None")
        };
        print!("{} ", word_id_str);
    }
    println!();

    for token in tokens {
        print!("{} ", token);
    }
    println!();

    let tokenizer2 = new_tokenizer("distilbert-base-cased");
    let (tokens2, token_ids2, word_id_ints2) = encode(tokenizer2, &words[..]);


    for token in tokens2 {
        print!("{} ", token);
    }
    println!();

    for token_id in token_ids2 {
        print!("{} ", token_id);
    }
    println!();

    for word_id_int in word_id_ints2 {
        print!("{} ", word_id_int);
    }
    println!();


    println!("Keith was here 2");

    Ok(())
}

fn new_tokenizer(name: &str) -> Tokenizer {
    let tokenizer = Tokenizer::from_pretrained(name, None).expect("hello");
    return tokenizer
}

fn encode(tokenizer: Tokenizer, words: &[&str]) -> (Vec<String>, Vec<u32>, Vec<i32>) {
    let encoding = tokenizer.encode(words, true).unwrap();
    let tokens = encoding.get_tokens().to_vec();
    let token_id_u32s = encoding.get_ids();
    let token_id_i32s = &token_id_u32s
      .iter()
      .map(|&token_id_u32| token_id_u32 as i32)
      .collect::<Vec<_>>()[..];

    let word_id_opts = encoding.get_word_ids();
    let word_id_ints = word_id_opts
        .iter()
        .map(|&word_id_opt| {
            if word_id_opt.is_some() {
                word_id_opt.unwrap() as i32
            } else {
                -1
            }
        })
        .collect::<Vec<_>>();

    (tokens, token_ids, word_id_ints)
}
