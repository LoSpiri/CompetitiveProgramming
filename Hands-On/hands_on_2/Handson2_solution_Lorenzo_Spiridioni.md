# Hands-On 1
## Introduction
The following are concise descriptions of the solutions implemented for the second [assignment](https://pages.di.unipi.it/rossano/blog/2023/handson22324/) of the exam **Competitive Programming and Contests, presented to better explain the reasoning behind every solution.\
In this *Introduction*, we will give a brief overview of the solutions and the assumptions that were made in developing the solutions, as well as references to resources that were used.

Both parts of the assignment were solved using *segment trees* as suggested within it. This data structure seemed obvious given the fact that range queries were central to both problems and given the time constraint that we were tasked to respect.

We made some assumptions:
- Assumed usage only with numbers. Both solutions could have been written using generics and be extended to all structs with traits ``PartialOrd`` and ``Ord``.
- Assumed only reasonable queries. In the test, there's no delving into what would happen to the execution if given absurd inputs. This would probably only require such analysis and some consequent ifs.
- Used the type ``i32`` disregarding the possibility of numbers that are greater than what this data type can contain.

Having explained all this, we can now go in-depth analyzing the implemented solutions.
## Min and Max
The solution to this first task is pretty straightforward and it's all based on the properties of segment trees.\
Being a problem of max values, we initialize the segment tree full of ``i32::MIN`` values. We then update its values using the input array, starting by putting the array values in the leaves and updating the max values in the ranges going upwards.\
We then update the tree at each *update* query, keeping the nodes traversed all updated and traversing only the least number of nodes, stopping when a range that is fully contained in the one we are searching is found. When this happens we update the corresponding child nodes in the lazy tree, and will since then check at each traversal in the segment tree if the corresponding node in the lazy tree is not the default value, in our case ``i32::MIN``.\
At this point, for the max queries, we just have to search in the tree for the ranges that are fully contained in the one we are searching into and then return the max values among the ones found./
Keeping the lazy tree alongside the segment tree lets us update only the nodes that we need to be updated at any given point, reducing the time consumed for each query./
Because of the properties of segment trees, all update queries take time O(log n) and all max queries take time O(log n). For this reason, the total time complexity of the solution, given m update queries and n max queries is O((n+m)log n).

## Is There
This problem is very similar to the previous one, with only small adjustments required.\
Instead of starting with an array as input to build the initial segment tree from and then updating it when update queries happen, we start with a segment tree fully initialized to zero and we build it using update queries directly. This means that we treat the array of update queries given as input not in the same way as the array of the previous problem but as queries, meaning that the lazy tree is built all along.\
In practical terms, this means that we treat each segment <l,r> of the ones given as an individual query. We then increment by one the segment tree values when a node representing a range that is fully contained in the one we are searching into is found and increment by one also its child nodes, if any.\
For the isThere queries, we then simply search in the segment tree for the nodes representing ranges contained in the one we are searching into and when we find them we check if they have a value that is equal to the k we have been given as input alongside the range.\
While traversing we update the segment tree values using the corresponding lazy tree values and we stop our traversal early if a node with a value less than the key we are searching is found. This is because each node contains the max value among the values of all its children and subchildren.\
Because of the properties of segment trees, all update queries take time O(log n) and all max queries take time O(log n). For this reason, the total time complexity of the solution, given m update queries and n max queries is O((n+m)log n).

## Conclusion and References
Both solutions take great inspiration from [the segment tree implementation by Tushar Roy](https://github.com/mission-peace/interview/blob/master/src/com/interview/tree/SegmentTreeMinimumRangeQuery.java) in Java.\
The testing has been done in ``main.rs``, parsing the files provided with the assignment and checking the resulting outputs against the one provided using ``assert_eq!``.\
As requested I included all the code in 2 files, `lib.rs` for the implementations and ``main.rs`` for the execution and testing. They both can also be found on [my GitHub](https://github.com/LoSpiri/CompetitiveProgramming/tree/main/Hands-On/hands_on_2).\
Hope the work will suffice.







For each solution I implemented various tests, trying to summarize all possible categories of inputs and outputs.<br />
I hope the work done will suffice and again I'm sorry if I did not implement iterative solutions. The idea of asking myself which approach was better came up too late.<br />
As requested I included all the code in a single file `lib.rs`, which can also be found on [my GitHub](https://github.com/LoSpiri/CompetitiveProgramming/tree/main/Hands-On/hands_on_1)
