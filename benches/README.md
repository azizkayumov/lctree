### Benchmark
The overall running time for performing a number of random operations (`link(v, w)`, `cut(v, w)`, `connected(v, w)` or `findmax(v, w)`):

| # Nodes     | # Operations    | Random seed           | [lctree](https://github.com/azizkayumov/lctree/blob/main/src/lctree.rs)    | [brute-force](https://github.com/azizkayumov/lctree/blob/main/benches/benchmark.rs)  | 
| :---        | :---            | :---                  | :---          | :---            |
| 1000        | 10K             | 0                     | 3.6795 ms     | 91.215 ms       |
| 1000        | 50K             | 1                     | 25.572 ms     | 728.53 ms       |
| 1000        | 100K            | 2                     | 54.677 ms     | 1.5267 s        |
| 1000        | 500K            | 3                     | 274.20 ms     | 7.8320 s        |
| 1000        | 1M              | 4                     | 540.83 ms     | 15.476 s        |

The following table includes worst-case time complexity analysis of each operation for the brute-force solution and Link-cut-trees:

| Operation   |  link(v, w)  |  cut(v, w) |  connected(v, w)  |  path(v, w)  |
| :---        | :---         | :---       |  :---             |  :---        |
| [lctree](https://github.com/azizkayumov/lctree/blob/main/src/lctree.rs)                     | `O(logn)`                   | `O(logn)`                    |  `O(logn)`  |  `O(logn)`              |
| [brute-force](https://github.com/azizkayumov/lctree/blob/main/benches/benchmark.rs)         | `O(min{size(v), size(w)})`  | `O(min{size(v), size(w)})`   |  `O(1)`     |  `O(size(v) + size(w))` |

\* Benchmarks were run on iMac 24" M3 2023 8Gb.

\* To reproduce these results, please run `cargo bench`.
