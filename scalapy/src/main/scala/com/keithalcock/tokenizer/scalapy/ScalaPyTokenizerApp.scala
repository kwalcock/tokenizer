package com.keithalcock.tokenizer.scalapy

object ScalaPyTokenizerApp extends App {
  val words = Array("EU", "rejects", "German", "call", "to", "boycott", "British", "lamb", ".")
  println(words.mkString(", "))

  val tokenizer = ScalaPyTokenizer("distilbert-base-cased")
  // val tokenizer = ScalaPyTokenizer("xlm-roberta-base")

  val (tokenIds, wordIds) = tokenizer.encode(words)
  println(tokenIds)
  println(wordIds)

  val tokens = tokenizer.decode(tokenIds)
  println(tokens)
}
