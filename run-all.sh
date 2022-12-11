#!/usr/bin/env bash
echo " === Python ==="
echo ""
python3 python/primes.py

echo ""
echo " === Go ==="
echo ""
go run go/main.go

echo ""
echo " === Java ==="
echo ""
(cd java && javac Main.java && java Main)

echo ""
echo " === Rust ==="
echo ""
(cd rust/primes && cargo run -q)
