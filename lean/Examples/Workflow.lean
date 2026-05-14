/-
A PR review workflow, with all the rules made explicit.

  Draft  --submit-->  InReview  --merge-->  Merged
  `---------- submit_then_merge ---------->'

We list the states (Draft, InReview, Merged) and the allowed
transitions, including the composite "submit and then merge". The
library checks that the bookkeeping is consistent: doing two steps in
a row and then a third should give the same result as doing the first
step and then a combined "second-and-third" step. That sanity check is
the same thing as checking the workflow really is a workflow.

Why bother? Because once a workflow lines up like this, the library can
hand back a `comonoid` for it: a compact algebraic description that
other tools can plug into without re-deriving the state diagram. It is
the same trick a parser generator pulls when it takes your grammar and
hands you a parse table.
-/

import TinyPoly

open TinyPoly

def workflow : Category :=
  { objects := #["Draft", "InReview", "Merged"]
    morphisms := #[
      #[
        { name := "id_Draft", target := 0 },
        { name := "submit", target := 1 },
        { name := "submit_then_merge", target := 2 }
      ],
      #[
        { name := "id_InReview", target := 1 },
        { name := "merge", target := 2 }
      ],
      #[{ name := "id_Merged", target := 2 }]
    ]
    table := #[
      #[#[0, 1, 2], #[1, 2], #[2]],
      #[#[0, 1], #[1]],
      #[#[0]]
    ] }

def main : IO Unit := do
  match workflow.checkAxioms with
  | .ok _ => pure ()
  | .error e => IO.println s!"workflow check failed: {e}"

  let c := workflow.comonoid
  IO.println s!"polynomial P_C = {c.poly.showNamed}"
  IO.println ""
  IO.println "epsilon : P_C -> y"
  IO.print c.epsilon.pretty
  IO.println "delta   : P_C -> P_C ◁ P_C"
  IO.print c.delta.pretty

  IO.println "derived facts:"
  let nMorph := workflow.morphisms.foldl (fun acc ms => acc + ms.size) 0
  IO.println s!"  {workflow.objects.size} objects, {nMorph} morphisms total, {nMorph - workflow.objects.size} of them composites"
