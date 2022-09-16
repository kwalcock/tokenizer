package com.keithalcock.tokenizer.j4rs

import com.keithalcock.tokenizer.Tokenizer
import org.clulab.j4rs.LibraryLoader

class ScalaTokenizer(name: String) extends Tokenizer(name) {
  val tokenizerId = JavaTokenizer.create(name)

  override def finalize: Unit = {
    JavaTokenizer.destroy(tokenizerId)
  }

  override def encode(words: Seq[String]): (IndexedSeq[Int], IndexedSeq[Int]) = {
    JavaTokenizer.tokenize(tokenizerId, words.toArray)
    null
  }

  override def decode(tokenIds: Seq[Int]): IndexedSeq[String] = ???
}

object ScalaTokenizer {
  val rustLibrary = LibraryLoader.load("rust_tokenizer")

  def apply(name: String): ScalaTokenizer = new ScalaTokenizer(name)
}
