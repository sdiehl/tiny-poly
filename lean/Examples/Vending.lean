/-
Vending machine as a state machine.

This is the toy example everyone draws on a whiteboard, but the point is
what the polynomial framing reveals: there are two completely separate
"shapes" hiding inside any state machine. The internal shape says
"from any state I can move to any state"; the external shape says
"at any output I might receive any input". Treating these as two
polynomials and the machine itself as a structure-preserving map
between them is what lets us later wire machines together like Lego
blocks (see Examples/Pipeline.lean) without writing glue code.
-/

import TinyPoly

open TinyPoly

def vending : Moore :=
  { states  := #["0c", "5c", "10c", "15c", "20c", "READY"]
    inputs  := #["nickel", "dime", "select", "refund"]
    outputs := #["0c", "5c", "10c", "15c", "20c", "READY"]
    readout := #[0, 1, 2, 3, 4, 5]
    update  := #[
      #[1, 2, 0, 0],
      #[2, 3, 1, 0],
      #[3, 4, 2, 0],
      #[4, 5, 3, 0],
      #[5, 5, 4, 0],
      #[5, 5, 0, 0]
    ] }

def session : Array String := #["nickel", "dime", "dime", "select", "nickel", "refund"]

def main : IO Unit := do
  IO.println s!"state polynomial    S y^S = {vending.statePoly.showAggregate}"
  IO.println s!"interface polynomial B y^A = {vending.interfacePoly.showAggregate}"

  let inputs : Array Nat := session.map fun name =>
    (vending.inputs.findIdx? (· == name)).getD 0
  let trace := vending.run 0 inputs

  IO.println ""
  IO.println s!"inputs : {String.intercalate " " session.toList}"
  IO.println s!"trace  : {vending.showTrace trace}"

  let lens := vending.asLens
  IO.println ""
  IO.println s!"lens shape: {lens.src.npositions} positions on the left, {lens.tgt.npositions} on the right"
