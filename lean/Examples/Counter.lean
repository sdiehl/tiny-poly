/-
A counter that ticks through 0, 1, 2, ... mod N.

This is the smallest Moore machine that does anything interesting: one
button (tick), one display (the current number). What it shows in the
polynomial framing is that the state space and the interface can be
completely different shapes. Here the state space is "all the numbers
from 0 to N-1" (N positions, each leading to any other), while the
interface is "one display, one button" (one position with one
direction). The Moore machine is the rule that takes "what state am I
in" and translates it down to "what does the user see, and what does
the user's input do".

Spivak & Niu, Example 4.6 (Counter), p.85.
-/

import TinyPoly

open TinyPoly

def N : Nat := 8

def counter : Moore :=
  let states := (Array.range N).map toString
  let outputs := states
  let readout := Array.range N
  let update := (Array.range N).map fun i => #[(i + 1) % N]
  { states, inputs := #["tick"], outputs, readout, update }

def main : IO Unit := do
  IO.println s!"state polynomial    S y^S = {counter.statePoly.showAggregate}"
  IO.println s!"interface polynomial B y^A = {counter.interfacePoly.showAggregate}"

  let trace := counter.run 0 (Array.replicate 10 0)
  IO.println ""
  IO.println s!"ticks  : {String.intercalate " " (List.replicate 10 "tick")}"
  IO.println s!"trace  : {counter.showTrace trace}"
