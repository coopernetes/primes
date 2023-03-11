# primes
Calculate primes up to a maximum number in various languages.

## Usage
Each program will accept in an argument to change the maximum number to loop through. 
By default, each will check numbers from 1 to 1,000.

The [`run-all.sh`](run-all.sh) script will run each program. To change the default
maximum number, set the `MAX_NUMBER` variable.

```shell
MAX_NUMBER=5000 ./run-all.sh
```

## Why these languages?
In no particular order:
1. I'm already familiar with the language and want to see it's (basic) performance characteristics 
   compared to other languages. It's interesting comparing them based off of static vs dynamic typing,
   compiled vs intepretted and the effects of how they handle memory (garbage collection).
2. An excuse to learn a little snippet from other languages that pique my own curiosity or where I don't have
   a lot of opportunity to use it in my day job (Rust, Kotlin).
3. Try out some profiling tools.
4. Learn some basic idioms.

## TODO
- [x] Python
- [x] Go
- [x] Java
- [x] Rust
- [x] JavaScript
- [x] C#
- [ ] Kotlin
- [ ] Clojure
