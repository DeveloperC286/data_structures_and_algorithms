# Quicksort
[![pipeline status](https://img.shields.io/badge/Version-0.1.1-blue)](https://gitlab.com/DeveloperC/sorting-algorithms/commits/master)

Implementation of the sorting algorithm Quicksort in pure Rust with no external libraries, complete with unit tests for edge cases and also property based testing.

This implementation has minor optimizations benchmarked and verified statically using the benchmarking crate 'criterion'.

## Building
```
cargo build --release
```

## Testing
```
cargo test
```

## Benchmarking
```
cargo bench
```