import TinyPoly.Poly

namespace TinyPoly

structure Lens where
  src : Poly
  tgt : Poly
  onPos : Array Nat
  onDir : Array (Array Nat)
  deriving Repr, Inhabited

namespace Lens

def validate (l : Lens) : Except String Unit := do
  if l.onPos.size != l.src.npositions then
    throw s!"onPos has {l.onPos.size} entries, expected {l.src.npositions}"
  if l.onDir.size != l.src.npositions then
    throw s!"onDir has {l.onDir.size} entries, expected {l.src.npositions}"
  for i in [:l.src.npositions] do
    let j := l.onPos[i]!
    if j >= l.tgt.npositions then
      throw s!"onPos[{i}] = {j} out of range"
    let expected := l.tgt.arity j
    let got := (l.onDir[i]!).size
    if got != expected then
      throw s!"onDir[{i}] has {got} entries, expected {expected}"
    let srcArity := l.src.arity i
    for k in [:got] do
      let d := (l.onDir[i]!)[k]!
      if d >= srcArity then
        throw s!"onDir[{i}][{k}] = {d} out of range (src arity {srcArity})"

def mk! (src tgt : Poly) (onPos : Array Nat) (onDir : Array (Array Nat)) : Lens :=
  let l : Lens := { src, tgt, onPos, onDir }
  match l.validate with
  | .ok _ => l
  | .error e => panic! s!"invalid lens: {e}"

def id (p : Poly) : Lens :=
  let onPos := (Array.range p.npositions)
  let onDir := p.positions.map fun pos => Array.range pos.arity
  { src := p, tgt := p, onPos, onDir }

def compose (f g : Lens) : Except String Lens := do
  if f.tgt != g.src then throw "cannot compose: f.tgt != g.src"
  let onPos := f.onPos.map fun p => g.onPos[p]!
  let onDir : Array (Array Nat) := Id.run do
    let mut out : Array (Array Nat) := #[]
    for i in [:f.src.npositions] do
      let mid := f.onPos[i]!
      out := out.push <| (g.onDir[mid]!).map fun rDir => (f.onDir[i]!)[rDir]!
    return out
  return { src := f.src, tgt := g.tgt, onPos, onDir }

def pretty (l : Lens) : String := Id.run do
  let mut out := s!"lens {l.src.showNamed} -> {l.tgt.showNamed}\n"
  for i in [:l.src.positions.size] do
    let p := l.src.positions[i]!
    let tgtIdx := l.onPos[i]!
    let tgtPos := l.tgt.positions[tgtIdx]!
    let mut line := s!"  {p.name} |-> {tgtPos.name}"
    if tgtPos.arity > 0 then
      let pullback : Array String := Id.run do
        let mut acc : Array String := #[]
        for k in [:tgtPos.dirs.size] do
          let td := tgtPos.dirs[k]!
          let si := (l.onDir[i]!)[k]!
          acc := acc.push s!"{td}<-{p.dirs[si]!}"
        return acc
      line := line ++ s!" [{String.intercalate ", " pullback.toList}]"
    out := out ++ line ++ "\n"
  return out

instance : ToString Lens where
  toString := pretty

end Lens

end TinyPoly
