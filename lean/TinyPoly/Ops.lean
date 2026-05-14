import TinyPoly.Poly

namespace TinyPoly

def sum (p q : Poly) : Poly :=
  let l := p.positions.map fun pos => { pos with name := s!"L.{pos.name}" }
  let r := q.positions.map fun pos => { pos with name := s!"R.{pos.name}" }
  { positions := l ++ r }

def tensor (p q : Poly) : Poly := Id.run do
  let mut positions : Array Position := #[]
  for pp in p.positions do
    for qq in q.positions do
      let name := s!"({pp.name},{qq.name})"
      let mut dirs : Array String := #[]
      for dp in pp.dirs do
        for dq in qq.dirs do
          dirs := dirs.push s!"{dp}.{dq}"
      positions := positions.push { name, dirs }
  return { positions }

-- All functions {0..k-1} -> {0..n-1} as length-k index arrays.
private def cartesianPower (n k : Nat) : Array (Array Nat) := Id.run do
  if k == 0 then return #[#[]]
  if n == 0 then return #[]
  let mut out : Array (Array Nat) := #[#[]]
  for _ in [:k] do
    let mut next : Array (Array Nat) := #[]
    for v in out do
      for i in [:n] do
        next := next.push (v.push i)
    out := next
  return out

-- Composition P <| Q: positions are (p, choice: P[p] -> Q(1)); directions are
-- pairs (d, e) with d in P[p] and e in Q[choice(d)].
def compose (p q : Poly) : Poly := Id.run do
  let nq := q.npositions
  let mut positions : Array Position := #[]
  for pp in p.positions do
    let k := pp.arity
    for choice in cartesianPower nq k do
      let chosenNames := choice.map fun j => (q.positions[j]!).name
      let name :=
        if k == 0 then pp.name
        else s!"{pp.name}<{String.intercalate "," chosenNames.toList}>"
      let mut dirs : Array String := #[]
      for dIdx in [:k] do
        let dName := pp.dirs[dIdx]!
        for eName in (q.positions[choice[dIdx]!]!).dirs do
          dirs := dirs.push s!"{dName}.{eName}"
      positions := positions.push { name, dirs }
  return { positions }

infixl:65 " ⊕ " => sum
infixl:70 " ⊗ " => tensor
infixr:75 " ◁ " => compose

end TinyPoly
