# Tokenization

Language models do not read characters or words directly. A tokenizer first splits text into tokens, which are usually pieces of words, and maps each one to an integer id from a fixed vocabulary.

Byte-pair encoding (BPE) builds that vocabulary by starting from single bytes and repeatedly merging the most frequent adjacent pair. After tens of thousands of merges, common words become a single token while rare words are split into several subword pieces.

Tokenizers explain many odd model behaviours. Counting letters in a word is hard because the model never sees the letters, only the subword pieces. Numbers may be split in inconsistent ways, and text in languages that were rare in the training data costs more tokens per sentence.

Pricing and context limits are measured in tokens, not words. A useful rule of thumb for English is that one token is roughly three quarters of a word.
