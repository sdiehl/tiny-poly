import TinyPoly.Moore

namespace TinyPoly

def parallel (m1 m2 : Moore) : Moore := Id.run do
  let n1 := m1.states.size
  let n2 := m2.states.size
  let nb2 := m2.outputs.size

  let mut states : Array String := #[]
  let mut readout : Array Nat := #[]
  for i in [:n1] do
    for j in [:n2] do
      states := states.push s!"({m1.states[i]!},{m2.states[j]!})"
      readout := readout.push (m1.readout[i]! * nb2 + m2.readout[j]!)

  let mut inputs : Array String := #[]
  for a in m1.inputs do
    for b in m2.inputs do
      inputs := inputs.push s!"({a},{b})"
  let mut outputs : Array String := #[]
  for a in m1.outputs do
    for b in m2.outputs do
      outputs := outputs.push s!"({a},{b})"

  let mut update : Array (Array Nat) := #[]
  for s1 in [:n1] do
    for s2 in [:n2] do
      let mut row : Array Nat := #[]
      for a1 in [:m1.inputs.size] do
        for a2 in [:m2.inputs.size] do
          let ns1 := (m1.update[s1]!)[a1]!
          let ns2 := (m2.update[s2]!)[a2]!
          row := row.push (ns1 * n2 + ns2)
      update := update.push row
  return { states, inputs, outputs, readout, update }

def series (m1 m2 : Moore) : Except String Moore := do
  if m1.outputs != m2.inputs then
    throw "series wiring needs M1 outputs to match M2 inputs"
  let n1 := m1.states.size
  let n2 := m2.states.size

  let mut states : Array String := #[]
  let mut readout : Array Nat := #[]
  for i in [:n1] do
    for j in [:n2] do
      states := states.push s!"({m1.states[i]!},{m2.states[j]!})"
      readout := readout.push m2.readout[j]!
  let inputs := m1.inputs
  let outputs := m2.outputs

  -- Each tick: external input drives M1; M1's current output is fed into M2.
  let mut update : Array (Array Nat) := #[]
  for s1 in [:n1] do
    for s2 in [:n2] do
      let bridge := m1.readout[s1]!
      let mut row : Array Nat := #[]
      for a in [:m1.inputs.size] do
        let ns1 := (m1.update[s1]!)[a]!
        let ns2 := (m2.update[s2]!)[bridge]!
        row := row.push (ns1 * n2 + ns2)
      update := update.push row
  return { states, inputs, outputs, readout, update }

def series! (m1 m2 : Moore) : Moore :=
  match series m1 m2 with
  | .ok m => m
  | .error e => panic! e

end TinyPoly
