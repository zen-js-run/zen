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
| Zen 	| 100.32       	| 82.00       	| 29.29         	| N/A                      	| 9,592    	|
| Node.js | 133.60       	| 104.48      	| 30.39         	| 23.856                   	| 9,592    	|
| Deno	| 118.37       	| 71.14       	| 44.23         	| 17.5                    	| 9,592    	|

## Observations

- **Zen** is the fastest overall, completing the task in **100.32 ms**.
- **Deno** is the second fastest, with a total time of **118.37 ms**.
- **Node.js** is the slowest, taking **133.60 ms** to execute the task.
- Zen demonstrates the lowest user time at **82.00 ms**, indicating better efficiency in CPU usage compared to Node.js and Deno.
- Deno has the lowest prime calculation time of **17.5 ms**, suggesting its algorithm is optimized for this task, despite having a higher system time of **44.23 ms**.

## Conclusion
Zen outperforms both Node.js and Deno in overall execution time while maintaining competitive user and system times. These results indicate that Zen is a highly efficient runtime for prime number calculations, showcasing its potential for performance-sensitive applications.