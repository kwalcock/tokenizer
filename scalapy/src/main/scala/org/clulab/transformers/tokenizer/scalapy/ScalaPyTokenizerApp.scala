package org.clulab.transformers.tokenizer.scalapy

object ScalaPyTokenizerApp extends App {
  val words = Array("EU", "rejects", "German", "call", "to", "boycott", "British", "lamb", ".")
  val names = Array(
    "distilbert-base-cased",
    "xlm-roberta-base"
  )

  println(s"words: ${words.mkString(" ")}")
  names.foreach { name =>
    val tokenizer = ScalaPyTokenizer(name)
    val tokenization = tokenizer.tokenize(words)

    println(tokenization)
  }
}
