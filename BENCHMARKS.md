# Zen JavaScript Runtime Benchmarks

This document summarizes the benchmark results for the Zen JavaScript runtime compared to Node.js, Deno, and Bun in calculating prime numbers up to 100,000.

## Benchmark Results

| Runtime    | Time (ms)   | Speedup Compared to Zen (%) |
|------------|-------------|------------------------------|
| Zen        | 20.24       | -                            |
| Node.js    | 101.36      | 80.00                        |
| Deno       | 264.70      | 91.36                        |
| Bun        | 82.70       | 309.58                       |

## Description

The benchmarks were conducted using a script that counts the number of prime numbers up to 100,000. The results demonstrate Zen's superior performance compared to other JavaScript runtimes.

### Execution Details

- **Zen**:
  - **User Time**: 4.69 ms
  - **System Time**: 7.78 ms
- **Node.js**:
  - **User Time**: 76.56 ms
  - **System Time**: 23.24 ms
- **Deno**:
  - **User Time**: 111.72 ms
  - **System Time**: 66.09 ms
- **Bun**:
  - **User Time**: 51.06 ms
  - **System Time**: 34.73 ms

## Conclusion

Zen demonstrates outstanding performance, completing the prime number calculation significantly faster than its competitors.