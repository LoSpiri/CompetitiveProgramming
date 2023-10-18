# Hands-On 1
## Introduction
The following are concise descriptions of the implemented solutions, presented in order to better explain the reasoning behind every solution. But first let us give some general specifications about all the implementations.<br /><br />
All the exercises are solved using the Node and Tree implementation presented with the [assignment](https://pages.di.unipi.it/rossano/blog/2023/handson12324/), and so they come with some limitations given by it, that for the sake of this hands-on will not be tested.<br />
- We do not consider empty trees. We would have to change how the `with_root` function works.
- We do not consider adding a node where it's already present. We would have to change how the `add_node` function works.
- We do not consider adding a node to a node that is not present. We would have to change how the `add_node` function works.<br /><br />
- We do not consider adding a node with a key that is not of `u32` type. We would have to change the whole implementation.

We also make another 3 important assumptions.
- That the values in the trees don't have to necessarily be unique.
- That the input trees are binary. Because of how the exercises were worded that looked implicit.
- That the recursion limit will suffice. This is the **most important** one. We chose to implement all solutions using recursion mimicking the one for `sum` presented in class. After finishing I asked myself the question of whether or not this was a good choice and it looks like in Rust it really isn't. It is possible to change the recursion limit with the macro `#![recursion_limit = "x"]` but it's discouraged and iterative solutions are still almost always to be preferred.<br />
Writing iterative solutions would not have been hard, requiring to instantiate at the beginning of the functions a queue or a deque, and using it to traverse the tree adding and popping the nodes while iterating in a `while pop` loop.<br />
Although easy, having already all the complete solutions I chose not to rewrite all of them.

Having explained all this, we can now go in-depth analysing the implemented solutions.
## Validate Binary Search Tree
A search tree is a tree data structure used for locating specific keys from within a set. In order for a tree to function as a search tree, the key for each node must be greater than any keys in subtrees on the left, and less than any keys in subtrees on the right.<br />
In order to check that, I wrote the function `validate_bst` that implements a *pre-order traversal* and that passes as arguments in the recursive functions the minimum and the maximum value that a child node can have. This way each node, before calling the recursive functions on its children, checks whether its value is greater than the minimum value permitted and smaller that the maximum. Otherwise the function returns false and stops calling itself. This is a top-down approach.
## Validate Balanced Tree
A balanced binary tree is defined as a binary tree in which the height of the left and right subtree of any node differs by not more than 1.<br />
The function `validate_balanced` checks this implementing a *post-order traversal*. The function calls return `1+max(left, right)` meaning that each node returns the max depth found in its subtrees plus 1. However before doing that it checks if the difference between the depths returned by its subtrees is greater than 1 and in that case returns -1. This value is then used like a false flag, propagating it with an if condition by all the nodes when received from a subtree.<br />
At the end of the traversal the function checks if the number returned is equal or greater than 0 and in that case returns true, otherwise false. This is a bottom-up approach.
## Validate Max-Heap Tree
In a max-heap tree, for any given node, the value of the node is greater than or equal to the values of its children. The tree also has to be complete, meaning that it is a binary tree in which every level, except possibly the last, is completely filled, and all nodes in the last level are as far left as possible.<br />
These properties imply 2 different checks and so that's what I did. First, to check whether a node satisfies the max-heap property, I passed as an argument in all the recursive function calls the key value of the current node, meaning that all nodes would have access to the key value of their parent and could return true if the max-heap property is not satisfied.<br />
For the complete property, having already access to the total number of nodes from the Tree implementation, I passed as argument in all the recursive function calls the index that that specific node would have if in a complete tree. I noticed that, in a complete tree, all nodes must indeed have a progressive index and all indexes must be present. I also noticed that each node, in that case, would have index equal to 2 times the index of its parent plus 1 if left child and 2 if right. This way if it's not a complete tree some nodes will have an index passed as argument that is greater than the total number of nodes.<br />
If either check fails there is no point in keeping on making recursive calls and so I implemented a pre-order traversal in the function `validate_max_heap`, using a top-down approach.
## Conclusion
For each solution I implemented various tests, trying to summarize all possible categories of inputs and outputs.<br />
I hope the work done will suffice and again I'm sorry if I did not implement iterative solutions. The idea of asking myself which  approach was better came up too late.<br />
As requested I included all the code in a single file `lib.rs`, which can also be found on [my github](https://github.com/LoSpiri/CompetitiveProgramming/tree/main/Hands-On/hands_on_1)
