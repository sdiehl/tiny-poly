/-
B-streams as a category.

A B-stream is an infinite sequence of letters from some alphabet B.
Shifting a stream forward by n positions is an "action" of the natural
numbers on the set of streams, and any action gives a category: the
objects are the streams, and the arrows out of a stream are the things
you can shift it by.

Truly infinite streams won't fit in a finite category, so we tile the
example: take period-3 streams over {0,1} and the orbit of "001"
under shift. That gives three streams (001, 010, 100) and a shift by
{0,1,2} between them, which is exactly the cyclic group Z/3Z acting
on its own three rotations.

What the polynomial framing buys us: this category is a comonoid in
(Poly, ◁). The epsilon arrow reads off "the current letter" of a
stream; the delta arrow says "walking a + b steps is the same as
walking a steps then b steps". Those two laws are all there is to a
stream, categorically speaking.

Spivak & Niu, Example 7.45 (The category of B-streams), p.253, here
quotiented to a finite cyclic action.
-/

import TinyPoly

open TinyPoly

def streams : Array String := #["001", "010", "100"]
def shiftNames : Array String := #["s0", "s1", "s2"]
def n : Nat := streams.size

def streamsCat : Category :=
  let morphisms : Array (Array Morphism) :=
    (Array.range n).map fun i =>
      (Array.range n).map fun k =>
        { name := shiftNames[k]!, target := (i + k) % n }
  let table : Array (Array (Array Nat)) :=
    (Array.range n).map fun _ =>
      (Array.range n).map fun f =>
        (Array.range n).map fun g => (f + g) % n
  { objects := streams, morphisms, table }

def main : IO Unit := do
  match streamsCat.checkAxioms with
  | .ok _ => pure ()
  | .error e => IO.println s!"streams check failed: {e}"

  let c := streamsCat.comonoid
  IO.println s!"polynomial P_C = {c.poly.showNamed}"
  IO.println ""
  IO.println "epsilon (read current position):"
  IO.print c.epsilon.pretty
  IO.println "delta (split one shift into two consecutive shifts):"
  IO.print c.delta.pretty

  IO.println "walking through stream 001 with shifts (1, 1, 1):"
  let mut here : Nat := 0
  IO.println s!"  start  : {streams[here]!}"
  for _ in [:3] do
    here := (here + 1) % n
    IO.println "  shift  : +1"
    IO.println s!"  stream : {streams[here]!}"
