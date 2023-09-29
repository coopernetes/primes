const { performance, PerformanceObserver } = require('perf_hooks')

const perfObserver = new PerformanceObserver(() => {})

perfObserver.observe({ entryTypes: ["measure"], buffer: true })
performance.mark('start')

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

performance.mark('end')
const perfResult = performance.measure('runtime', 'start', 'end')


console.log(`Time (ns): ${(perfResult.duration * 1_000_000.0).toFixed(0)}`)
console.log(`Time (µs): ${(perfResult.duration * 1_000.0).toFixed(3)}`)
console.log(`Time (ms): ${perfResult.duration.toFixed(3)}`)
console.log(`Time (s):  ${(perfResult.duration / 1_000.0).toFixed(4)}`)
