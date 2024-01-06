# Hands-On 3
## Introduction
The following are concise descriptions of the solutions implemented for the third [assignment](https://pages.di.unipi.it/rossano/blog/2023/handson32324/) of the exam **Competitive Programming and Contests**, presented to better explain the reasoning behind every solution.\
In this *Introduction*, we will give a brief overview of the solutions and the assumptions that were made in developing the solutions, as well as references to resources that were used.

## Holiday Planning
For this problem the idea of using dynamic programming was immediate, given that saving the state of previous iterations - ergo using memoization - looked as the only was possible to attain that time complexity.\
The solution follows the following steps:

- A `prefix_sums` vector is initialized to store the cumulative sums of the input values. This vector is a 2D matrix where each row represents the prefix sums of the corresponding row in the input matrix. The prefix sum is calculated for each element in a row.

- Two vectors, `optimum_vector` and `indices_vector`, are initialized to store the optimum values and the corresponding arrays containing the decisions of number of days per city made for that solution, for each day.
For each day from 1 to the total number of days the code iterates to find the optimum value for that day.

- For each local day (from 0 to the current day), the code takes one of the optimum values `local_optimum` and adds it to the best use of the remaining days, found computing the difference between the column in the `prefix_sums` corresponing to that number of days and the column calculated using the values that correspond to that optimum in the `indices_vector`.
The code then updates the `local_indices_of_max` vector to store the indices corresponing to the new max value just found.

- The code compares the global optimum value `max_value` with the sum of the local optimum value and the maximum local difference. If the sum is greater or equal, it updates the global optimum value and the corresponding indices `indices_of_max`.

- The optimum value and indices for the current day are stored in the `optimum_vector` and `indices_vector`, respectively.

- The final result is the last element in the `optimum_vector`, representing the maximum overall value achievable.

Overall, the code uses dynamic programming to iteratively calculate the optimal values for each day, considering the local optima for each possible split of transportation days. The goal is to find the combination of days that maximizes the overall value while minimizing transportation costs. The solution is built upon previous optimal solutions for smaller subsets of days.
After understaning the concept of the solution the code is pretty straightforward and easy to follow I think.
The time complexity of the solution as we can see from the loops used is `O(n * D^2)`, given `n` as the number of cities and `D` as the number of days.\
Ths space complexity is `O(n * D)`.

## Design a course
The solution for this problem doesn't use dynamic programming.\
Colleagues of mine asked for permission and the fact that no time complexity was suggested seemed like an encouragement to opt for the most efficient solution. Also the words `devise an efficient algorithm` in the assignment's text contributed.\
The solution, written in the function `max_topics` follows the following steps:
- Sort the input vector of tuples based on the first element in ascending order and, in case of ties, the second element in descending order. This is done in order to avoid errors for ties, keeping the courses chosen in strictly ascending order.

- Initialize a vector to store the longest increasing subsequence and add the first element of the sorted input as the initial element of the subsequence.

- Iterate over the sorted input, starting from the second element. For each element, check if its second attribute is greater than the last element's second attribute in the subsequence. If yes, append it to the subsequence. If no, perform a binary search to find the right position for insertion, update the subsequence, and maintain the sorted order.

- Print the resulting subsequence at each step to show its evolution and return the length of the longest increasing subsequence.

In summary, the code takes a vector of tuples, sorts it based on the first and second attributes, and then finds the longest increasing subsequence based on these attributes using a combination of appending elements and binary search for efficient updates.

In the code there's also another function `longest_increasing_subsequence`, that takes inspiration from [the implementation by Tushar Roy](https://www.youtube.com/watch?v=S9oUiVYEq7E). This differs in the fact that it is also able to retrace the topics that compose the longest increasing subsequence and not only output its length.\
It's been left there as it follows a slightly different approach and it's already ready and working.

## Conclusion and References
The testing has been done in ``main.rs``, parsing the files provided with the assignment and checking the resulting outputs against the one provided using ``assert_eq!``. As requested I included all the code in 2 files, `lib.rs` for the implementations and ``main.rs`` for the execution and testing. They both can also be found on [my GitHub](https://github.com/LoSpiri/CompetitiveProgramming/tree/main/Hands-On/hands_on_3).\
It's worth to mention that being the difficulty of this 2 problems way higher than that of the previous ones, they succedded not in canceling all my holiday plans but in shortening them.\
Also I appreciated having a story added to the problems, sounded very advent-of-code-ish.
Hope the work will suffice.