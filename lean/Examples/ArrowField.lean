/-
An arrow field on a small category.

Take any category C. An arrow field on C is a choice of one outgoing
arrow per object. That's it. If you stand at object x, the field tells
you which arrow to take; if you then stand at the object that arrow
led to, the field tells you the next arrow; and so on. So a field
gives you a way to "walk" through C, one step per click. This is
exactly the discrete analogue of a vector field on a manifold, where
the manifold is replaced by C and the vectors by arrows.

We illustrate on a 4-object linear poset {0, 1, 2, 3} with i -> j
whenever i <= j. There are 4 + 3 + 2 + 1 = 10 arrows in total. We
pick a field that always jumps "forward by one if possible, else stay"
and iterate it from object 0.

The punchline of Spivak & Niu's framing: an arrow field is the same
data as a retrofunctor `C -> y^N` from C to the natural-number monoid.
The retrofunctor laws (preserve identities, preserve composition)
recover "zero steps is do nothing" and "a then b is the same as a+b
steps", with no further bookkeeping required.

Spivak & Niu, Example 7.71 (Arrow fields), p.262.
-/

import TinyPoly

open TinyPoly

def linearPoset (n : Nat) : Category :=
  let objects : Array String := (Array.range n).map toString
  let morphisms : Array (Array Morphism) :=
    (Array.range n).map fun i =>
      (Array.range (n - i)).map fun k =>
        let name := if k == 0 then s!"id_{i}" else s!"e_{i}_{i + k}"
        { name, target := i + k }
  let table : Array (Array (Array Nat)) :=
    (Array.range n).map fun i =>
      let mi := morphisms[i]!
      (Array.range mi.size).map fun f =>
        let mid := (mi[f]!).target
        let mMid := morphisms[mid]!
        (Array.range mMid.size).map fun g => f + g
  { objects, morphisms, table }

def main : IO Unit := do
  let c := linearPoset 4
  match c.checkAxioms with
  | .ok _ => pure ()
  | .error e => IO.println s!"poset check failed: {e}"

  let field : Array Nat :=
    (Array.range c.objects.size).map fun i =>
      if (c.morphisms[i]!).size > 1 then 1 else 0

  IO.println s!"category : {c.objects.size}-element linear poset"
  IO.println "field    :"
  for i in [:c.objects.size] do
    let m := (c.morphisms[i]!)[field[i]!]!
    IO.println s!"  at {c.objects[i]!} take {m.name} (-> {c.objects[m.target]!})"

  IO.println ""
  IO.println "walking from 0, applying the field N times:"
  let mut cursor : Nat := 0
  IO.println s!"  step 0: at {c.objects[cursor]!}"
  for step in [1:7] do
    let m := (c.morphisms[cursor]!)[field[cursor]!]!
    cursor := m.target
    IO.println s!"  step {step}: take {m.name} -> {c.objects[cursor]!}"

  let a : Nat := 2
  let b : Nat := 3
  let mut left : Nat := 0
  for _ in [:a + b] do
    left := ((c.morphisms[left]!)[field[left]!]!).target
  let mut right : Nat := 0
  for _ in [:a] do
    right := ((c.morphisms[right]!)[field[right]!]!).target
  for _ in [:b] do
    right := ((c.morphisms[right]!)[field[right]!]!).target
  IO.println ""
  IO.println s!"retrofunctor law (composition) for a={a}, b={b}:"
  IO.println s!"  walk {a + b} steps      lands at {c.objects[left]!}"
  IO.println s!"  walk {a} then {b} steps lands at {c.objects[right]!}"
  if left == right then
    IO.println "  ok"
  else
    IO.println "  MISMATCH"
