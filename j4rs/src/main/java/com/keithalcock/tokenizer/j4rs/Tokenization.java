package com.keithalcock.tokenizer.j4rs;

import java.util.AbstractList;
import java.util.ArrayList;

public class Tokenization {
    private Integer tokenId;
    private ArrayList tokenIds;
//    private int wordIds[];
//    private String tokens[];

    public Tokenization(Integer tokenId, AbstractList<Integer> tokenIds) { // }, int wordIds[], String tokens[]) {
        this.tokenId = tokenId;
        this.tokenIds = new ArrayList(tokenIds); // tokenIds; // tokenIds.toArray(new Integer[tokenIds.size()]);
//        this.wordIds = wordIds;
//        this.tokens = tokens;
    }

    public Integer getTokenId() {
        return this.tokenId;
    }

    public void setTokenId(Integer tokenId) {
        this.tokenId = tokenId;
    }

    public ArrayList<Integer> getTokenIds() {
        return this.tokenIds;
    }

    public void setTokenIds(ArrayList<Integer> tokenIds) {
        this.tokenIds = tokenIds;
    }

//    public int[] getWordIds() {
//        return this.wordIds;
//    }

//    public void setWordIds(int wordIds[]) {
//        this.wordIds = wordIds;
//    }

//    public String[] getTokens() {
//        return this.tokens;
//    }

//    public void setTokens(String tokens[]) {
//        this.tokens = tokens;
//    }
}
