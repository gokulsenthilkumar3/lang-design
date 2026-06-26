# Benchmark Targets

This document consolidates all quantitative performance targets across
RFCs. Every target here is referenced in at least one RFC. If an
implementation fails a target, the RFC must be updated before the
corresponding phase gate is passed.

---

## Compiler Performance

| Metric | Target | Source RFC |
|--------|--------|------------|
| Type check: 100k LOC | < 5 seconds | RFC-0004 |
| Refinement check: simple predicate | < 100ms | RFC-0004 |
| Tensor shape inference: per matmul | < 1ms | RFC-0004 |
| Type error: actionable message rate | > 90% | RFC-0004 |
| Compile-time regression per PR | < 2% | RFC-0004 |

---

## Memory System

| Metric | Target | Source RFC |
|--------|--------|------------|
| Allocation throughput | > 500M alloc/sec | RFC-0005 |
| Deallocation throughput | > 500M dealloc/sec | RFC-0005 |
| GC pause (safe subset) | 0ms (no GC) | RFC-0005 |
| Region allocator overhead vs malloc | < 5% | RFC-0005 |
| Functions needing lifetime annotations | < 10% | RFC-0005 |

---

## Effect System

| Metric | Target | Source RFC |
|--------|--------|------------|
| Effect handler dispatch overhead | < 5 ns | RFC-0006 |
| Async task switch latency | < 1 microsecond | RFC-0006 |
| Pure function effect overhead | 0 ns | RFC-0006 |

---

## Runtime (Phase 5, to be detailed in RFC-0008+)

| Metric | Target | Notes |
|--------|--------|-------|
| Kernel boot time (if OS integration) | < 1 second | |
| Context switch | < 300 ns | |
| Filesystem IOPS | > 1M IOPS | NVMe target |
| Network stack latency | < 50 microseconds | Loopback |
| Startup time (hello world binary) | < 10ms | |
| Binary size (hello world, stripped) | < 500KB | |

---

## Toolchain and Ecosystem

These targets support RFC-0007 and the later ecosystem phases.

| Metric | Target | Notes |
|--------|--------|-------|
| Package registry: publish time | < 30 seconds | |
| Package registry: install (cached) | < 1 second | |
| LSP response time (completions) | < 100ms | |
| Formatter: 100k LOC | < 1 second | |
| REPL startup | < 500ms | |

---

## Reliability

| Metric | Target | Notes |
|--------|--------|-------|
| Compiler fuzzing: crashes | 0 after 10B inputs | Phase 10 gate |
| Compiler test coverage | > 95% line coverage | |
| Stdlib test coverage | > 95% line coverage | |
| Reproducible build: bit-identical output | 100% | Phase 10 gate |
| CVE response time | < 14 days | SECURITY.md |

---

## How to Update This Document

When a new RFC introduces a quantitative target, add it here with the
RFC reference. When a target is revised (via RFC update), update both
the RFC and this document in the same PR.
