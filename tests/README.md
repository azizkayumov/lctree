### Benchmark
This benchmark report contains overall running time analysis of Link-Cut trees in comparison to its brute-force counterpart (iMac 24", M1, 2021, 16Gb).
Each test performs a number of random operations (`link(v, w)`, `cut(v, w)`, `connected(v, w)` or `findmax(v, w)`) on forests of varying sizes.

| # Nodes     | # Operations    | Random seed           | [lctree](https://github.com/azizkayumov/lctree/blob/main/src/lctree.rs)    | [brute-force](https://github.com/azizkayumov/lctree/blob/main/tests/test_random.rs)  | 
| :---        | :---            | :---                  | :---          | :---            |
| 100         | 10K             | 14371286973218730379  | 18.005544ms   | 291.587072ms    |
| 100         | 100K            | 18146878621059190265  | 186.174183ms  | 3.055154731s    |
| 100         | 1M              | 6839381432488849859   | 1.824378819s  | 30.510083671s   |
| 500         | 2M              | 12719220817276010307  | 5.17505883s   | 303.150073635s  |
| 1000        | 5M              | 16452801585435658354  | 14.711844242s | 1527.065366409s |

The brute force solution takes  time for `link(v, w)` and `cut(v, w)` operations where `size(v)` or `size(w)` is the number of points connected to the point.
Then, `connected(v, w)` query can be performed in a constant time.
However, `path(v, w)` operation takes `O(size(v) + size(w))` where `size(v) + size(w) = n` in the worst-case scenario for the brute force.
On the other hand, all of these operations take `O(logn)` amortized time in the case of Link-cut trees.

The time complexity analysis can be observed in practice on the last experiment with 1000 nodes and 5M random operations:

| Operation   | Count   | [lctree](https://github.com/azizkayumov/lctree/blob/main/src/lctree.rs)    | [brute-force](https://github.com/azizkayumov/lctree/blob/main/tests/test_random.rs)  |
| :---        | :---            | :---            | :---                  |
| link        | 1249509         | 3.671253873s    | 1.877281667s          |
| cut         | 1251768         | 3.694333793s    | 4.175882634s          |
| connected   | 1250180         | 3.662950986s    | 79.612576ms           |
| path        | 1248543         | 3.68330559s     | 1520.932589532s       |
