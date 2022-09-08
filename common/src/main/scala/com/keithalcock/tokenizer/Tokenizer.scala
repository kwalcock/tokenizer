package com.keithalcock.tokenizer

abstract class Tokenizer(name: String) {
  def tokenize(words: Array[String]): Array[Int]
  def untokenize(ints: Array[Int]): Array[String]
}
