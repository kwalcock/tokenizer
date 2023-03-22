
from transformers import AutoTokenizer

def procedurally():
  words = ["EU", "rejects", "German", "call", "to", "boycott", "British", "lamb", "."]
  names = [
    "distilbert-base-cased",
    "xlm-roberta-base",
    "bert-base-cased"
  ]

  for name in names:
    tokenizer = AutoTokenizer.from_pretrained(name)
    token_input = tokenizer(words, is_split_into_words = True)
    token_ids = token_input['input_ids']
    print(token_ids)
    word_ids = token_input.word_ids(batch_index = 0)
    print(word_ids)
    tokens = token_input.tokens(batch_index = 0)
    print(tokens)

class Tokenizer():
  def __init__(self, name: str):
    self.tokenizer = AutoTokenizer.from_pretrained(name)

  def tokenize(self, words: list[str]) -> tuple[list[int], list[int]]:
    token_input = self.tokenizer(words, is_split_into_words = True)
    token_ids = token_input['input_ids']
    word_ids = token_input.word_ids(batch_index = 0)
    tokens = token_input.tokens(batch_index = 0)
    return token_ids, word_ids, tokens

def objectively():
  words = ["EU", "rejects", "German", "call", "to", "boycott", "British", "lamb", "."]
  names = [
    "distilbert-base-cased",
    "xlm-roberta-base",
    "bert-base-cased"
  ]

  for name in names:
    tokenizer = Tokenizer(name) # add something here for spaces
    token_ids, word_ids, tokens = tokenizer.tokenize(words)
    print(token_ids)
    print(word_ids)
    print(tokens)

procedurally()
objectively()
