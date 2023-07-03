# Algorithms
Implementation of algorithms from "The Algorithm Design Manual" by Steven Skiena in Rust

## Complexity & Big O
- Constant functions, f(n) = 1: There is no dependence on the parameter n.
- Logarithmic functions, f (n) = log n: Logarithmic time complexity shows up in algorithms such as binary search. Such functions grow quite slowly as n gets big, but faster than the constant function (which is standing still, after all).
- Linear functions, f(n) = n: Such functions measure the cost of looking at each item once (or twice, or ten times) in an n-element array, say to identify the biggest item, the smallest item, or compute the average value.
- Superlinear functions, f(n) = nlgn: This important class of functions arises in such algorithms as quicksort and mergesort. They grow just a little faster than linear, but enough so to rise to a higher dominance class.
- Quadratic functions, f(n) = n^2: Such functions measure the cost of looking at most or all pairs of items in an n-element universe. These arise in algorithms such as insertion sort and selection sort.
- Cubic functions, f(n) = n^3: Such functions enumerate all triples of items in an n-element universe. These also arise in certain dynamic programming algorithms.
- Exponential functions, f(n) = c^n for a given constant c > 1: Functions like 2^n arise when enumerating all subsets of n items. As we have seen, exponential algorithms become useless fast, but not as fast as...
- Factorial functions, f(n) = n!: Functions like n! arise when generating all permutations or orderings of n items.

|     n     |   lgn    |    n    |   nlgn   |   n^2   |  2^n   |
|:---------:|:--------:|:-------:|:--------:|:-------:|:------:|
|    10     | 0.003 μs | 0.01 μs | 0.033 μs | 0.1 μs  |  1 μs  |
|    20     | 0.004 μs | 0.02 μs | 0.086 μs | 0.4 μs  |  1 ms  |
|    30     | 0.005 μs | 0.03 μs | 0.147 μs | 0.9 μs  |  1 sec |
|    40     | 0.005 μs | 0.04 μs | 0.213 μs | 1.6 μs  | 18.3 min |
|    50     | 0.006 μs | 0.05 μs | 0.282 μs | 2.5 μs  |  13 days |
|   100     | 0.007 μs | 0.1 μs  | 0.644 μs | 10 μs   |         |
|  1,000    | 0.010 μs | 1.00 μs | 9.966 μs | 1 ms    |         |
| 10,000    | 0.013 μs | 10 μs   | 130 μs   | 100 ms  |         |
| 100,000   | 0.017 μs | 0.10 ms | 1.67 ms  | 10 sec  |         |
| 1,000,000 | 0.020 μs | 1 ms    | 19.93 ms | 16.7 min|         |
|10,000,000 | 0.023 μs | 0.01 sec| 0.23 sec | 1.16 days|        |
|100,000,000| 0.027 μs | 0.10 sec| 2.66 sec | 115.7 days|       |
|1,000,000,000| 0.030 μs | 1 sec  | 29.90 sec| 31.7 years|      |

## Canonicalization
Reducing complicated objects to a standard (i.e. “canonical”) form. String transformations like reducing letters to lower case or stemming (removing word suffixes like -ed, -s, or -ing) result in increased matches, because multiple strings collide on the same code.

## Compaction
Also called fingerprinting, where we representing large objects by small hash codes. It is easier to work with small objects than large ones, and the hash code generally preserves the identity of each item.
