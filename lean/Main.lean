import TinyPoly

open TinyPoly

def banner (title : String) : IO Unit := do
  IO.println ""
  IO.println s!"== {title} =="

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
  let parity := Moore.parity
  let trace := parity.run 0 #[1, 1, 0, 1, 0, 1]
  IO.println "inputs : 1 1 0 1 0 1"
  IO.println s!"trace  : {parity.showTrace trace}"

  banner "wiring (series)"
  let wired := series! parity parity
  let trace := wired.run 0 #[1, 1, 1]
  IO.println s!"trace  : {wired.showTrace trace}"

  banner "comonoids are categories"
  let cat := Category.walkingArrow
  match cat.checkAxioms with
  | .ok _ => pure ()
  | .error e => IO.println s!"category check failed: {e}"
  let c := cat.comonoid
  IO.println s!"P(C)        = {c.poly.showNamed}"
  IO.println "epsilon:"
  IO.print c.epsilon.pretty
  IO.println "delta:"
  IO.print c.delta.pretty
