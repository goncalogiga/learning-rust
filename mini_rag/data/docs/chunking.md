# Chunking

Before documents can be retrieved they are split into chunks. A chunk is the unit that gets embedded, scored, and pasted into the prompt, so its size matters a great deal.

Chunks that are too small lose context: a sentence like "it was released in 2019" is useless without the paragraph around it. Chunks that are too large dilute the signal and waste the context window.

Simple strategies split on a fixed number of tokens with some overlap between neighbouring chunks. Better strategies respect the structure of the document and split on headings, paragraphs, or sentences.

Each chunk should remember which document it came from, so the system can show a citation and the user can check the source.
