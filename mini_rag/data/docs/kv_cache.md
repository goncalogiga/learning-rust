# KV Cache

When a transformer generates text one token at a time, the keys and values computed for earlier tokens do not change. The KV cache stores them so that each new token only computes attention for itself instead of recomputing the whole sequence.

The cache makes generation dramatically faster, but it costs memory. Its size grows with the number of layers, the number of heads, the head dimension, and the sequence length, and for long contexts it can be larger than the model weights.

Techniques such as grouped-query attention share keys and values across heads to shrink the cache. Paged attention stores the cache in fixed-size blocks, like virtual memory, so a server can pack many concurrent requests together.

Prompt caching reuses the KV cache of a shared prefix, such as a long system prompt, across requests, which lowers both latency and cost.
