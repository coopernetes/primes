const start = Date.now();
let max = 1000;
let primes = [];

if (process.argv.length > 2) {
    const maybeInt = parseInt(process.argv[2])
    if (!isNaN(maybeInt)) {
        max = maybeInt
    }
}

for (let i = 1; i <= max; i++) {
    let out = `Is ${i} prime? `
    let isPrime = true;
    for (p of primes) {
        if (p == 1 || p == i) {
            continue
        }
        if (i % p == 0) {
            isPrime = false;
            break
        }
    }
    if (isPrime) {
        primes.push(i)
        out += 'yes\r'
    }
    else {
        out += 'no \r'
    }
    process.stdout.write(out)
}

console.log(`\nCount: ${primes.length}`)

const elapsed = Date.now() - start;
console.log(`Time (ms): ${elapsed}`)
console.log(`Time (s): ${elapsed / 1_000.0}`)
