package com.keithalcock.tokenizer.j4rs;

import org.astonbitecode.j4rs.api.Instance;
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils;

public class JavaTokenizer {
    // Take the name and create a RustTokenizer.
    // return an instance
    private static native Instance create(Instance<String> name);

    // Garbage collect the RustTokenizer.
    public static native void destroy(/*Instance rustTokenizer*/);

    // Perform tokenization on the words.
    private static native void tokenize(/*Instance rustTokenizer,*/ Instance<Integer> i /*, Instance<String[]> words*/);

    public static Integer create(String name) {
        destroy();
        // return an instance of the rust.
        Instance instance = create(Java2RustUtils.createInstance(name));
        Integer result = Java2RustUtils.getObjectCasted(instance);

        return result;
    }

    public static void tokenize(/*Instance rustTokenizer,*/ Integer i/*, String[] words*/) {
        tokenize(/*rustTokenizer,*/ Java2RustUtils.createInstance(i) /*, Java2RustUtils.createInstance(words)*/);
    }
}
