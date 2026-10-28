# Attention

Attention lets every token in a sequence look at every other token and decide how much each one matters. For each position the model computes a query vector, and for every position a key vector and a value vector. The dot product between a query and a key says how relevant that key's token is.

The scores are divided by the square root of the key dimension and passed through a softmax, which turns them into weights that sum to one. The output for a position is the weighted sum of the value vectors. This is called scaled dot-product attention.

Multi-head attention runs several of these in parallel, each with its own learned projections. One head might track syntax while another tracks which noun a pronoun refers to. The heads are concatenated and projected back to the model dimension.

Because every token attends to every other token, the cost grows quadratically with sequence length. That quadratic cost is the main reason long contexts are expensive.
