use j4rs::InvocationArg;
use j4rs::prelude::*;
use j4rs_derive::*;
use serde::Deserialize;
//use std::collections::HashMap;
use std::convert::TryFrom;
use std::result::Result;
use tokenizers::tokenizer::Tokenizer;
//use tokenizers::tokenizer::Result as TokenizersResult;

#[derive(Deserialize, Debug)]
struct Tokenization {
    token_id: i32,
    token_ids: Vec<i32> //,
//    word_ids: Vec<i32>,
//    tokens: Vec<String>
}

static global_tokenizer_id: i32 = 43;

// j4rs insists that a Result is always returned and it must be an Instance.
#[call_from_java("com.keithalcock.tokenizer.j4rs.JavaTokenizer.create")]
fn create_rust_tokenizer(name_instance: Instance) -> Result<Instance, String> {
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let name: String = jvm.to_rust(name_instance).unwrap();
    println!("create_rust_tokenizer({})", name);

    let _tokenizer = Tokenizer::from_pretrained(name, None).expect("Error: Tokenizer didn't load from {}!");
    println!("The tokenizer did not crash!");

    //let result = Instance::try_from(tokenizer).map_err(|error| format!("{}", error));
    let tokenizer_id_invocation_arg = InvocationArg::try_from(global_tokenizer_id).map_err(|error| format!("{}", error)).unwrap();
    let tokenizer_id_result = Instance::try_from(tokenizer_id_invocation_arg).map_err(|error| format!("{}", error));

    // global_tokenizer_id += 1;
    return tokenizer_id_result; // Ok is type of result
}

#[call_from_java("com.keithalcock.tokenizer.j4rs.JavaTokenizer.destroy")]
fn destroy_rust_tokenizer(tokenizer_id_instance: Instance) {
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let tokenizer_id: i32 = jvm.to_rust(tokenizer_id_instance).unwrap();
    println!("destroy_rust_tokenizer({})", tokenizer_id);
    // TODO: perform work
    return;
}

use std::convert::TryInto;

fn demo<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

#[call_from_java("com.keithalcock.tokenizer.j4rs.JavaTokenizer.tokenize")]
fn rust_tokenizer_tokenize(tokenizer_id_instance: Instance, words_instance: Instance) -> Result<Instance, String> {
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let tokenizer_id: i32 = jvm.to_rust(tokenizer_id_instance).unwrap();
    let words: Vec<String> = jvm.to_rust(words_instance).unwrap();
    println!("rust_tokenizer_tokenize({}, <words>)", tokenizer_id);
    for word in &words {
        print!("{} ", word);
    }
    println!("");
    println!("No more words");

    let tokenizer = Tokenizer::from_pretrained("distilbert-base-cased", None).expect("tokenizer didn't load!");
    let encoding = tokenizer.encode(&words[..], true).unwrap();
    let token_id_u32s = encoding.get_ids();
    let token_id_i32s = &token_id_u32s
       .iter()
       .map(|&token_id_u32| token_id_u32 as i32)
       .collect::<Vec<_>>()[..];
    let word_id_opts = encoding.get_word_ids();
    let word_id_i32s = &word_id_opts
       .iter()
       .map(|&word_id_opt| {
           if word_id_opt.is_some() {
               word_id_opt.unwrap() as i32
           } else {
               -1
           }
       })
       .collect::<Vec<_>>()[..];
    let tokens = encoding.get_tokens();

    for token in tokens {
        print!("{} ", token);
    }
    println!();

//    let tokenization = Tokenization {
//        token_ids: token_id_i32s,
//        word_ids: word_id_i32s,
//        tokens: tokens
//    };
    let java_tokenization_instance = jvm.create_instance("com.keithalcock.tokenizer.j4rs.Tokenization", &[
        
        InvocationArg::try_from(32).map_err(|error| format!("{}", error)).unwrap(),
        InvocationArg::try_from(token_id_i32s).map_err(|error| format!("{}", error)).unwrap() //,
  //      InvocationArg::try_from(word_id_i32s).map_err(|error| format!("{}", error)).unwrap(),
    //    InvocationArg::try_from(tokens).map_err(|error| format!("{}", error)).unwrap()
    ]).map_err(|error| format!("{}", error)).unwrap(); // The above is an instance!

//    let tokenization_invocation_arg = InvocationArg::try_from(tokens).map_err(|error| format!("{}", error)).unwrap();
//    let tokenization_result = Instance::try_from(tokenization_invocation_arg).map_err(|error| format!("{}", error));
    let java_tokenization_result = Instance::try_from(java_tokenization_instance).map_err(|error| format!("{}", error));

    println!("Got here");
//    return tokenization_result;
    return java_tokenization_result;
}
