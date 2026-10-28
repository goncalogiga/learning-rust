# Evaluation

You cannot improve what you do not measure. An evaluation set is a collection of inputs with known good outputs, used to score a system every time you change it.

For retrieval, the usual metrics are recall at k and mean reciprocal rank. Recall at k asks: is a relevant document somewhere in the top k results? Mean reciprocal rank rewards putting the relevant document first: a hit at rank one scores 1, at rank two scores 0.5, at rank three scores a third, averaged over all queries.

For generated text, exact match rarely works, so teams use rubric-based grading, pairwise comparison, or a strong model acting as a judge. Judges must themselves be checked against human labels.

A small, carefully labelled evaluation set that you run on every change is worth more than a large one you never look at.
