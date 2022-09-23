use j4rs::InvocationArg;
use j4rs::prelude::*;
use j4rs_derive::*;
use std::convert::TryFrom;
use std::result::Result;
use tokenizers::tokenizer::Tokenizer;


fn get_tokenizer_id(name_instance: Instance) -> i64 {
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let name: String = jvm.to_rust(name_instance).unwrap();
    println!("create_rust_tokenizer(\"{}\")", name);

    let x = Box::new(41);
    let static_ref: &'static mut usize = Box::leak(x);
    *static_ref += 1;
    assert_eq!(*static_ref, 42);


    // See https://doc.rust-lang.org/std/primitive.pointer.html
    let tokenizer_stack: Tokenizer = Tokenizer::from_pretrained(name, None).unwrap();
    let tokenizer_heap: Box<Tokenizer> = Box::new(tokenizer_stack);
    let tokenizer_ref: &'static mut Tokenizer = Box::leak(tokenizer_heap);
    let tokenizer_ptr: *mut Tokenizer = tokenizer_ref;
    let tokenizer_id =  tokenizer_ptr as i64;
    // It lo
    // let tokenizer_ptr = Box::into_raw(tokenizer_heap);
    // let tokenizer_ptr: *const Tokenizer = &*tokenizer_heap;
//    let tokenizer_id =  tokenizer_ptr as i64;

    println!("tokenizer_id is {}", tokenizer_id);
    let mut words = Vec::new();
    words.push(String::from("this"));
    words.push(String::from("is")); 
    words.push(String::from("a"));
    words.push(String::from("test"));
    tokenize(tokenizer_id, words);

    return tokenizer_id;
}

#[call_from_java("org.clulab.transformers.tokenizer.j4rs.JavaJ4rsTokenizer.create")]
fn create_rust_tokenizer(name_instance: Instance) -> Result<Instance, String> {
    let tokenizer_id = get_tokenizer_id(name_instance);

    println!("tokenizer_id is {}", tokenizer_id);
    let mut words = Vec::new();
    words.push(String::from("this"));
    words.push(String::from("is")); 
    words.push(String::from("a"));
    words.push(String::from("test"));
    tokenize(tokenizer_id, words);

    println!("tokenizer_id is {}", tokenizer_id);
    let mut words = Vec::new();
    words.push(String::from("this"));
    words.push(String::from("is")); 
    words.push(String::from("a"));
    words.push(String::from("test"));
    tokenize(tokenizer_id, words);

    let tokenizer_id_invocation_arg = InvocationArg::try_from(tokenizer_id).unwrap();
    let tokenizer_id_result = Instance::try_from(tokenizer_id_invocation_arg).map_err(|error| format!("{}", error));

    return tokenizer_id_result;
}

#[call_from_java("org.clulab.transformers.tokenizer.j4rs.JavaJ4rsTokenizer.destroy")]
fn destroy_rust_tokenizer(tokenizer_id_instance: Instance) {
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let tokenizer_id: i64 = jvm.to_rust(tokenizer_id_instance).unwrap();
    println!("destroy_rust_tokenizer({})", tokenizer_id);

    // This will take ownership of the pointer and drop it later.

    // unsafe {
        // drop(Box::from_raw(my_speed));
    // }

    // TODO drop something
    return;
}

fn tokenize(tokenizer_id: i64, words: Vec<String>) {
    println!("About to crash top!");
    let tokenizer = unsafe { &*(tokenizer_id as *const Tokenizer) };
    println!("Got this far top!");
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

    for token_id_i32 in token_id_i32s {
        println!("{} ", token_id_i32);
    }
    println!();

    for word_id_i32 in word_id_i32s {
        println!("{} ", word_id_i32);
    }
    println!();

    for token in tokens {
        println!("{} ", token);
    }
    println!();
}

#[call_from_java("org.clulab.transformers.tokenizer.j4rs.JavaJ4rsTokenizer.tokenize")]
fn rust_tokenizer_tokenize(tokenizer_id_instance: Instance, words_instance: Instance) -> Result<Instance, String> {
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let tokenizer_id: i64 = jvm.to_rust(tokenizer_id_instance).unwrap();
    let words: Vec<String> = jvm.to_rust(words_instance).unwrap();
    println!("rust_tokenizer_tokenize({}, <words>)", tokenizer_id);

    println!("About to crash bottom!");
    let tokenizer = unsafe { &*(tokenizer_id as *const Tokenizer) };
    println!("Got this far bottom!");
    let encoding = tokenizer.encode(&words[..], true).unwrap();
    println!("After trying to encode bottom!");
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
    let java_tokenization_instance = jvm.create_instance("org.clulab.transformers.tokenizer.j4rs.JavaJ4rsTokenization", &[
        InvocationArg::try_from(token_id_i32s).unwrap(),
        InvocationArg::try_from(word_id_i32s).unwrap(),
        InvocationArg::try_from(tokens).unwrap()
    ]).unwrap();

    return Ok(java_tokenization_instance);
}
