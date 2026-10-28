# Retrieval-Augmented Generation

Retrieval-augmented generation, or RAG, gives a language model access to documents it was never trained on. Before answering, the system retrieves the passages most relevant to the user's question and pastes them into the prompt.

A RAG pipeline has two halves. The retrieval half splits documents into chunks, indexes them, and finds the best chunks for a query. The generation half asks the model to answer using only those chunks and, ideally, to cite them.

RAG reduces hallucination because the model can quote its sources instead of relying on memory. It also lets you update knowledge by re-indexing documents, which is far cheaper than retraining.

Most RAG failures are retrieval failures: if the right passage is not in the prompt, the model cannot use it. That is why measuring retrieval quality on its own is so valuable.
