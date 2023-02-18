#!/usr/bin/env bash
_arg=""
if [ -n "${MAX_NUMBER}" ]; then
    _arg=$MAX_NUMBER
fi

echo " === Python ==="
echo ""
python3 python/primes.py "${_arg}"

echo ""
echo " === Go ==="
echo ""
go run go/main.go "${_arg}"

echo ""
echo " === Java ==="
echo ""
(cd java && javac Main.java && java Main "${_arg}")

echo ""
echo " === Rust ==="
echo ""
(cd rust/primes && cargo build -q -r && ./target/release/primes "${_arg}")

echo ""
echo " === Node ==="
echo ""
(cd js && node primes.js "${_arg}")
