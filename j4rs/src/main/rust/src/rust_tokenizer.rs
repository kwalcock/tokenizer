use j4rs::InvocationArg;
use j4rs::prelude::*;
use j4rs_derive::*;
use serde::Deserialize;
use std::convert::TryFrom;
use std::result::Result;
use tokenizers::tokenizer::Tokenizer;
use tokenizers::tokenizer::Result as TokenizersResult;

// j4rs insists that a Result is always returned and it must be an Instance.
#[call_from_java("com.keithalcock.tokenizer.j4rs.JavaTokenizer.create")]
fn create_rust_tokenizer(j_name: Instance) -> Result<Instance, String> {
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let r_name: String = jvm.to_rust(j_name).unwrap();

    println!("Hello from the Rust world!");
    println!("{}", r_name);

    let tokenizer = Tokenizer::from_pretrained(r_name, None).expect("Error: Tokenizer didn't load from {}!");
    println!("The tokenizer did not crash!");

    let ia = InvocationArg::try_from(43).map_err(|error| format!("{}", error)).unwrap();
    let result = Instance::try_from(ia).map_err(|error| format!("{}", error));

    return result; // Ok is type of result
}

#[call_from_java("com.keithalcock.tokenizer.j4rs.JavaTokenizer.destroy")]
fn destroy_rust_tokenizer() { // take a rust_tokenizer
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    println!("destroy_rust_tokenizer");
    let tokenizer = Tokenizer::from_pretrained("distilbert-base-cased", None).expect("tokenizer didn't load!");
    return;
}

#[call_from_java("com.keithalcock.tokenizer.j4rs.JavaTokenizer.tokenize")]
fn rust_tokenizer_tokenize(j_int: Instance/*, j_words: Instance*/) { // take a rust_tokenizer and words, return tuple of arrays
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let r_int: i32 = jvm.to_rust(j_int).unwrap();
    println!("Got here");
//    let r_words = jvm.to_rust(j_words).unwrap();
//    println!("{}", r_words);
    println!("rust_tokenizer_tokenize!");
    println!("{}", r_int);
    return;
}
