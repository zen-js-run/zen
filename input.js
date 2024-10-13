function isPrime(num) {
  if (num <= 1) return false;
  for (let i = 2; i <= Math.sqrt(num); i++) {
      if (num % i === 0) return false;
  }
  return true;
}

function findPrimes(limit) {
  const primes = [];
  for (let i = 2; i <= limit; i++) {
      if (isPrime(i)) {
          primes.push(i);
      }
  }
  return primes;
}

// Adjust the limit for more CPU usage
const limit = 100000; 
console.time('Prime Calculation');
const primes = findPrimes(limit);
console.timeEnd('Prime Calculation');

console.log(`Found ${primes.length} primes up to ${limit}.`);