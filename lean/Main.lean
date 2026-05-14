import TinyPoly

open TinyPoly

def banner (title : String) : IO Unit := do
  IO.println ""
  IO.println s!"== {title} =="

def parity : Moore :=
  { states := #["even", "odd"]
    inputs := #["0", "1"]
    outputs := #["0", "1"]
    readout := #[0, 1]
    update := #[#[0, 1], #[1, 0]] }

def walkingArrow : Category :=
  { objects := #["a", "b"]
    morphisms := #[
      #[{ name := "id_a", target := 0 }, { name := "f", target := 1 }],
      #[{ name := "id_b", target := 1 }]
    ]
    table := #[
      #[#[0, 1], #[1]],
      #[#[0]]
    ] }

def main : IO Unit := do
  banner "polynomials"
  let p : Poly := { positions := #[
    Position.ofArity "a" 2,
    Position.ofArity "b" 1,
    Position.ofArity "c" 0
  ] }
  let q : Poly := { positions := #[
    Position.ofArity "x" 1,
    Position.ofArity "y" 0
  ] }
  IO.println s!"P = {p}"
  IO.println s!"Q = {q}"

  banner "operations"
  IO.println s!"P ⊕ Q = {p ⊕ q}"
  IO.println s!"P ⊗ Q = {p ⊗ q}"
  IO.println s!"P ◁ Q = {p ◁ q}"

  banner "Moore parity machine"
  let trace := parity.run 0 #[1, 1, 0, 1, 0, 1]
  IO.println "inputs : 1 1 0 1 0 1"
  IO.println s!"trace  : {parity.showTrace trace}"

  banner "wiring (series)"
  let wired := series! parity parity
  let trace := wired.run 0 #[1, 1, 1]
  IO.println s!"trace  : {wired.showTrace trace}"

  banner "comonoids are categories"
  match walkingArrow.checkAxioms with
  | .ok _ => pure ()
  | .error e => IO.println s!"category check failed: {e}"
  let c := walkingArrow.comonoid
  IO.println s!"P(C)        = {c.poly.showNamed}"
  IO.println "epsilon:"
  IO.print c.epsilon.pretty
  IO.println "delta:"
  IO.print c.delta.pretty
