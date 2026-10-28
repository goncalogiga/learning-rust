# Fine-Tuning

Fine-tuning continues training a pretrained model on a smaller, task-specific dataset. It changes the model's weights, which makes it good at teaching a consistent style, format, or narrow skill.

Full fine-tuning updates every weight and needs a lot of GPU memory. Parameter-efficient methods such as LoRA freeze the original weights and train small low-rank matrices that are added to them. A LoRA adapter can be a few megabytes for a model that is many gigabytes.

Fine-tuning is a poor way to add facts. The model may memorise some of them but will still hallucinate around the edges, and the knowledge is frozen at training time. For facts that change, retrieval is usually the better tool.

A common recipe is to collect a few thousand high-quality examples, hold some out for evaluation, and stop training before the model starts to overfit.
