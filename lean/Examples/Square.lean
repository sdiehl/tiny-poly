/-
The walking commutative square: the smallest category that is not free.

       h
  w ------> y
  |         |
  | f       | k
  v         v
  x ------> z
       g

  f # g  =  h # k

Four objects, six "named" arrows (id_w, id_x, id_y, id_z, f, g, h, k),
and the rule that the two paths around the square give the same arrow
w -> z. Adding that single morphism (call it fg = hk) gives nine
morphisms total. Going round the long way and going round the short
way have to land at the same place; that is what "commutative" means.

Why bother? Because the library's checkAxioms exists exactly to catch
the moment when "I thought I wrote down a category but I forgot a
composite" turns into "this is not a category at all". For a freely
generated category the check is trivial; for one with relations like
this square it actually has work to do.

Spivak & Niu, Example 7.68 (Retrofunctors to the walking commutative
square), p.260.
-/

import TinyPoly

open TinyPoly

def square : Category :=
  { objects := #["w", "x", "y", "z"]
    morphisms := #[
      #[
        { name := "id_w", target := 0 },
        { name := "f",    target := 1 },
        { name := "h",    target := 2 },
        { name := "fg",   target := 3 }
      ],
      #[
        { name := "id_x", target := 1 },
        { name := "g",    target := 3 }
      ],
      #[
        { name := "id_y", target := 2 },
        { name := "k",    target := 3 }
      ],
      #[{ name := "id_z", target := 3 }]
    ]
    table := #[
      #[#[0, 1, 2, 3], #[1, 3], #[2, 3], #[3]],
      #[#[0, 1], #[1]],
      #[#[0, 1], #[1]],
      #[#[0]]
    ] }

def main : IO Unit := do
  match square.checkAxioms with
  | .ok _ => pure ()
  | .error e => IO.println s!"square check failed: {e}"

  let c := square.comonoid
  IO.println s!"polynomial P_C = {c.poly.showNamed}"
  IO.println ""
  IO.println "epsilon : P_C -> y"
  IO.print c.epsilon.pretty
  IO.println "delta   : P_C -> P_C ◁ P_C"
  IO.print c.delta.pretty

  let nMorph := square.morphisms.foldl (fun acc ms => acc + ms.size) 0
  IO.println "derived facts:"
  IO.println s!"  {square.objects.size} objects, {nMorph} morphisms total ({nMorph - square.objects.size} non-identity)"
  IO.println "  the long arrow w -> z is named fg, equal to h # k by the square relation"
