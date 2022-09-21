package com.keithalcock.tokenizer.j4rs;

import java.util.AbstractList;
import java.util.ArrayList;

public class Tokenization {
    private ArrayList<Integer> tokenIds;
    private ArrayList<Integer> wordIds;
    private ArrayList<String> tokens;

    public Tokenization(AbstractList<Integer> tokenIds, AbstractList<Integer> wordIds, AbstractList<String> tokens) {
        this.tokenIds = new ArrayList<Integer>(tokenIds); // tokenIds.toArray(new Integer[tokenIds.size()]);
        this.wordIds = new ArrayList<Integer>(wordIds);
        this.tokens = new ArrayList<String>(tokens);
    }

    public ArrayList<Integer> getTokenIds() {
        return this.tokenIds;
    }

    public void setTokenIds(ArrayList<Integer> tokenIds) {
        this.tokenIds = tokenIds;
    }

    public ArrayList<Integer> getWordIds() {
        return this.wordIds;
    }

    public void setWordIds(ArrayList<Integer> wordIds) {
        this.wordIds = wordIds;
    }

    public ArrayList<String> getTokens() {
        return this.tokens;
    }

    public void setTokens(ArrayList<String> tokens) {
        this.tokens = tokens;
    }
}
