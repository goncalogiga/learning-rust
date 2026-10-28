# Quantization

Quantization stores model weights with fewer bits than the 16-bit or 32-bit floats used during training. An 8-bit or 4-bit integer version of a model needs a fraction of the memory, so a model that needed a large GPU can run on a laptop.

The simplest scheme maps each block of weights to integers using a scale factor, and multiplies back by that scale at inference time. Smarter schemes such as GPTQ and AWQ pick the rounding so that the layer's output changes as little as possible.

Quantized models are a little less accurate. At 8 bits the loss is usually negligible, at 4 bits it is noticeable on hard reasoning tasks, and below that quality drops quickly.

Activations and the KV cache can be quantized too, which mainly helps throughput when serving many requests at once.
