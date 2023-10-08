#!/usr/bin/env bash
set -ueo pipefail
_arg=""
if [[ $# -eq 1 ]]; then
    _arg=$1
else
    _arg=1000
fi

if ! [[ "$_arg" =~ ^[0-9]+$ ]]; then
    echo "Error: argument must be an integer"
    exit 1
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

echo ""
echo " === C# ==="
echo ""
(cd csharp && dotnet run "${_arg}")
