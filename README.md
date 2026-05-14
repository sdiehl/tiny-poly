# tiny-poly

A WIP implementation in the style of David Spivak and Nelson Niu's _Polynomial Functors: A Mathematical Theory of Interaction_. Two parallel implementations live here: [`rust/`](rust/) and [`lean/`](lean/).

tldr; Polynomial functors are a way to make state machines, pipelines, and workflows snap together like Lego. They give us a single algebraic language for interactive systems where "wiring two things together" is just composition in a category, and the category laws fall out as theorems instead of conventions.

- `Poly` and `Position` in standard form: a polynomial as a list of positions, each with a list of named directions.
- The three core operations: sum (`P + Q`), parallel tensor (`P (x) Q`, the Dirichlet product), and composition (`P <| Q`). In Lean these are written `⊕`, `⊗`, `◁`.
- `Lens`: the morphisms of Poly. A pair `(forward on positions, backward on directions)` with identity, composition, and structural validation.
- `Moore`: Moore machines as lenses `S y^S -> B y^A`. A `run` method produces state/output traces.
- `wiring`: `parallel` and `series` combinators that build a single Moore machine from two, demonstrating how wiring diagrams are just lenses between tensors.
- `Category` and `Comonoid`: the Ahman-Uustalu / Garner punchline. Any small category gives a comonoid `(P, epsilon, delta)` in `(Poly, <|)`.

```bash
cd rust
cargo build
cargo test
cargo run --example demo
cargo run --example vending
cargo run --example pipeline
cargo run --example workflow
```

```bash
cd lean
lake build
lake exe tests
lake exe demo
lake exe vending
lake exe pipeline
lake exe workflow
```

The worked examples (parity, traffic light, walking arrow, Z/2, discrete category, vending machine, pipeline, PR workflow) live in the `examples/` directory of each subproject.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
