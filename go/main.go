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
	primes(largestNumber)
	end := time.Now()
	diff := end.Sub(start)
	diffNanos := float64(diff.Nanoseconds())
	fmt.Printf("Time (ns): %d\n", diff.Nanoseconds())
	fmt.Printf("Time (µs): %.3f\n", diffNanos / 1_000.0)
	fmt.Printf("Time (ms): %.3f\n", diffNanos / 1_000_000.0)
	fmt.Printf("Time (s):  %.4f\n", diffNanos / 1_000_000_000.0)
}


func primes(largestNumber int) {
	primes := make([]int, 1)
	i := 1
	for {
		fmt.Printf("Is %d prime? ", i)
		if i >= largestNumber {
			break
		}
		if i == 1 {
			fmt.Printf("yes\r")
			primes[0] = i
		} else if i == 2 || i == 3 {
			fmt.Printf("yes\r")
			primes = append(primes, i)
		} else {
			divs := make([]int, 0)
			for _, p := range primes {
				if i % p == 0 {
					divs = append(divs, p)
				}
			}
			if len(divs) == 1 {
				fmt.Printf("yes\r")
				primes = append(primes, i)
			} else {
				fmt.Printf("no \r")
			}
		}
		i += 1
	}
	fmt.Printf("\nCount: %d\n", len(primes))
}