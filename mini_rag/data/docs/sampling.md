# Sampling and Temperature

At each step a language model outputs a probability for every token in its vocabulary. Sampling is how we pick the next token from that distribution.

Temperature rescales the logits before the softmax. A temperature below one sharpens the distribution, making the most likely token even more likely. A temperature above one flattens it and makes the output more random and creative. Temperature zero is greedy decoding: always pick the top token.

Top-k sampling keeps only the k most likely tokens. Top-p, or nucleus sampling, keeps the smallest set of tokens whose probabilities add up to p. Both cut off the long tail of unlikely tokens that would make the text incoherent.

For extraction and classification, low temperature gives repeatable answers. For brainstorming and creative writing, higher temperature gives more variety.
