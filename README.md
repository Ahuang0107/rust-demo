# Readme

## Benchmark

目前测下来的一些操作cost的时间空间。

| Action                                        | Matrix Size           | Cost  |   Memory    | 
|:----------------------------------------------|:----------------------|:-----:|:-----------:|
| pass to webassembly                           | 1,000*1,000=1,000,000 |  1ms  | 3889999byte |
| traverse in webassembly and store in variable | 1,000*1,000=1,000,000 | 161ms | 3889999byte |
| create 1000 3*3 matrix                        | 1,000*(3*3)=9,000     | 31ms  |  15999byte  |