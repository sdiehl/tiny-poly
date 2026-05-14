# tiny-poly

A WIP implementation in the style of David Spivak and Nelson Niu's [_Polynomial Functors: A Mathematical Theory of Interaction_](https://topos.site/poly-book.pdf). Two parallel implementations live here: [`rust/`](rust/) and [`lean/`](lean/). The companion [video lectures](https://www.youtube.com/playlist?list=PLhgq-BqyZ7i6IjU82EDzCqgERKjjIPlmh) from the Topos Institute walk through the same material chapter by chapter.

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
cargo run --example counter
cargo run --example dfa
cargo run --example file_reader
cargo run --example vending
cargo run --example pipeline
cargo run --example workflow
cargo run --example square
cargo run --example streams
cargo run --example arrow_field
cargo run --example life
```

```bash
cd lean
lake build
lake exe tests
lake exe demo
lake exe counter
lake exe dfa
lake exe file_reader
lake exe vending
lake exe pipeline
lake exe workflow
lake exe square
lake exe streams
lake exe arrow_field
lake exe life
```

The worked examples live in the `examples/` directory of each subproject and pair small concrete machines with the book sections they come from:

- `counter`, `dfa`, `file_reader`, `vending` (Chapter 4) build Moore machines and read off their interface polynomials.
- `pipeline`, `workflow`, `life` (Chapter 4) wire smaller machines together with `series`, `parallel`, and a closed Conway grid.
- `square`, `streams`, `arrow_field` (Chapter 7) cross over to comonoids: a walking commutative square, a cyclic shift action on three streams, and a retrofunctor that picks a "next step" at every object.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
