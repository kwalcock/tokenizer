use jni::JNIEnv;
use jni::objects::JClass;
use jni::objects::JString;
use jni::sys::jlong;
use jni::sys::jstring;
use std::convert::TryFrom;
use std::result::Result;
use std::io::{self, Write};
use tokenizers::tokenizer::Tokenizer;

fn create_tokenizer(name: &String) -> i64 {
    // See https://doc.rust-lang.org/std/primitive.pointer.html
    let tokenizer_stack: Tokenizer = Tokenizer::from_pretrained(name, None).unwrap();
    let tokenizer_heap: Box<Tokenizer> = Box::new(tokenizer_stack);
    let tokenizer_ref: &'static mut Tokenizer = Box::leak(tokenizer_heap);
    let tokenizer_ptr: *mut Tokenizer = tokenizer_ref;
    let tokenizer_id: i64 = tokenizer_ptr as i64;

    return tokenizer_id;
}

fn destroy_tokenizer(tokenizer_id: i64) {
    let tokenizer_ptr = tokenizer_id as *mut Tokenizer;
    // This takes ownership and will cause memory to be released.
    unsafe { Box::from_raw(tokenizer_ptr) };
    return;
}

#[no_mangle]
pub extern "system" fn Java_org_clulab_transformers_tokenizer_jni_JavaJniTokenizer_native_1create(env: JNIEnv,
        _class: JClass, j_name: JString) -> jlong {
    let r_name: String = env.get_string(j_name).unwrap().into();
    eprintln!("=> native_create(\"{}\")", r_name);

    let tokenizer_id = create_tokenizer(&r_name);
    eprintln!("<= {}", tokenizer_id);
    return tokenizer_id;
}

#[no_mangle]
pub extern "system" fn Java_org_clulab_transformers_tokenizer_jni_JavaJniTokenizer_native_1destroy(_env: JNIEnv,
        _class: JClass, tokenizer_id: jlong) {
    eprintln!("=> destroy_rust_tokenizer({})", tokenizer_id);
    
    destroy_tokenizer(tokenizer_id);
    return;
}

/*
#[call_from_java("org.clulab.transformers.tokenizer.j4rs.JavaJ4rsTokenizer.tokenize")]
fn rust_tokenizer_tokenize(tokenizer_id_instance: Instance, words_instance: Instance) -> Result<Instance, String> {
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let tokenizer_id: i64 = jvm.to_rust(tokenizer_id_instance).unwrap();
    let words: Vec<String> = jvm.to_rust(words_instance).unwrap();
    // println!("rust_tokenizer_tokenize({}, <words>)", tokenizer_id);

    let tokenizer = unsafe { &*(tokenizer_id as *const Tokenizer) };
    let encoding = tokenizer.encode(&words[..], true).unwrap();
    let token_id_u32s = encoding.get_ids();
    let token_id_i32s = unsafe { std::mem::transmute::<&[u32], &[i32]>(token_id_u32s) };
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
*/
