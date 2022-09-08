
from transformers import AutoTokenizer

tokenizer = AutoTokenizer.from_pretrained("distilbert-base-cased")
# tokenizer = AutoTokenizer.from_pretrained("xlm-roberta-base")

words = ["EU", "rejects", "German", "call", "to", "boycott", "British", "lamb", "."]

token_input = tokenizer(words, is_split_into_words = True)
print(token_input)

token_ids = token_input['input_ids']
print(token_ids)
word_ids = token_input.word_ids(batch_index = 0)
print(word_ids)

tokens = tokenizer.convert_ids_to_tokens(token_ids)
print(tokens)

class Tokenizer():
  def __init__(self, name: str):
    self.tokenizer = AutoTokenizer.from_pretrained(name)

  def encode(self, words: list[str]) -> tuple[list[int], list[int]]:
    token_input = self.tokenizer(words, is_split_into_words = True)
    token_ids = token_input['input_ids']
    word_ids = token_input.word_ids(batch_index = 0)
    return token_ids, word_ids

  def decode(self, token_ids: list[int]) -> list[str]:
    return self.tokenizer.convert_ids_to_tokens(token_ids)

tokenizer = Tokenizer("distilbert-base-cased")
# tokenizer = Tokenizer("xlm-roberta-base")

words = ["EU", "rejects", "German", "call", "to", "boycott", "British", "lamb", "."]

token_ids, word_ids = tokenizer.encode(words)
print(token_ids)
print(word_ids)

tokens = tokenizer.decode(token_ids)
print(tokens)
