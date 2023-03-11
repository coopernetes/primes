package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"time"
)

func main() {
	start := time.Now()
	largestNumber := 1000
	if len(os.Args) > 1 && len(os.Args[1]) != 0 {
		a, err := strconv.Atoi(os.Args[1])
		if err != nil {
			log.Fatal(err)	
		}
		largestNumber = a
	}
	result := primes(largestNumber)
	end := time.Now()
	fmt.Printf("\nCount: %d\n", len(result))
	diff := end.Sub(start)
	diffNanos := float64(diff.Nanoseconds())
	fmt.Printf("Time (ns): %d\n", diff.Nanoseconds())
	fmt.Printf("Time (µs): %.3f\n", diffNanos / 1_000.0)
	fmt.Printf("Time (ms): %.3f\n", diffNanos / 1_000_000.0)
	fmt.Printf("Time (s):  %.4f\n", diffNanos / 1_000_000_000.0)
}


func primes(largestNumber int) []int {
	primes := make([]int, 0)
	i := 1
	for i <= largestNumber {
		fmt.Printf("Is %d prime? ", i)
		isPrime := true
		for _, p := range primes {
			if (p == 1 || p == i) {
				continue;
			}
			if (i % p == 0) {
				isPrime = false;
			}
		}
		if isPrime {
			fmt.Printf("yes\r")
			primes = append(primes, i)
		} else {
			fmt.Printf("no \r")
		}
		i += 1
	}
	return primes
}