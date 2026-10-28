# Reinforcement Learning from Human Feedback

A pretrained model predicts text, but it does not know which answers people actually want. Reinforcement learning from human feedback, or RLHF, teaches it those preferences.

First, people compare pairs of model responses and pick the better one. A reward model is trained on these comparisons to predict which response a person would prefer.

Then the language model is optimised to maximise that reward, usually with an algorithm such as PPO, while a penalty keeps it from drifting too far from the original model.

Direct preference optimisation, or DPO, skips the separate reward model and trains directly on the preference pairs. It is simpler and more stable, which has made it a popular alternative.
