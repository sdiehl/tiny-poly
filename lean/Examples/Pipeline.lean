/-
Two small machines piped together, exactly like `cmd1 | cmd2` in a shell.

Stage 1 is a parity counter: it reads bits and outputs 0 or 1 depending
on whether it has seen an even or odd number of 1s so far.
Stage 2 watches that stream and trips an ALARM if it ever sees three
1s in a row. So the combined pipeline is an anomaly detector: each
stage is dumb on its own, the whole thing computes something useful.

The interesting bit: the "wire two machines together" operation is
itself a first-class thing in this library. You did not have to write
any glue. Same reason Unix pipes feel powerful: a tiny composition
operator on tiny components covers an enormous amount of ground.
-/

import TinyPoly

open TinyPoly

def detector : Moore :=
  { states  := #["q0", "q1", "q2", "ALARM"]
    inputs  := #["0", "1"]
    outputs := #["idle", "ALARM"]
    readout := #[0, 0, 0, 1]
    update  := #[
      #[0, 1],
      #[0, 2],
      #[0, 3],
      #[3, 3]
    ] }

-- Bridge value at tick t is readout(parity) at tick t-1, so the detector
-- sees three '1's whenever parity stays odd for three consecutive ticks.
def bits : Array Nat := #[1, 0, 0, 0, 1, 1, 0, 0, 0]

def main : IO Unit := do
  let wired := series! Moore.parity detector

  IO.println s!"stage 1 interface: {Moore.parity.interfacePoly.showAggregate}"
  IO.println s!"stage 2 interface: {detector.interfacePoly.showAggregate}"
  IO.println s!"wired   interface: {wired.interfacePoly.showAggregate}"

  let trace := wired.run 0 bits
  let bitStr := String.intercalate " " (bits.toList.map toString)
  IO.println ""
  IO.println s!"inputs : {bitStr}"
  IO.println "trace  :"
  for (s, o) in trace do
    IO.println s!"  state={wired.states[s]!} output={wired.outputs[o]!}"
