# Hands-On 1
## Introduction
The following are concise descriptions of the solutions implemented for the second [assignment](https://pages.di.unipi.it/rossano/blog/2023/handson22324/) of the exam **Competitive Programming and Contests**, presented in order to better explain the reasoning behind every solution.\
In this *Introduction* we will give a brief overview of the solutions and the assumptions that were made developing the solutions, as well as references to resources that were used.

Both parts of the assignment were solved using *segment trees* as suggested within it. This data structure seemed obvious given the fact that range queries were central to both problems and given the time constraint that we were tasked to respect.

We though made some assumptions:
- Assumed usage only with numbers. Both solutions could have been written using generics and be extended to all structs with traits ``PartialOrd`` and ``Ord``.
- Used the type ``i32`` disregarding the possibility of numbers that are greater than what this data type can contain.

Having explained all this, we can now go in-depth analysing the implemented solutions.
## Min and Max


## Is There


## Conclusion
For each solution I implemented various tests, trying to summarize all possible categories of inputs and outputs.<br />
I hope the work done will suffice and again I'm sorry if I did not implement iterative solutions. The idea of asking myself which  approach was better came up too late.<br />
As requested I included all the code in a single file `lib.rs`, which can also be found on [my github](https://github.com/LoSpiri/CompetitiveProgramming/tree/main/Hands-On/hands_on_1)
