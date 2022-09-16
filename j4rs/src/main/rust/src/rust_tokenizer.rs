use j4rs::InvocationArg;
use j4rs::prelude::*;
use j4rs_derive::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::result::Result;
use tokenizers::tokenizer::Tokenizer;
use tokenizers::tokenizer::Result as TokenizersResult;

//static rust_tokenizer_map = HashMap::new();

// j4rs insists that a Result is always returned and it must be an Instance.
#[call_from_java("com.keithalcock.tokenizer.j4rs.JavaTokenizer.create")]
fn create_rust_tokenizer(name_instance: Instance) -> Result<Instance, String> {
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let name: String = jvm.to_rust(name_instance).unwrap();

    println!("Hello from the Rust world!");
    println!("{}", name);

    let tokenizer = Tokenizer::from_pretrained(name, None).expect("Error: Tokenizer didn't load from {}!");
    println!("The tokenizer did not crash!");

    //let result = Instance::try_from(tokenizer).map_err(|error| format!("{}", error));
    let tokenizer_id: i32 = 43;
    let tokenizer_id_invocation_arg = InvocationArg::try_from(tokenizer_id).map_err(|error| format!("{}", error)).unwrap();
    let tokenizer_id_result = Instance::try_from(tokenizer_id_invocation_arg).map_err(|error| format!("{}", error));

    return tokenizer_id_result; // Ok is type of result
}

#[call_from_java("com.keithalcock.tokenizer.j4rs.JavaTokenizer.destroy")]
fn destroy_rust_tokenizer() { // take a rust_tokenizer
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    println!("destroy_rust_tokenizer");
    let tokenizer = Tokenizer::from_pretrained("distilbert-base-cased", None).expect("tokenizer didn't load!");
    return;
}

#[call_from_java("com.keithalcock.tokenizer.j4rs.JavaTokenizer.tokenize")]
fn rust_tokenizer_tokenize(tokenizer_id_instance: Instance, words_instance: Instance) {
// take a rust_tokenizer and words, return tuple of arrays
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let tokenizer_id: i32 = jvm.to_rust(tokenizer_id_instance).unwrap();
    let words: Vec<String> = jvm.to_rust(words_instance).unwrap();

    println!("rust_tokenizer_tokenize!");
    println!("{}", tokenizer_id);

    println!("Got here");
    for word in words {
        print!("{} ", word);
    }
    println!("No more words");
    return;
}
