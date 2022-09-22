package com.keithalcock.tokenizer.j4rs

object ScalaTokenizerApp extends App {
  val words = Array("EU", "rejects", "German", "call", "to", "boycott", "British", "lamb", ".")
  val names = Array(
    "distilbert-base-cased",
    "xlm-roberta-base"
  )

  names.foreach { name =>
    val scalaTokenizer = ScalaTokenizer(name)
    val tokenization = scalaTokenizer.encode(words)
  }
}
