# Algorithms
Implementation of algorithms from "Introduction to Algorithms" by Thomas Cormen in Rust

## Complexity & Big O
- Constant Time Complexity: O(1). Algorithms with constant time complexity always take the same amount of time to run, regardless of the input size. They have a constant number of operations.
- Logarithmic Time Complexity: O(log n). Algorithms with logarithmic time complexity have running times that grow logarithmically with the input size. These algorithms often divide the problem in half at each step.
- Linear Time Complexity: O(n). Algorithms with linear time complexity have running times that grow linearly with the input size. Each additional element in the input requires an additional operation.
- Linearithmic Time Complexity: O(n log n). Algorithms with linearithmic time complexity have running times that grow in a combination of linear and logarithmic rates. They often arise in divide-and-conquer algorithms like merge sort and quicksort.
- Quadratic Time Complexity: O(n^2). Algorithms with quadratic time complexity have running times that grow quadratically with the input size. They typically involve nested loops or exhaustive comparisons.
0 Exponential Time Complexity: O(k^n), where k > 1. Algorithms with exponential time complexity have running times that grow exponentially with the input size. They become highly inefficient as the input size increases.