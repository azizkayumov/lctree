### Benchmark
The overall running time for performing a number of random operations (`link(v, w)`, `cut(v, w)`, `connected(v, w)` or `findmax(v, w)`) on forests of varying sizes.

| # Nodes     | # Operations    | Random seed           | [lctree](https://github.com/azizkayumov/lctree/blob/main/src/lctree.rs)    | [brute-force](https://github.com/azizkayumov/lctree/blob/main/tests/test_random.rs)  | 
| :---        | :---            | :---                  | :---          | :---            |
| 100         | 10K             | 0                     | 4.8973 ms     | 18.992 ms       |
| 200         | 20K             | 1                     | 11.175 ms     | 74.780 ms       |
| 500         | 50K             | 2                     | 31.590 ms     | 471.71 ms       |
| 1000        | 1M              | 3                     | 699.00 ms     | 19.608 s        |
| 5000        | 5M              | 4                     | 4.5047 s      | 500.04 s        |

The following table includes worst-case time complexity analysis of each operation for the brute-force solution and Link-cut-trees:

| Operation   |  link(v, w)  |  cut(v, w) |  connected(v, w)  |  path(v, w)  |
| :---        | :---         | :---       |  :---             |  :---        |
| [lctree](https://github.com/azizkayumov/lctree/blob/main/src/lctree.rs)                     | `O(logn)`                   | `O(logn)`                    |  `O(logn)`  |  `O(logn)`              |
| [brute-force](https://github.com/azizkayumov/lctree/blob/main/tests/test_random.rs)         | `O(min{size(v), size(w)})`  | `O(min{size(v), size(w)})`   |  `O(1)`     |  `O(size(v) + size(w))` |

\* Benchmarks were run on iMac 24" M1 2021 16Gb.

\* To reproduce these results, please refer to the `\benches` folder or run `cargo bench`.
