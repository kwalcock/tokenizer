use j4rs::InvocationArg;
use j4rs::prelude::*;
use j4rs_derive::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::result::Result;
use std::sync::Mutex;
use tokenizers::tokenizer::Tokenizer;

// TODO put these together somehow
static mut TOKENIZER_COUNT: i32 = 0;
lazy_static! {
    static ref TOKENIZER_MAP: Mutex<HashMap<i32, Tokenizer>> = {
        let map: HashMap<i32, Tokenizer> = HashMap::new();
        Mutex::new(map)
    };
}

// j4rs insists that a Result is always returned and it must be an Instance.
#[call_from_java("com.keithalcock.tokenizer.j4rs.JavaTokenizer.create")]
fn create_rust_tokenizer(name_instance: Instance) -> Result<Instance, String> {
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let name: String = jvm.to_rust(name_instance).unwrap();
    println!("create_rust_tokenizer({})", name);

    let tokenizer = Tokenizer::from_pretrained(name, None).unwrap();
    let tokenizer_id = unsafe {
        TOKENIZER_MAP.lock().unwrap().insert(TOKENIZER_COUNT, tokenizer);
        let tokenizer_id = TOKENIZER_COUNT;
        TOKENIZER_COUNT += 1;
        tokenizer_id
    };
    let tokenizer_id_invocation_arg = InvocationArg::try_from(tokenizer_id).unwrap();
    let tokenizer_id_result = Instance::try_from(tokenizer_id_invocation_arg).map_err(|error| format!("{}", error));

    return tokenizer_id_result;
}

#[call_from_java("com.keithalcock.tokenizer.j4rs.JavaTokenizer.destroy")]
fn destroy_rust_tokenizer(tokenizer_id_instance: Instance) {
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let tokenizer_id: i32 = jvm.to_rust(tokenizer_id_instance).unwrap();
    println!("destroy_rust_tokenizer({})", tokenizer_id);
    TOKENIZER_MAP.lock().unwrap().remove(&tokenizer_id);
    return;
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

    let encoding = {
        let tokenizer_mutex = TOKENIZER_MAP.lock().unwrap();
        let tokenizer= tokenizer_mutex.get(&tokenizer_id).unwrap();

        // TODO: It looks like there is still a mutex on, which is a problem.
        tokenizer.encode(&words[..], true).unwrap()
    };
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

    let java_tokenization_instance = jvm.create_instance("com.keithalcock.tokenizer.j4rs.Tokenization", &[
        InvocationArg::try_from(token_id_i32s).unwrap(),
        InvocationArg::try_from(word_id_i32s).unwrap(),
        InvocationArg::try_from(tokens).unwrap()
    ]).unwrap();
    let java_tokenization_result = Instance::try_from(java_tokenization_instance).map_err(|error| format!("{}", error));

    return java_tokenization_result;
}
