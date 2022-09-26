
import time

from transformers import AutoTokenizer

sentences = [
  ["EU", "rejects", "German", "call", "to", "boycott", "British", "lamb", "."],
  ["The", "Computational", "Language", "Understanding", "(", "CLU", ")", "Lab", "at", "University", "of", "Arizona", "is", "a", "team", "of", "faculty,", "students,", "and", "research", "programmers", "who", "work", "together", "to", "build", "systems", "that", "extract", "meaning", "from", "natural", "language", "texts", ",", "including", "question", "answering", "(", "answering", "natural", "language", "questions", ")", ",", "information", "extraction", "(", "extracting", "specific", "relations", "and", "events", ")", ",", "semantic", "role", "labeling", "(", "extracting", "semantic", "frames", "that", "model", "who", "did", "what", "to", "whom,", "when", "and", "where", "),", "parsing", "the", "discourse", "structure", "of", "complex", "texts,", "and", "other", "computational", "linguistics", "problems", "."],
  ["These", "systems", "were", "used", "in", "several", "applications", ",", "ranging", "from", "extracting", "cancer", "signaling", "pathways", "from", "biomedical", "articles", "to", "automated", "systems", "for", "answering", "multiple-choice", "science-exam", "questions", "."],
  ["The", "CLU", "lab", "includes", "members", "from", "the", "Computer", "Science", "department,", "the", "Linguistics", "department,", "and", "the", "School", "of", "Information", ".", "For", "more", "on", "natural", "language", "processing", "(", "NLP", ")", "work", "at", "UofA", ",", "please", "see", "our", "NLP", "cluster", "page", "."]
]
name = "distilbert-base-cased"
tokenizer = AutoTokenizer.from_pretrained(name)

def loop():
  for i in range(1, 10000):
    for words in sentences:
      token_input = tokenizer(words, is_split_into_words = True)
      token_ids = token_input['input_ids']
      word_ids = token_input.word_ids(batch_index = 0)
      tokens = token_input.tokens(batch_index = 0)
      # print(tokens)

def measure():
  start_time = time.time()
  loop()
  end_time = time.time()
  print(f"--- {end_time - start_time} seconds ---")

measure()
