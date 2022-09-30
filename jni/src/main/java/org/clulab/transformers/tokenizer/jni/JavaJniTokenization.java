package org.clulab.transformers.tokenizer.jni;

import java.util.AbstractList;

public class JavaJniTokenization {
    public int tokenIds[];
    public int wordIds[];
    public String tokens[];

    public JavaJniTokenization(AbstractList<Integer> tokenIds, AbstractList<Integer> wordIds, AbstractList<String> tokens) {
        this.tokenIds = tokenIds.stream().mapToInt(i -> i).toArray();
        this.wordIds = wordIds.stream().mapToInt(i -> i).toArray();
        this.tokens = (String[]) tokens.toArray();
    }
}
