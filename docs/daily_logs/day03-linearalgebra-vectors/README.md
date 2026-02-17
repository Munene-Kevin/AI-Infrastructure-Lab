..Linear algebra is the grammar of AI.
..AI models do not think; they transform vectors.

..A Vector is just an ordered list of numbers.
..In AI, they represent structured numerical data(embeddings), that encode the meaning, features, and relationships of various data types.
..Vectors are used to convert raw, unstructured information into a format that machine learning models can process, compare, and analyze.
..Vectors in AI represent::
    1. data representations(embeddings);
        * text & words: vectors represent semantic meaning in Natural Language Processing(NPL), allowing models to understand that "king" and "queen" are related.
        * images: images are represented by pixel intensities or feature descriptors eg; shapes, color, texture.
        * audio/voice: sounds can be converted into vectors for voice recognition and classification.
        * structured data: vectors can represent tabular data such as a house's price, size, and number of bedrooms.
        * user/product features: in recommendation systems, vectors represent user preferences like browsing history and item attributes.
    2. conceptual representation;
        * semantic meaning: embeddings map data into a high-dimensional space where 'semantically similar' items are closer to each other.
        * contextual relationship: intransformer models eg; GPT, vectors change based on the surrounding text, representing the context-aware meaning of a word.
        * features/attributes: each dimension in a vector can represent a specific, often abstract feature.
    3. model & algorithmic components;
        * neural network weights: during training, vectors and matrices store the learned parameters and biases of a model.
        * latent space: in generative models like; GANs or Autoencoders, vectors represent latent variables - a compact, compressed, and learned representation of the input data.
        * inputs/outputs: vectors act as input data for models and the resulting output eg; predicted classifications.


Key characteristics:
    1. dimensionality;
        ..vectors can have hundreds or thousands of dimensions, each capturing a nuance of the data.
    2. numerical vectorization;
        ..the process converts non-numeric data into a list of numbers.
    3. operations;
        ..vectors allow for mathematical operations like cosine similarity for comparing meaning and vector addition/subtraction for finding relationships.