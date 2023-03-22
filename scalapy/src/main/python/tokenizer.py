
from transformers import AutoTokenizer

def procedurally():
  words = ["EU", "rejects", "German", "call", "to", "boycott", "British", "lamb", "."]
  name_and_space_tuples = [
    ("distilbert-base-cased", False),
    ("xlm-roberta-base", False),
    ("bert-base-cased", False),
    ("google/bert_uncased_L-4_H-512_A-8", False),
    ("roberta-base", True)
    # "microsoft/deberta-v3-base"
  ]

  print("Procedurally...")
  for name, space in name_and_space_tuples:
    tokenizer = AutoTokenizer.from_pretrained(name, add_prefix_space=space)
    token_input = tokenizer(words, is_split_into_words = True)
    token_ids = token_input['input_ids']
    word_ids = token_input.word_ids(batch_index = 0)
    tokens = token_input.tokens(batch_index = 0)
    print(name)
    print(token_ids)
    print(word_ids)
    print(tokens)
    print()

class Tokenizer():
  def __init__(self, name: str, add_prefix_space: bool):
    self.tokenizer = AutoTokenizer.from_pretrained(name, add_prefix_space=add_prefix_space)

  def tokenize(self, words: list[str]) -> tuple[list[int], list[int]]:
    token_input = self.tokenizer(words, is_split_into_words = True)
    token_ids = token_input['input_ids']
    word_ids = token_input.word_ids(batch_index = 0)
    tokens = token_input.tokens(batch_index = 0)
    return token_ids, word_ids, tokens

def objectively():
  words = ["EU", "rejects", "German", "call", "to", "boycott", "British", "lamb", "."]
  name_and_space_tuples = [
    ("distilbert-base-cased", False),
    ("xlm-roberta-base", False),
    ("bert-base-cased", False),
    ("google/bert_uncased_L-4_H-512_A-8", False),
    ("roberta-base", True)
    # "microsoft/deberta-v3-base"
  ]

  print("Objectively...")
  for name, space in name_and_space_tuples:
    tokenizer = Tokenizer(name, add_prefix_space=space)
    token_ids, word_ids, tokens = tokenizer.tokenize(words)
    print(name)
    print(token_ids)
    print(word_ids)
    print(tokens)
    print()

procedurally()
objectively()
