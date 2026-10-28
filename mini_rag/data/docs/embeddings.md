# Embeddings

An embedding is a dense vector of floating point numbers that represents the meaning of a piece of text. Texts with similar meaning end up close together in this vector space, even when they share no words.

Embedding models are usually trained with a contrastive objective: pairs that belong together, such as a question and its answer, are pulled close, while unrelated pairs are pushed apart.

Closeness is normally measured with cosine similarity, the dot product of two vectors divided by the product of their lengths. If the vectors are normalised to unit length first, cosine similarity is just the dot product, which is cheap to compute.

Typical embedding models produce vectors with a few hundred to a few thousand dimensions. Larger vectors can capture more nuance but cost more memory and make search slower.
