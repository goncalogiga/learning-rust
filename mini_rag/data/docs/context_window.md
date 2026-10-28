# Context Window

The context window is the maximum number of tokens a model can consider at once, counting both the prompt and the generated output. Anything outside the window is invisible to the model.

Windows have grown from a few thousand tokens to hundreds of thousands, but a long context is not free. Latency and cost grow with the number of input tokens, and attention over very long inputs can be slow.

Models also use long contexts unevenly. Information buried in the middle of a long prompt is sometimes ignored, a problem known as lost in the middle. Putting the most important material near the start or the end of the prompt helps.

Even with a huge window, retrieving a handful of relevant chunks is often cheaper and more accurate than pasting an entire knowledge base into the prompt.
