# PPVS: Physical Principles Verification System

> **"Formalizing Physics into an Executable Specification & Backward-Compatibility Test Harness."**

---

## 1. Philosophy & Epistemology

In mathematics, proof assistants like **Lean** and **Coq** treat mathematical truth as type-checking: propositions are types, and proofs are terms that inhabit them.

**PPVS** establishes an analogous formal foundation for **Physics**:
1. **Explainability Mapping $E(t)$**:
   Let $\mathcal{P}$ be the set of empirically observed phenomena and $\mathcal{T}$ the space of theories.
   The explainability mapping:
   $$E: \mathcal{T} \to \mathcal{P}(\mathcal{P})$$
   assigns to each theory $t$ the set of phenomena that $t$ successfully predicts within experimental tolerance.
2. **Theory Progression Algebra**:
   If an established prior theory $t_1$ explains phenomenon set $p_1 = E(t_1)$, then any valid successor candidate $t_2$ must satisfy:
   $$E(t_2) = p_1 \cup u$$
   where $u = E(t_2) \setminus E(t_1) \neq \emptyset$ is the **empirical innovation** of $t_2$, and:
   $$\text{Regressions} = E(t_1) \setminus E(t_2) = \emptyset$$
   *Any new theory cannot afford to break existing phenomena data and hence the existing theories that explain them.*
3. **The Normalized Relevance Metric**:
   The scientific relevance / epistemic efficiency of any theory across the whole empirical registry is:
   $$\text{Relevance } R(t) = \frac{\text{phenomena explained}}{\text{total known phenomena} \times \text{assumptions made}} = \frac{|E(t)|}{|\mathcal{P}_{\text{total}}| \times |A(t)|} = \frac{\text{Coverage}(t)}{|A(t)|}$$
   Bounded in $[0, 1]$, where $1.0$ represents a single-axiom Theory of Everything explaining 100% of all known phenomena. A progressive successor must increase relevance: $R(t_2) > R(t_1)$.
4. **The Correspondence Principle**:
   $$\lim_{\epsilon \to 0} T_{\text{new}}(x, \epsilon) = T_{\text{prior}}(x)$$
   A candidate theory must asymptotically reduce to prior theories in their established limits ($c \to \infty$, $\hbar \to 0$, $G \to 0$).

---

## 2. Core Architecture

```
┌────────────────────────────────────────────────────────┐
│            4. Compatibility & Correspondence           │
│   (Asymptotic Limits: T_new → T_prior as ε → 0)        │
├────────────────────────────────────────────────────────┤
│           3. Empirical Phenomenon Registry             │
│   (Regression Test Suite: Orbits, Spectra, Lifetimes)  │
├────────────────────────────────────────────────────────┤
│             2. Formal Physical Theories                │
│   (Axioms, Symmetries, Lagrangians, State Spaces)      │
├────────────────────────────────────────────────────────┤
│      1. Type System, Dimensions & Manifolds            │
│   (Dimensional Safety, Tensor Ranks, Coordinates)      │
└────────────────────────────────────────────────────────┘
```

### Module Overview

- [`ppvs::dimensions`](src/dimensions.rs): SI base dimensions ($M, L, T, I, \Theta, N, J$). Compile-time and runtime dimensional safety. Dimensional mismatch errors prevent unphysical operations.
- [`ppvs::empirical`](src/empirical.rs): The Empirical Phenomena Registry containing benchmark measurements across all physics domains (CODATA $G$, Keplerian orbits, CERN 1977 muon dilation, Bertozzi relativistic momentum, Mercury perihelion precession, Maxwell wave speed, Coulomb force, Ideal Gas molar volume, Carnot cycle efficiency, Photoelectric stopping potential, Balmer hydrogen emission).
- [`ppvs::theory`](src/theory.rs): Formal `Theory` trait declaring principles, symmetries ($SO(3)$, $SO(1,3)$, Galilei group, Diff($\mathcal{M}$), $U(1)$ gauge), validity regimes, and deterministic predictions.
- [`ppvs::established`](src/established.rs): The Established Theories Registry holding the foundational pillars of physics (Newtonian Mechanics, Special Relativity, Maxwellian Electrodynamics, Classical Thermodynamics, Quantum Mechanics), and the multi-theory `CompatibilityAudit` engine.
- [`ppvs::explainability`](src/explainability.rs): Set-theoretic explainability $E(t)$, normalized relevance metric $R(t)$, theory progression, and grand unification analysis.
- [`ppvs::reduction`](src/reduction.rs): Asymptotic correspondence solver and backward-compatibility regression checker.
- [`ppvs::verifier`](src/verifier.rs): Executable verification engine producing deterministic diagnostic failure reports.
- [`ppvs::theories`](src/theories/):
  - `NewtonianMechanics`: Classical gravity, Keplerian orbits, Galilean relativity.
  - `SpecialRelativity`: Flat spacetime Lorentz boost, time dilation, relativistic momentum.
  - `MaxwellianElectrodynamics`: Maxwell's field equations, wave speed $c = 1/\sqrt{\epsilon_0 \mu_0}$, Coulomb force.
  - `ClassicalThermodynamics`: Four thermodynamic laws, ideal gas equation of state, Carnot efficiency limit.
  - `QuantumMechanics`: Wave-particle duality, photoelectric effect, Balmer hydrogen spectral series.
  - `GeneralRelativity`: Spacetime curvature, 1PN orbital dynamics, Mercury precession excess ($42.98''/\text{century}$).
  - `FlawedMomentumTheory`: Candidate theory with missing relativistic factors demonstrating diagnostic failure reporting.
- [`ppvs::theories`](src/theories/):
  - `NewtonianMechanics`: Classical gravity, Keplerian orbits, Galilean relativity.
  - `SpecialRelativity`: Flat spacetime Lorentz boost, time dilation, relativistic momentum.
  - `GeneralRelativity`: Spacetime curvature, 1PN orbital dynamics, Mercury perihelion precession.
  - `FlawedMomentumTheory`: Negative test candidate demonstrating strict rejection by PPVS.

---

## 3. Running Verification & Tests

### Execute the PPVS Specification
```powershell
cargo run
```

### Run Deterministic Test Suite
```powershell
cargo test
```
All 14 unit and integration tests run deterministically across:
- Dimensional arithmetic & error safety
- Empirical registry & uncertainty z-score calculations
- Theory predictions against empirical benchmarks
- Asymptotic correspondence limits ($v/c \to 0$)
- Backward-compatibility regression prevention
- Strict rejection of broken candidate theories
