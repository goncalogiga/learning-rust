# Mixture of Experts

A mixture-of-experts layer replaces one large feed-forward network with many smaller ones, called experts, plus a router. For each token the router picks a few experts, often two, and only those run.

This decouples the total number of parameters from the compute spent per token. A model can hold an enormous amount of knowledge across its experts while each token only pays for a small slice of it.

Training is trickier than for a dense model. If the router keeps choosing the same experts, the others never learn, so training adds a load-balancing loss that encourages an even spread.

At serving time all experts must still fit in memory, so mixture-of-experts models are cheap in compute but expensive in memory.
