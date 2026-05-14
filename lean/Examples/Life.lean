/-
Conway's Game of Life on an 8x8 torus.

Each cell is a tiny state machine with two states (dead, alive) and
256 possible inputs (one per configuration of its eight neighbours).
We build that machine through the library as a Moore machine with
interface polynomial 2 y^256: each readout reports "dead" or "alive",
each input is "the next neighbourhood I'm about to see".

The whole grid is then "all those cells wired together so that each
one's input is read off from its neighbours' outputs". Spivak and Niu
call this a closed wiring diagram (a section of the tensor of all
the per-cell polynomials onto y); from the cell's point of view there
is no external input, the grid is closed.

We don't expand the joint state space (2^64 states would be enormous),
so the evolution is done by hand on a fixed initial pattern. The cell
machine printed up front is the genuine library object; the runner
below is exactly what the wiring section in Example 4.66 unfolds to.

Spivak & Niu, Example 4.66 + Exercise 4.67 (Game of Life), p.118.
-/

import TinyPoly

open TinyPoly

def R : Nat := 8
def C : Nat := 8

def popcount (n : Nat) : Nat := Id.run do
  let mut x := n
  let mut count := 0
  for _ in [:8] do
    count := count + (x % 2)
    x := x / 2
  return count

def cellMachine : Moore :=
  let inputs : Array String :=
    (Array.range 256).map fun n => s!"n{n}"
  let update : Array (Array Nat) :=
    (Array.range 2).map fun s =>
      (Array.range 256).map fun n =>
        let count := popcount n
        let alive := if s == 1 then count == 2 || count == 3 else count == 3
        if alive then 1 else 0
  { states := #["dead", "alive"]
    inputs := inputs
    outputs := #["dead", "alive"]
    readout := #[0, 1]
    update := update }

def neighbourMask (grid : Array (Array Nat)) (r c : Nat) : Nat := Id.run do
  let offsets : List (Int × Int) :=
    [(-1, -1), (-1, 0), (-1, 1),
     (0,  -1),          (0,  1),
     (1,  -1), (1,  0), (1,  1)]
  let mut mask : Nat := 0
  let mut bit : Nat := 1
  for (dr, dc) in offsets do
    let nr := (((r : Int) + dr) % (R : Int) + (R : Int)) % (R : Int)
    let nc := (((c : Int) + dc) % (C : Int) + (C : Int)) % (C : Int)
    let v := (grid[nr.toNat]!)[nc.toNat]!
    if v == 1 then mask := mask + bit
    bit := bit * 2
  return mask

def step (cell : Moore) (grid : Array (Array Nat)) : Array (Array Nat) := Id.run do
  let mut next : Array (Array Nat) := #[]
  for r in [:R] do
    let mut row : Array Nat := #[]
    for c in [:C] do
      let s := (grid[r]!)[c]!
      let mask := neighbourMask grid r c
      row := row.push ((cell.update[s]!)[mask]!)
    next := next.push row
  return next

def showGrid (grid : Array (Array Nat)) : String :=
  String.intercalate "\n" (grid.toList.map fun row =>
    String.ofList (row.toList.map fun c => if c == 1 then '#' else '.'))

def initialGrid : Array (Array Nat) := Id.run do
  let mut g : Array (Array Nat) := #[]
  for r in [:R] do
    let mut row : Array Nat := #[]
    for c in [:C] do
      let alive := r == 3 && (c == 2 || c == 3 || c == 4)
      row := row.push (if alive then 1 else 0)
    g := g.push row
  return g

def main : IO Unit := do
  IO.println s!"per-cell interface polynomial = {cellMachine.interfacePoly.showAggregate}"
  IO.println s!"grid                          = {R}x{C} torus"

  let mut grid := initialGrid
  for gen in [:4] do
    IO.println ""
    IO.println s!"generation {gen}:"
    IO.println (showGrid grid)
    grid := step cellMachine grid
