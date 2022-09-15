use std::convert::TryFrom;
use std::result::Result;

use j4rs::InvocationArg;
use j4rs::prelude::*;
use j4rs_derive::*;
use serde::Deserialize;

#[call_from_java("com.keithalcock.tokenizer.j4rs.JavaTokenizer.create")]
fn create_rust_tokenizer(j_name: Instance) {
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let r_name: String = jvm.to_rust(j_name).unwrap();
    println!("Hello from the Rust world!");
    println!("{}", r_name);
    return; // return a rust_tokenizer, or at least a pointer?
}

#[call_from_java("com.keithalcock.tokenizer.j4rs.JavaTokenizer.destroy")]
fn destroy_rust_tokenizer() { // take a rust_tokenizer
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    println!("destroy_rust_tokenizer");
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
