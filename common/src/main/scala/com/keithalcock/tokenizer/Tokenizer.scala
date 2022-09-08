package com.keithalcock.tokenizer

abstract class Tokenizer(name: String) {
  // This returns (tokenIds, wordIds)
  def encode(words: Seq[String]): (IndexedSeq[Int], IndexedSeq[Int])
  // This returns words
  def decode(tokenIds: Seq[Int]): IndexedSeq[String]
}
