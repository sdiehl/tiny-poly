import TinyPoly.Poly
import TinyPoly.Ops
import TinyPoly.Lens
import TinyPoly.Moore
import TinyPoly.Wiring
import TinyPoly.Comonoid

namespace TinyPoly.Tests

private def smallP : Poly := { positions := #[
  Position.ofArity "a" 2,
  Position.ofArity "b" 1,
  Position.ofArity "c" 0
] }

private def smallQ : Poly := { positions := #[
  Position.ofArity "x" 1,
  Position.ofArity "y" 0
] }

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

private def parityRun : Array (Nat × Nat) := Moore.parity.run 0 #[1, 1, 0, 1, 0, 1]
#guard Moore.parity.showTrace parityRun == "even/0 -> odd/1 -> even/0 -> even/0 -> odd/1 -> odd/1 -> even/0"

#guard (Category.walkingArrow.checkAxioms.toOption.isSome)
#guard (Category.monoidZ2.checkAxioms.toOption.isSome)
#guard (Category.discrete #["x", "y", "z"]).checkAxioms.toOption.isSome

private def wArr := Category.walkingArrow.comonoid
#guard wArr.epsilon.onDir == #[#[0], #[0]]

end TinyPoly.Tests
