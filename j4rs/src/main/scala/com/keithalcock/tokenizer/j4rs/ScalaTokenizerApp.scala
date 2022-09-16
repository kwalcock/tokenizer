package com.keithalcock.tokenizer.j4rs

object ScalaTokenizerApp extends App {
  val words = Array("EU", "rejects", "German", "call", "to", "boycott", "British", "lamb", ".")
  val scalaTokenizer = ScalaTokenizer("distilbert-base-cased")
  // val scalaTokenizer = ScalaTokenizer("xlm-roberta-base")

  scalaTokenizer.encode(words)
}
