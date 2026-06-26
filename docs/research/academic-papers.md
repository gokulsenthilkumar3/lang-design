# Academic Papers: Reading List

This document tracks academic papers relevant to the language design.
Each entry notes the paper's contribution and which RFC(s) it informs.

---

## Type Systems

### Hindley-Milner

**"A Theory of Type Polymorphism in Programming"**\
Robin Milner, 1978\
*The foundation of ML-style type inference. Algorithm W is still the basis
of most modern type inference engines.*\
Relevant to: RFC-0003

**"Principal Type-Schemes for Functional Programs"**\
Damas and Milner, 1982\
*Proves completeness of Algorithm W.*\
Relevant to: RFC-0003

### Refinement Types

**"Refinement Types for ML"**\
Freeman and Pfenning, 1991\
*Original refinement types paper. Predicates as type refinements.*\
Relevant to: RFC-0003

**"LiquidHaskell: Experience with Refinement Types"**\
Vazou et al., 2014\
*Practical refinement types in a production language. Key lessons:
SMT solver integration, annotation burden, false positive rate.*\
Relevant to: RFC-0003

### Affine / Linear Types

**"Linear Types Can Change the World"**\
Philip Wadler, 1990\
*Foundational paper on linear types and resource management.*\
Relevant to: RFC-0004

**"The Rust Reference (Memory Model)"**\
The Rust Project\
*Production evidence that affine types work at scale.*\
Relevant to: RFC-0004

---

## Effect Systems

**"An Introduction to Algebraic Effects and Handlers"**\
Matija Pretnar, 2015\
*Best tutorial-level introduction to algebraic effects.*\
Relevant to: RFC-0005

**"Do Be Do Be Do"**\
Lindley, McBride, and McLaughlin, 2016\
*Frank language: effects and handlers with effect polymorphism.*\
Relevant to: RFC-0005

**"Algebraic Effects for the Rest of Us"**\
Dan Abramov, 2019 (blog post)\
*Accessible explanation of algebraic effects via React's mental model.*\
Relevant to: RFC-0005 (communication / documentation)

---

## Memory Models

**"Foundations of the C++ Concurrency Memory Model"**\
Boehm and Adve, 2008\
*The basis of the acquire/release memory model used here.*\
Relevant to: RFC-0004

**"Ownership is Theft"**\
Niko Matsakis, 2014 (blog post)\
*Explains the intuition behind Rust's ownership model.*\
Relevant to: RFC-0004

---

## AI / Tensor Types

**"Dex: Array Programming with Typed Indices"**\
Paszke et al., 2021\
*Type-safe array programming where shapes are in the type. Direct
predecessor of tensor shape types in RFC-0003.*\
Relevant to: RFC-0003

**"MLIR: Scaling Compiler Infrastructure for Domain Specific Computation"**\
Lattner et al., 2021\
*Multi-level IR. Relevant to the IR design in Phase 3.*\
Relevant to: future RFC on IR design

---

## Compiler Engineering

**"Modern Compiler Implementation in ML"**\
Andrew Appel, 1998\
*The standard reference for recursive-descent parsers and SSA IR.*\
Relevant to: Phases 1, 3

**"Engineering a Compiler"**\
Cooper and Torczon, 2011\
*Comprehensive compiler engineering reference.*\
Relevant to: Phases 1-4
