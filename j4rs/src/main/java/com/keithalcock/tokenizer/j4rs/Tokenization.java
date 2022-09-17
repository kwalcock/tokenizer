package com.keithalcock.tokenizer.j4rs;

public class Tokenization {
//    private int tokenIds[];
//    private int wordIds[];
//    private String tokens[];

    private Integer tokenId;

    /*
    public Tokenization(int tokenIds[], int wordIds[], String tokens[]) {
        this.tokenIds = tokenIds;
        this.wordIds = wordIds;
        this.tokens = tokens;
    }
     */

    public Tokenization(Integer tokenId) {
        this.tokenId = tokenId;
    }

    public Integer getTokenId() {
        return this.tokenId;
    }

    public void setTokenId(Integer tokenId) {
        this.tokenId = tokenId;
    }

//    public int[] getTokenIds() {
//        return this.tokenIds;
//    }

//    public void setTokenIds(int tokenIds[]) {
//        this.tokenIds = tokenIds;
//    }

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
