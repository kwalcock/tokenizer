package com.keithalcock.tokenizer

abstract class Tokenizer(name: String) {
  def encode(words: Array[String]): (Array[Int], Array[Int])
  def untokenize(ints: Array[Int]): Array[String]
}
