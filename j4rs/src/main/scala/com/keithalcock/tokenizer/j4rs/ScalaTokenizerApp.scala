package com.keithalcock.tokenizer.j4rs

object ScalaTokenizerApp extends App {
  val scalaTokenizer = ScalaTokenizer("distilbert-base-cased")
  // val scalaTokenizer = ScalaTokenizer("xlm-roberta-base")

  scalaTokenizer.encode(Seq("one", "two"))
}
