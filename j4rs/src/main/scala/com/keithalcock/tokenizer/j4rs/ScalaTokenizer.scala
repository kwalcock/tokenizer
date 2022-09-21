package com.keithalcock.tokenizer.j4rs

import com.keithalcock.tokenizer.Tokenizer
import org.clulab.j4rs.LibraryLoader

import scala.collection.mutable.{HashMap => MutableHashMap}
import scala.ref.WeakReference

class ScalaTokenizer(name: String) extends Tokenizer(name) {
  val tokenizerId = JavaTokenizer.create(name)

  override def finalize: Unit = {
    JavaTokenizer.destroy(tokenizerId)
  }

  override def encode(words: Seq[String]): (IndexedSeq[Int], IndexedSeq[Int]) = {
    val tokenization = JavaTokenizer.tokenize(tokenizerId, words.toArray)

    tokenization.getTokenIds.forEach { id =>
      println(id)
    }
    null
  }

  override def decode(tokenIds: Seq[Int]): IndexedSeq[String] = ???
}

object ScalaTokenizer {
  val rustLibrary = LibraryLoader.load("rust_tokenizer")
  val map = new MutableHashMap[String, WeakReference[ScalaTokenizer]]()

  def apply(name: String): ScalaTokenizer = synchronized {
    // If the key is known and the weak reference is valid, then the result is
    // Some(scalaTokenizer) with a strong reference that will remain valid.
    val scalaTokenizerOpt = map.get(name).flatMap(_.get)

    if (scalaTokenizerOpt.isDefined)
      scalaTokenizerOpt.get
    else {
      val scalaTokenizer = new ScalaTokenizer(name)
      map(name) = WeakReference(scalaTokenizer)
      scalaTokenizer
    }
  }
}
