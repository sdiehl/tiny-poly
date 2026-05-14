import TinyPoly.Poly
import TinyPoly.Ops
import TinyPoly.Lens

namespace TinyPoly

structure Morphism where
  name : String
  target : Nat
  deriving Repr, Inhabited

-- A small category: objects, morphisms out of each object (index 0 = identity),
-- and a composition table table[x][f][g] = index in M_x of "first f, then g".
structure Category where
  objects : Array String
  morphisms : Array (Array Morphism)
  table : Array (Array (Array Nat))
  deriving Repr, Inhabited

structure Comonoid where
  poly : Poly
  epsilon : Lens
  delta : Lens
  deriving Repr, Inhabited

namespace Category

def poly (c : Category) : Poly :=
  let positions : Array Position := c.objects.zipWith
    (fun obj ms => { name := obj, dirs := ms.map (·.name) }) c.morphisms
  { positions }

def epsilon (c : Category) : Lens :=
  let p := c.poly
  let y := Poly.y
  let onPos := (Array.range p.npositions).map (fun _ => 0)
  let onDir : Array (Array Nat) := (Array.range p.npositions).map (fun _ => #[0])
  Lens.mk! p y onPos onDir

private def powNat (base exp : Nat) : Nat := Id.run do
  let mut r := 1
  for _ in [:exp] do r := r * base
  return r

private def positionIndex (c : Category) (n objIdx : Nat) (choice : Array Nat) : Nat := Id.run do
  let mut idx := 0
  for prevObj in [:objIdx] do
    let a := (c.morphisms[prevObj]!).size
    idx := idx + powNat n a
  let a := (c.morphisms[objIdx]!).size
  let mut offset := 0
  for k in [:choice.size] do
    offset := offset + choice[k]! * powNat n (a - 1 - k)
  return idx + offset

def delta (c : Category) : Lens := Id.run do
  let p := c.poly
  let pp := compose p p
  let n := c.objects.size

  let mut onPos : Array Nat := #[]
  let mut onDir : Array (Array Nat) := #[]
  for x in [:c.morphisms.size] do
    let ms := c.morphisms[x]!
    let choice := ms.map (·.target)
    onPos := onPos.push (positionIndex c n x choice)
    let mut dirs : Array Nat := #[]
    for fIdx in [:ms.size] do
      let mF := ms[fIdx]!
      let yObj := mF.target
      for gIdx in [:(c.morphisms[yObj]!).size] do
        let composite := ((c.table[x]!)[fIdx]!)[gIdx]!
        dirs := dirs.push composite
    onDir := onDir.push dirs
  return Lens.mk! p pp onPos onDir

def comonoid (c : Category) : Comonoid :=
  { poly := c.poly, epsilon := c.epsilon, delta := c.delta }

def checkAxioms (c : Category) : Except String Unit := do
  for x in [:c.morphisms.size] do
    let ms := c.morphisms[x]!
    let idY := 0
    for fIdx in [:ms.size] do
      let f := ms[fIdx]!
      if ((c.table[x]!)[fIdx]!)[idY]! != fIdx then
        throw s!"right unit fails at {c.objects[x]!}.{f.name}"
    let idX := 0
    for gIdx in [:ms.size] do
      if ((c.table[x]!)[idX]!)[gIdx]! != gIdx then
        throw s!"left unit fails at {c.objects[x]!}"
  for x in [:c.morphisms.size] do
    let ms := c.morphisms[x]!
    for fIdx in [:ms.size] do
      let mF := ms[fIdx]!
      let y := mF.target
      for gIdx in [:(c.morphisms[y]!).size] do
        let mG := (c.morphisms[y]!)[gIdx]!
        let z := mG.target
        for hIdx in [:(c.morphisms[z]!).size] do
          let gf := ((c.table[x]!)[fIdx]!)[gIdx]!
          let hgfLeft := ((c.table[x]!)[gf]!)[hIdx]!
          let hg := ((c.table[y]!)[gIdx]!)[hIdx]!
          let hgfRight := ((c.table[x]!)[fIdx]!)[hg]!
          if hgfLeft != hgfRight then
            throw s!"associativity fails at {c.objects[x]!}.{mF.name}.{mG.name}"

end Category

end TinyPoly
