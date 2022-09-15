package com.keithalcock.tokenizer.j4rs

import io.github.astonbitecode.j4rs.example.RustSimpleFunctionCall

object ScalaTokenizerApp extends App {

//  val rustFnCalls = new RustSimpleFunctionCall();
//  rustFnCalls.doCallNoArgs();

  val scalaTokenizer = ScalaTokenizer("distilbert-base-cased")
  // val scalaTokenizer = ScalaTokenizer("xlm-roberta-base")

  scalaTokenizer.encode(Seq("one", "two"))
}
