# Benchmark Documentation: Prime Number Calculation

## Objective
To compare the performance of Zen, Node.js, and Deno in calculating prime numbers up to 100,000.

## Methodology
The benchmark involves executing a script (`input.js`) that calculates the number of prime numbers up to 100,000 using each runtime. The following metrics were recorded:

- Total execution time
- User time (time spent in user mode)
- System time (time spent in kernel mode)
- Time taken for prime number calculation
- Number of primes found

## Results

| Runtime | Total Time (ms) | User Time (ms) | System Time (ms) | Prime Calculation Time (ms) | Primes Found |
|---------|------------------|-----------------|-------------------|------------------------------|--------------|
| Zen     | 55.82            | 37.73           | 18.82             | N/A                          | 9,592        |
| Node.js | 86.96            | 75.46           | 12.69             | 23.601                       | 9,592        |
| Deno    | 313.37           | 118.69          | 90.79             | 41.9                         | 9,592        |

## Observations

- **Zen** is the fastest overall, completing the task in **55.82 ms**.
- **Node.js** is the second fastest, with a total time of **86.96 ms**.
- **Deno** is the slowest, taking **313.37 ms** to execute the task.
- Zen demonstrates the lowest user time at **37.73 ms**, indicating better efficiency in CPU usage compared to Node.js and Deno.
- Deno has the lowest prime calculation time of **41.9 ms**, suggesting its algorithm is optimized for this task, despite having a much higher total execution time.

## Conclusion
Zen outperforms both Node.js and Deno in overall execution time while maintaining competitive user and system times. These results indicate that Zen is a highly efficient runtime for prime number calculations, showcasing its potential for performance-sensitive applications.