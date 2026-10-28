# Vector Databases

A vector database stores embeddings and answers nearest-neighbour queries: given a query vector, return the stored vectors closest to it.

Comparing the query against every stored vector, called brute-force or exact search, is simple and perfectly fine for a few hundred thousand vectors. Beyond that, approximate nearest neighbour indexes trade a little accuracy for a lot of speed.

HNSW, the most popular index, builds a layered graph in which each vector links to its neighbours, and searches by greedily walking the graph. IVF indexes cluster the vectors and only search the clusters closest to the query.

Most vector databases also store metadata next to each vector, so you can filter results, for example to documents a user is allowed to see, and many combine vector search with keyword search.
