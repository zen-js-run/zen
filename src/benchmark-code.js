function countPrimes(upTo) {
  let count = 0;
  const isPrime = Array(upTo + 1).fill(true);
  isPrime[0] = isPrime[1] = false;

  for (let i = 2; i * i <= upTo; i++) {
      if (isPrime[i]) {
          for (let j = i * i; j <= upTo; j += i) {
              isPrime[j] = false;
          }
      }
  }

  for (let i = 2; i <= upTo; i++) {
      if (isPrime[i]) {
          count++;
      }
  }

  console.log(`Number of primes up to ${upTo}: ${count}`);
}

countPrimes(100000);