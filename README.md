# Rust SAT Solver

A command-line SAT solver written in Rust for propositional logic expressions in **CNF / Product of Sums (PoS)** form.

## What it does

- Parses literals and clauses such as `(P0 v P1).(~P0 v P2)`
- Supports negation and multi-digit variable names such as `P10`
- Represents truth assignments as bits in an integer
- Uses short-circuit evaluation for clauses and the full expression
- Exhaustively searches assignments until it finds a satisfying one
- Includes unit tests for SAT, UNSAT, parsing, and a system-specification example

## Example

```bash
cargo run -- "(P0 v P1).(~P0 v P2)"
```

Example output includes a satisfying assignment when one exists, or `UNSAT` when no assignment satisfies the expression.

## Testing

```bash
cargo test
```

The included tests cover:

- a contradictory UNSAT expression
- a satisfiable CNF expression
- parsing of multi-digit variables
- a small system-specification constraint set

## Tech

Rust • Cargo • CNF / PoS parsing • bit-based truth assignments • unit testing
