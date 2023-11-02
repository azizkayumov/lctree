### Benchmark
The overall running time for performing a number of random operations (`link(v, w)`, `cut(v, w)`, `connected(v, w)` or `findmax(v, w)`) on forests of varying sizes (check benchmark details [here](https://github.com/azizkayumov/lctree/blob/main/tests/README.md)).

| # Nodes     | # Operations    | Random seed           | [lctree](https://github.com/azizkayumov/lctree/blob/main/src/lctree.rs)    | [brute-force](https://github.com/azizkayumov/lctree/blob/main/tests/test_random.rs)  | 
| :---        | :---            | :---                  | :---          | :---            |
| 100         | 10K             | 14371286973218730379  | 18.005544ms   | 291.587072ms    |
| 100         | 100K            | 18146878621059190265  | 186.174183ms  | 3.055154731s    |
| 100         | 1M              | 6839381432488849859   | 1.824378819s  | 30.510083671s   |
| 500         | 2M              | 12719220817276010307  | 5.17505883s   | 303.150073635s  |
| 1000        | 5M              | 16452801585435658354  | 14.711844242s | 1527.065366409s |

The following table includes worst-case time complexity analysis of each operation for the brute-force solution and Link-cut-trees:

| Operation   |  link(v, w)  |  cut(v, w) |  connected(v, w)  |  path(v, w)  |
| :---        | :---         | :---       |  :---             |  :---        |
| [lctree](https://github.com/azizkayumov/lctree/blob/main/src/lctree.rs)                     | `O(logn)`                   | `O(logn)`                    |  `O(logn)`  |  `O(logn)`              |
| [brute-force](https://github.com/azizkayumov/lctree/blob/main/tests/test_random.rs)         | `O(min{size(v), size(w)})`  | `O(min{size(v), size(w)})`   |  `O(1)`     |  `O(size(v) + size(w))` |

This time complexity analysis can clearly be observed in practice on the last experiment with 1000 nodes and 5M random operations:

| Operation   | Count   | [lctree](https://github.com/azizkayumov/lctree/blob/main/src/lctree.rs)    | [brute-force](https://github.com/azizkayumov/lctree/blob/main/tests/test_random.rs)  |
| :---        | :---            | :---            | :---                  |
| link        | 1249509         | 3.671253873s    | 1.877281667s          |
| cut         | 1251768         | 3.694333793s    | 4.175882634s          |
| connected   | 1250180         | 3.662950986s    | 79.612576ms           |
| path        | 1248543         | 3.68330559s     | 1520.932589532s       |

\* Benchmarks were run on iMac 24" M1 2021 16Gb.

\* To reproduce these results, please refer to the `\tests` folder and run the random test by configuring its random seed, the number of nodes, and the number of operations.
