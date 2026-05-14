namespace TinyPoly

structure Position where
  name : String
  dirs : Array String
  deriving Repr, BEq, Inhabited

namespace Position

def arity (p : Position) : Nat := p.dirs.size

def ofArity (name : String) (n : Nat) : Position :=
  { name, dirs := (Array.range n).map (fun i => s!"d{i}") }

end Position

structure Poly where
  positions : Array Position
  deriving Repr, BEq, Inhabited

namespace Poly

def zero : Poly := { positions := #[] }

def one : Poly := { positions := #[{ name := "*", dirs := #[] }] }

def y : Poly := { positions := #[{ name := "y", dirs := #["d0"] }] }

def monomial (name : String) (n : Nat) : Poly :=
  { positions := #[Position.ofArity name n] }

def term (count arity : Nat) : Poly :=
  { positions := (Array.range count).map (fun i => Position.ofArity s!"p{i}" arity) }

def npositions (p : Poly) : Nat := p.positions.size

def arity (p : Poly) (i : Nat) : Nat :=
  (p.positions[i]!).arity

-- signature: list of (arity, count) pairs in descending order of arity.
def signature (p : Poly) : Array (Nat × Nat) := Id.run do
  let arities : Array Nat := (p.positions.map Position.arity).qsort (· > ·)
  let mut out : Array (Nat × Nat) := #[]
  for a in arities do
    match out.back? with
    | some (a', c) =>
        if a' == a then out := out.pop.push (a', c + 1)
        else out := out.push (a, 1)
    | none => out := out.push (a, 1)
  return out

def showAggregate (p : Poly) : String :=
  if p.positions.isEmpty then "0" else
    let parts : Array String := p.signature.map fun (arity, count) =>
      match count, arity with
      | c, 0     => toString c
      | 1, 1     => "y"
      | c, 1     => s!"{c}y"
      | 1, a     => s!"y^{a}"
      | c, a     => s!"{c}y^{a}"
    String.intercalate " + " parts.toList

def showNamed (p : Poly) : String :=
  if p.positions.isEmpty then "0" else
    let parts : Array String := p.positions.map fun pos =>
      if pos.dirs.isEmpty then s!"[{pos.name}]"
      else s!"[{pos.name}]y^\{{String.intercalate "," pos.dirs.toList}}"
    String.intercalate " + " parts.toList

instance : ToString Poly where
  toString := showAggregate

end Poly

end TinyPoly
