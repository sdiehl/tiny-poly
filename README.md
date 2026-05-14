# tiny-poly

A WIP implementation of polynomial functors as a category, in the style of David Spivak and Nelson Niu's _Polynomial Functors: A Mathematical Theory of Interaction_. Two parallel implementations live here: [`rust/`](rust/) and [`lean/`](lean/).

- `Poly` and `Position` in standard form: a polynomial as a list of positions, each with a list of named directions.
- The three core operations: sum (`P + Q`), parallel tensor (`P (x) Q`, the Dirichlet product), and composition (`P <| Q`). In Lean these are written `⊕`, `⊗`, `◁`.
- `Lens`: the morphisms of Poly. A pair `(forward on positions, backward on directions)` with identity, composition, and structural validation.
- `Moore`: Moore machines as lenses `S y^S -> B y^A`. Parity and traffic-light examples; a `run` method that produces state/output traces.
- `wiring`: `parallel` and `series` combinators that build a single Moore machine from two, demonstrating how wiring diagrams are just lenses between tensors.
- `Category` and `Comonoid`: the Ahman-Uustalu / Garner punchline. Any small category gives a comonoid `(P, epsilon, delta)` in `(Poly, <|)`. The walking arrow, `Z/2` as a one-object category, and the discrete category are built in.

```bash
cd rust && cargo build && cargo test && cargo run --example demo
cd lean && lake build && lake exe demo
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
