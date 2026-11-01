# PPVS (Physical Principles Verification System)

> **Status:** Proof of Concept (PoC). This is an experimental prototype to explore the idea and is by no means complete or perfect.

---

## The Idea

In software engineering, continuous integration runs a test suite to ensure new code does not break existing features.

**PPVS** explores applying this concept to physical theories:
- **Empirical Benchmarks as Tests:** Well-established experimental measurements (such as planetary orbits, the speed of light, muon dilation, or atomic spectra) serve as regression tests with real experimental uncertainties.
- **Non-Regression:** A new proposal should not contradict established empirical facts.
- **Diagnostic Feedback:** Instead of dogmatic rejection, failed predictions report exact numerical discrepancies and $\sigma$-deviations to highlight where the model needs work.
- **Occam's Razor:** Evaluates theories by how much they explain relative to the assumptions they make:
  $$\text{Relevance} = \frac{\text{phenomena explained}}{\text{total known phenomena} \times \text{assumptions}}$$
- **Correspondence Limits:** Verifies whether a candidate theory simplifies to established physics in familiar regimes (e.g., relativistic mechanics $\to$ Newtonian mechanics at low velocities $v \ll c$).

---

## Project Structure

- [`src/dimensions.rs`](src/dimensions.rs): SI base dimensional safety to catch unphysical operations.
- [`src/empirical.rs`](src/empirical.rs): A sample registry of benchmark observations across gravity, quantum/electrodynamics, and thermodynamics.
- [`src/theory.rs`](src/theory.rs): Traits defining physical theories, principles, and predictions.
- [`src/established.rs`](src/established.rs): Established baseline theories and compatibility auditing.
- [`src/explainability.rs`](src/explainability.rs): Explainability set tracking and the relevance metric.
- [`src/reduction.rs`](src/reduction.rs): Asymptotic limit checking ($T_{\text{new}} \to T_{\text{prior}}$).
- [`src/verifier.rs`](src/verifier.rs): Test harness that runs theories against benchmarks and outputs failure diagnostics.
- [`src/theories/`](src/theories/): Example models (Newtonian, Special Relativity, Maxwell, Thermodynamics, General Relativity, Quantum, and a flawed test model).

---

## Quick Start

### Run the Verification Demo
```powershell
cargo run
```

### Run the Test Suite
```powershell
cargo test
```

---

## Limitations

- **Prototype Scope:** Covers a curated set of 11 benchmark measurements and simplified analytical models to demonstrate the workflow.
- **Approximations:** Uses closed-form approximations (e.g., 1PN orbital corrections, ideal gas laws) rather than general numerical field solvers.
- **Early Stage:** Intended as a starting point for discussion on how automated testing and formal specification could assist theoretical physics.
