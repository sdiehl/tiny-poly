import TinyPoly.Poly
import TinyPoly.Lens

namespace TinyPoly

structure Moore where
  states : Array String
  inputs : Array String
  outputs : Array String
  readout : Array Nat
  update : Array (Array Nat)
  deriving Repr, Inhabited

namespace Moore

def statePoly (m : Moore) : Poly :=
  { positions := m.states.map fun s => { name := s, dirs := m.states } }

def interfacePoly (m : Moore) : Poly :=
  { positions := m.outputs.map fun o => { name := o, dirs := m.inputs } }

def asLens (m : Moore) : Lens :=
  Lens.mk! m.statePoly m.interfacePoly m.readout m.update

def step (m : Moore) (state input : Nat) : Nat :=
  (m.update[state]!)[input]!

def output (m : Moore) (state : Nat) : Nat :=
  m.readout[state]!

def run (m : Moore) (initial : Nat) (inputs : Array Nat) : Array (Nat × Nat) := Id.run do
  let mut state := initial
  let mut trace : Array (Nat × Nat) := #[(state, m.readout[state]!)]
  for a in inputs do
    state := (m.update[state]!)[a]!
    trace := trace.push (state, m.readout[state]!)
  return trace

def showTrace (m : Moore) (trace : Array (Nat × Nat)) : String :=
  String.intercalate " -> " (trace.map (fun (s, o) => s!"{m.states[s]!}/{m.outputs[o]!}")).toList

def parity : Moore :=
  { states := #["even", "odd"]
    inputs := #["0", "1"]
    outputs := #["0", "1"]
    readout := #[0, 1]
    update := #[#[0, 1], #[1, 0]] }

def trafficLight : Moore :=
  { states := #["red", "green", "yellow"]
    inputs := #["tick", "emergency"]
    outputs := #["STOP", "GO", "SLOW"]
    readout := #[0, 1, 2]
    update := #[#[1, 0], #[2, 0], #[0, 0]] }

end Moore

end TinyPoly
