# absagl

A personal Rust crate for experimenting with abstract algebra concepts, including groups, rings, and related structures.

# 🚧 Status

**Work in progress** — This crate is under active development and not yet ready for production use. To ensure mathematical correctness, each module is thoroughly tested.

# ✨ Features

- Finite group and group element abstractions
- Common group implementations (cyclic, permutation, dihedral, etc.)
- Group operations, including subgroup and normal subgroup generation
- Homomorphism constructed from a user-provided closure; verifies structural validity of the mapping.
- Cosets with enumeration; factor groups via coset partitions
- Abelian group decomposition (e.g., direct products of cyclic groups)
- Finite rings with modulo coefficients

# ⚙️ Usage

Clone and use locally for experimentation:

```
git clone https://github.com/2vincentLin/absagl
```

# 🧪 Examples

See `mod test` sections in each module for usage examples and test coverage.

# 🎯 Goals

- Deepen understanding of abstract algebra through Rust
- Create reusable algebraic structures for future projects

# 🛣️ Roadmap

Planned features and modules in development:

- [ ] **Matrix Rings** — Support for matrix ring constructions over finite rings or groups, enabling noncommutative ring exploration
- [ ] **Polynomial Rings** — Implementation of univariate and multivariate polynomial rings, with evaluation and modular arithmetic
- [ ] **Field Extensions** — (Optional) Basic support for quotient fields and extension fields for richer algebraic structures
- [ ] **General Ring Homomorphisms** — Mappings and structure-preserving functions across rings and fields
- [ ] **Improved Testing & Examples** — More targeted tests for edge cases, structural properties, and algebraic identities

These features push into more abstract and computationally demanding territory, so I’m prioritizing mathematical correctness and modular design throughout.


# 📄 License

MIT

_This is a toy project for learning and fun! Contributions, suggestions, or just nerdy banter are always welcome._