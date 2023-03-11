import time
import sys

def main():
    primes = []
    largest_number = 1000
    if len(sys.argv) > 1 and len(sys.argv[1]) != 0:
        largest_number = int(sys.argv[1])
    i = 1
    while i <= largest_number:
        print(f"Is {i} prime?\t", end='')
        is_prime = True
        for p in primes:
            if p == 1 or p == i:
                continue
            if i % p == 0:
                is_prime = False    
        if is_prime:
            print("yes", end='\r', flush=True)
            primes.append(i)
        else:
            print("no ", end='\r', flush=True)
        i += 1
    print(f"\nCount: {len(primes)}")

if __name__ == "__main__":
    start = time.time_ns()
    main()
    end = time.time_ns()
    td = end - start
    print(f"Time (ns): {td}")
    print("Time (µs): {0:.3f}".format(td / 1_000))
    print("Time (ms): {0:.3f}".format(td / 1_000_000))
    print("Time (s):  {0:.4f}".format(td / 1_000_000_000))

