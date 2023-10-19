### Benchmark
This benchmark report contains overall running time analysis of Link-Cut trees in comparison to its brute-force counterpart (MBP 14" 16Gb).
Each test performs a number of random operations (`link(v, w)`, `cut(v, w)`, `connected(v, w)` or `findmax(v, w)`) on forests of varying sizes.

| # Nodes     | # Operations    | Random seed           | [lctree](https://github.com/azizkayumov/lctree/blob/main/src/lctree.rs)    | [brute-force](https://github.com/azizkayumov/lctree/blob/main/tests/test_random.rs)  | 
| :---        | :---            | :---                  | :---          | :---          |
| 100         | 10K             | 564315935137013477    | 6.567967ms    | 53.48109ms    |
| 100         | 100K            | 5233351911053448040   | 44.379304ms   | 321.900746ms  |
| 100         | 1M              | 10905789823848117209  | 476.117191ms  | 3.915883695s  |
| 500         | 2M              | 5863263585868731364   | 984.139022ms  | 11.542679321s |
| 1000        | 5M              | 3900765363016383448   | 2.203334524s  | 15.85451642s  |

The running time decomposition for the experiment with 1000 nodes and 5M random operations:

| Operation   | Count   | [lctree](https://github.com/azizkayumov/lctree/blob/main/src/lctree.rs)    | [brute-force](https://github.com/azizkayumov/lctree/blob/main/tests/test_random.rs)  |
| :---        | :---            | :---            | :---                  |
| link        | 1249676         | 481.699835ms    | 3.725153109s          |
| cut         | 1246762         | 809.13079ms     | 7.535368601s          |
| connected   | 1249204         | 432.403952ms    | 63.781244ms           |
| path        | 1249798         | 480.099947ms    | 4.530213466s          |
