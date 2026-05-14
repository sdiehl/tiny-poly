/-
A deterministic finite automaton, recognising "ends in 01".

You feed it a string of 0s and 1s, one symbol at a time. After each
symbol it shines a green light (accept) if everything seen so far ends
in 01, otherwise a red light (reject). Three states are enough to keep
track: q0 = haven't seen anything useful yet, q1 = just saw a 0, q2 =
just saw a 0 then a 1.

What the polynomial framing buys us: the automaton is a lens
`S y^S -> 2 y^A`. The 2 in the codomain is literally the set
{reject, accept}: the same shape we'd use for any yes/no output, and
the same shape that lets us later compose this DFA with anything else
expecting a 2-output.

Spivak & Niu, Section 4.1.1 (Deterministic state automata), p.88.
-/

import TinyPoly

open TinyPoly

def dfa : Moore :=
  { states  := #["q0", "q1", "q2"]
    inputs  := #["0", "1"]
    outputs := #["reject", "accept"]
    readout := #[0, 0, 1]
    update  := #[
      #[1, 0],
      #[1, 2],
      #[1, 0]
    ] }

def words : List (Array Nat) :=
  [#[0, 1], #[1, 0, 1], #[0, 0, 1], #[1, 1, 0], #[0, 1, 0, 1, 1]]

def main : IO Unit := do
  IO.println s!"state polynomial    S y^S = {dfa.statePoly.showAggregate}"
  IO.println s!"interface polynomial B y^A = {dfa.interfacePoly.showAggregate}"

  for w in words do
    let trace := dfa.run 0 w
    let (finalState, _) := trace[trace.size - 1]!
    let label := dfa.outputs[dfa.readout[finalState]!]!
    let s := String.intercalate "" (w.toList.map toString)
    IO.println ""
    IO.println s!"input  : {s}"
    IO.println s!"trace  : {dfa.showTrace trace}"
    IO.println s!"verdict: {label}"
