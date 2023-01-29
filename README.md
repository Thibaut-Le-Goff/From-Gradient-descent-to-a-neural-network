<center><h2><ins>From gradient-descent to a neural network</ins></h2></center>

This project is the rest of the previous one, [here](https://github.com/Thibaut-Le-Goff/gradient-descent-for-Runst).

The goals of this project are:
- to see what the gradient descent algorithm would look like if this was a neural network;
- to implement the algorithm in my main project, [here](https://github.com/Thibaut-Le-Goff/Runst).

<ins>As a Neural Network</ins>\
As mentioned in Runst, the propagation works by multiplying a matrix of weights by a vector of outputs from the neurons at layer $N - 1$:

<p align="center">
    <img src="images/nn4.png" width="450"/>
</p>

But I didn't mention the addition of the bias between the multiplication and the activation functions.

This is important because the formula of the value stored in a neuron before the activation function is:

$$(\sum_{i=1}^{\textrm{\color{red}nb neurons N - 1}}\textrm{\color{green} Weight}_i * \textrm{\color{red} neuron N - 1}_i) + \textrm{\color{blue} Bias}$$


<ins>Propagation and gradient-descent</ins>\
I need to know what to change to the gradient descent algorithm to implement it in the project.

And these changes are:
|Before the implementation|After the implementation|
|-|-|
|||
|||

<ins>hypothesis:</ins>\
With respect to the sum in the neuron at layer $N$, we want to calculate what should be the $\textrm{\color{red}weight W1}$, the $\textrm{\color{blue}bias}$, and the sum the neuron at layer $N - 1$ receive (to create a loop):

<p align="center">
    <img src="images/hypothesis_layers_N_N-1.png" width="450"/>
</p>

The problem is:
If the neuron at layer $N$ tells to both neurons at layer $N - 1$ he expects $\sum$, each of them will want to give it $\sum$ and it will get $2 * \sum$ instead of $\sum$.

Maybe this problem could be solved if the neuron at layer N asks $\sum / 2$ to both neurons at layer $N - 1$.
