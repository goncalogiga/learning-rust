# Hybrid Search

Hybrid search runs keyword search and vector search side by side and merges the two ranked lists. Keyword search catches exact terms, vector search catches paraphrases, and together they miss less than either alone.

The scores from the two systems are on different scales, so adding them directly does not work well. Reciprocal rank fusion sidesteps this by only using ranks: each document scores one divided by a constant plus its rank in each list, and the scores are summed. A constant of 60 is the usual choice.

A document that appears near the top of both lists ends up with the highest fused score, while one that only one system found can still make the cut.

Many production systems add a third stage, a reranker, which rescores the top fused candidates with a slower but more accurate model.
