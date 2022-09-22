package org.clulab.transformers.tokenizer.j4rs;

import org.astonbitecode.j4rs.api.Instance;
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils;

public class JavaJ4rsTokenizer {
    // Take the name and create a RustTokenizer.
    // return an instance
    private static native Instance create(Instance<String> name);

    // Garbage collect the RustTokenizer.
    public static native void destroy(Instance<Integer> rustTokenizerId);

    // Perform tokenization on the words.
    private static native Instance tokenize(Instance<Integer> rustTokenizerId, Instance<String[]> words);

    public static int create(String name) {
        Instance name_instance = Java2RustUtils.createInstance(name);
        Instance tokenizer_id_instance = create(name_instance);
        int tokenizer_id = Java2RustUtils.getObjectCasted(tokenizer_id_instance);

        return tokenizer_id;
    }

    public static void destroy(int tokenizer_id) {
        Instance tokenizer_id_instance = Java2RustUtils.createInstance(tokenizer_id);
        destroy(tokenizer_id_instance);
    };

    public static JavaJ4rsTokenization tokenize(int tokenizer_id, String[] words) {
        Instance tokenizer_id_instance = Java2RustUtils.createInstance(tokenizer_id);
        Instance words_instance = Java2RustUtils.createInstance(words);
        Instance tokenization_instance = tokenize(tokenizer_id_instance, words_instance);
        JavaJ4rsTokenization tokenization = Java2RustUtils.getObjectCasted(tokenization_instance);

        return tokenization;
    }
}
