# Fuzzing Setup for Lease Crate

This document describes the fuzzing setup implemented to test the unsafe code in the lease crate.

## Overview

The lease crate contains unsafe code (`ptr::read`/`write` operations) and complex RAII patterns (`CancellationGuard`). Fuzzing provides additional confidence that this unsafe code is correct and memory-safe.

## Fuzz Targets

### `lease_mut` (Most Critical)
- **Purpose**: Tests the core unsafe `ptr::read`/`write` operations in `lease_mut`
- **Coverage**: Various data types, edge cases, error paths
- **Run time**: 30+ seconds recommended

### `lease_async_mut`
- **Purpose**: Tests cancellation guards and async safety in `lease_async_mut` operations
- **Coverage**: Both checked (clone + guard) and unchecked (panic-on-cancel) paths
- **Run time**: 30+ seconds recommended

### `owned_lease`
- **Purpose**: Tests zero-cost owned value operations (always safe)
- **Coverage**: Error propagation, various data patterns
- **Run time**: 15+ seconds recommended

## Running Fuzzing

### Prerequisites
```bash
rustup install nightly
cargo install cargo-fuzz
```

### Individual Targets
```bash
cd fuzz
cargo +nightly fuzz run lease_mut -- -max_total_time=30
cargo +nightly fuzz run lease_async_mut -- -max_total_time=30
cargo +nightly fuzz run owned_lease -- -max_total_time=15
```

### All Targets (Convenience Script)
```bash
./run_fuzz.sh
```

### With Sanitizers (Recommended)
```bash
cargo +nightly fuzz run lease_mut --sanitizer=address -- -max_total_time=30
```

## Success Criteria

- ✅ **No crashes** after 30+ seconds per target
- ✅ **No sanitizer warnings** (ASan/UBSan reports)
- ✅ **Growing corpus** indicates interesting edge cases found
- ✅ **CI passes** on all pull requests

## What Fuzzing Tests

- **Memory safety**: Double-frees, use-after-free, invalid pointers
- **UB detection**: Incorrect `ptr::read`/`write` usage
- **Guard correctness**: Failed restoration on drop/cancellation
- **Edge cases**: Large data, unusual patterns, boundary conditions

## CI Integration

GitHub Actions automatically runs fuzzing on every PR:
- `.github/workflows/fuzz.yml`
- Runs each target for 30-60 seconds
- Fails if any crashes are found

## Notes

- **Nightly required**: Uses unstable compiler flags for sanitizers
- **No intentional panics**: Fuzz targets avoid expected panics (tested in unit tests)
- **Corpus management**: Artifacts saved for crash reproduction
- **Performance**: Fuzzing is compute-intensive but finds bugs unit tests miss

## Troubleshooting

**"command not found: timeout"**
- Use Ctrl+C to stop fuzzing manually
- Or run with `--max_total_time=N` parameter

**Sanitizer errors**
- Ensure you're using nightly Rust
- Check that you have LLVM development tools installed

**Slow fuzzing**
- Reduce `--max_total_time` for faster iteration
- Focus on one target at a time during development
## Recent Fuzz Results (Mon Feb 23 18:34:14 UTC 2026)

Total executions across all targets: **24656960**

### Target Results
- **lease_mut**: 10020065 runs in 30s
- **lease_async_mut**: 8676960 runs in 30s
- **owned_lease**: 5959935 runs in 15s

### Summary
- ✅ No crashes detected
- ✅ Memory safety verified
- ✅ Sanitizers passed

