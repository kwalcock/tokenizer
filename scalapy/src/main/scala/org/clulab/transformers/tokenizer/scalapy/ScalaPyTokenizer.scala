package org.clulab.transformers.tokenizer.scalapy

import me.shadaj.scalapy.interpreter.CPythonAPI
import me.shadaj.scalapy.interpreter.Platform
import me.shadaj.scalapy.py
import me.shadaj.scalapy.py.SeqConverters
import me.shadaj.scalapy.readwrite.Reader
import org.clulab.transformers.tokenizer.Tokenization
import org.clulab.transformers.tokenizer.Tokenizer
import ScalaPyTokenizer._

class ScalaPyTokenizer(name: String) extends Tokenizer(name) {
  val tokenizer = transformers.AutoTokenizer.from_pretrained(name)

  override def tokenize(words: Array[String]): Tokenization = {
    val tokenInput = tokenizer(
      /*text=*/ pyList(words.toPythonProxy),
      /*text_pair=*/ pyNone, // None
      /*add_special_tokens=*/ pyTrue, // True
      /*padding=*/ pyFalse, // False
      /*truncation=*/ pyFalse, // False
      /*max_length=*/ pyNone, // None
      /*stride=*/ 0, // 0
      /*is_split_into_words=*/ pyTrue // False
    )
    val tokenIds = tokenInput.bracketAccess("input_ids").as[IndexedSeq[Int]].toArray
    val wordIds = {
      implicit val reader: Reader[Int] = posIntReader

      tokenInput.word_ids(/*batch_index=*/ 0 /*=0*/).as[IndexedSeq[Int]].toArray
    }
    val tokens = tokenInput.tokens(/*batch_index=*/ 0 /*=0*/).as[IndexedSeq[String]].toArray

    Tokenization(tokenIds, wordIds, tokens)
  }
}

object ScalaPyTokenizer {

  class IntOrNoneReader(none: Int = -1) extends Reader[Int] {

    def getAndClearError(): Boolean = {
      val errorOccurred = Platform.pointerToLong(CPythonAPI.PyErr_Occurred()) != 0

      if (errorOccurred)
        CPythonAPI.PyErr_Clear()
      errorOccurred
    }

    override def readNative(r: Platform.Pointer): Int = {
      val res = CPythonAPI.PyLong_AsLongLong(r)

      if (res == -1) {
        getAndClearError()
        none
      }
      else
        res.toInt
    }
  }

  val pyList = py.Dynamic.global.list
  val pyNone = py.Dynamic.global.None
  val pyTrue = py.Dynamic.global.True
  val pyFalse = py.Dynamic.global.False
  val posIntReader: Reader[Int] = new IntOrNoneReader()
  val transformers = py.module("transformers")

  def apply(name: String): ScalaPyTokenizer = new ScalaPyTokenizer(name)
}
