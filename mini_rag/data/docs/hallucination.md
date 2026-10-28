# Hallucination

A hallucination is a fluent, confident answer that is not supported by facts. Language models are trained to produce plausible text, and plausible is not the same as true.

Hallucinations are most common for rare facts, precise numbers, citations, and questions about events after the training cutoff. The model often invents a believable author, paper title, or API function rather than saying it does not know.

Grounding the model in retrieved sources, asking it to quote the passage that supports each claim, and allowing it to answer that it does not know all reduce the rate of invented answers.

Detecting hallucinations automatically is hard. Common approaches compare the answer against the source documents, or sample several answers and flag claims on which the samples disagree.
