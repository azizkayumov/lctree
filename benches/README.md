### Benchmark
The overall running time for performing a number of random operations (`link(v, w)`, `cut(v, w)`, `connected(v, w)` or `findmax(v, w)`) on forests of varying sizes.

| # Nodes     | # Operations    | Random seed           | [lctree](https://github.com/azizkayumov/lctree/blob/main/src/lctree.rs)    | [brute-force](https://github.com/azizkayumov/lctree/blob/main/benches/benchmark.rs)  | 
| :---        | :---            | :---                  | :---          | :---            |
| 100         | 10K             | 0                     | 4.8161 ms     | 18.013 ms       |
| 200         | 20K             | 1                     | 11.091 ms     | 69.855 ms       |
| 500         | 50K             | 2                     | 31.623 ms     | 429.53 ms       |
| 1000        | 100K            | 3                     | 68.649 ms     | 1.8746 s        |
| 5000        | 500K            | 4                     | 445.83 ms     | 46.854 s        |
| 10K         | 1M              | 5                     | 964.64 ms     | 183.24 s        |

The following table includes worst-case time complexity analysis of each operation for the brute-force solution and Link-cut-trees:

| Operation   |  link(v, w)  |  cut(v, w) |  connected(v, w)  |  path(v, w)  |
| :---        | :---         | :---       |  :---             |  :---        |
| [lctree](https://github.com/azizkayumov/lctree/blob/main/src/lctree.rs)                     | `O(logn)`                   | `O(logn)`                    |  `O(logn)`  |  `O(logn)`              |
| [brute-force](https://github.com/azizkayumov/lctree/blob/main/benches/benchmark.rs)         | `O(min{size(v), size(w)})`  | `O(min{size(v), size(w)})`   |  `O(1)`     |  `O(size(v) + size(w))` |

\* Benchmarks were run on iMac 24" M1 2021 16Gb.

\* To reproduce these results, please refer to the `\benches` folder or simply run `cargo bench`.
