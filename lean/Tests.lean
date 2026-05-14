import TinyPoly

namespace TinyPoly.Tests

open TinyPoly

private def smallP : Poly := { positions := #[
  Position.ofArity "a" 2,
  Position.ofArity "b" 1,
  Position.ofArity "c" 0
] }

private def smallQ : Poly := { positions := #[
  Position.ofArity "x" 1,
  Position.ofArity "y" 0
] }

private def parity : Moore :=
  { states := #["even", "odd"]
    inputs := #["0", "1"]
    outputs := #["0", "1"]
    readout := #[0, 1]
    update := #[#[0, 1], #[1, 0]] }

private def walkingArrow : Category :=
  { objects := #["a", "b"]
    morphisms := #[
      #[{ name := "id_a", target := 0 }, { name := "f", target := 1 }],
      #[{ name := "id_b", target := 1 }]
    ]
    table := #[
      #[#[0, 1], #[1]],
      #[#[0]]
    ] }

private def monoidZ2 : Category :=
  { objects := #["*"]
    morphisms := #[#[{ name := "e", target := 0 }, { name := "t", target := 0 }]]
    table := #[#[#[0, 1], #[1, 0]]] }

private def discrete (objs : Array String) : Category :=
  { objects := objs
    morphisms := objs.mapIdx fun i o => #[{ name := s!"id_{o}", target := i }]
    table := objs.map (fun _ => #[#[0]]) }

#guard Poly.zero.showAggregate == "0"
#guard Poly.one.showAggregate == "1"
#guard Poly.y.showAggregate == "y"
#guard smallP.showAggregate == "y^2 + y + 1"
#guard smallQ.showAggregate == "y + 1"

#guard (smallP ⊕ smallQ).showAggregate == "y^2 + 2y + 2"
#guard (smallP ⊗ smallQ).showAggregate == "y^2 + y + 4"
#guard (smallP ◁ smallQ).showAggregate == "y^2 + 3y + 3"

-- P ◁ y is structurally P (identity on the right).
#guard (smallP ◁ Poly.y).showAggregate == smallP.showAggregate

#guard (Lens.id smallP).onPos == #[0, 1, 2]

private def parityRun : Array (Nat × Nat) := parity.run 0 #[1, 1, 0, 1, 0, 1]
#guard parity.showTrace parityRun == "even/0 -> odd/1 -> even/0 -> even/0 -> odd/1 -> odd/1 -> even/0"

#guard walkingArrow.checkAxioms.toOption.isSome
#guard monoidZ2.checkAxioms.toOption.isSome
#guard (discrete #["x", "y", "z"]).checkAxioms.toOption.isSome

private def wArr := walkingArrow.comonoid
#guard wArr.epsilon.onDir == #[#[0], #[0]]

end TinyPoly.Tests

def main : IO Unit := IO.println "tests ok"
