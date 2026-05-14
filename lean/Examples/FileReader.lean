/-
A file reader, and the same reader wrapped into a file searcher.

The reader walks a fixed file one character at a time. Its only input
is "advance", and at each step it shows the character at the cursor;
when it falls off the end it shows EOF. As a Moore machine, that's
states 0..length with a one-step update and a readout that maps each
cursor to its character.

The searcher is a one-character pattern matcher: it watches a stream
of characters and flips a "found" flag whenever it sees the target
(here 'L'). On its own it has nothing to do with files.

The interesting move is the wiring. We pipe the reader's output into
the searcher's input via `series`. The result is a new Moore machine
whose state is the pair (reader cursor, searcher memory) and whose
only external input is still "advance": one button, one verdict per
step. You did not have to write any plumbing; the polynomial framing
hands you the wired machine for free, and it would have hooked up any
two machines with compatible interfaces the same way.

Spivak & Niu, Exercise 4.14 (File reader) + Exercise 4.40 (wrapper
turning a reader into a searcher), pp.87, 101.
-/

import TinyPoly

open TinyPoly

def file : Array String := #["H", "E", "L", "L", "O"]
def alphabet : Array String := #["H", "E", "L", "O", "EOF"]

def readerStates : Array String :=
  (Array.range (file.size + 1)).map fun i => s!"c{i}"

def readerReadout : Array Nat :=
  (Array.range (file.size + 1)).map fun i =>
    if i < file.size then
      (alphabet.findIdx? (· == file[i]!)).getD 0
    else
      alphabet.size - 1

def readerUpdate : Array (Array Nat) :=
  (Array.range (file.size + 1)).map fun i =>
    #[min (i + 1) file.size]

def reader : Moore :=
  { states := readerStates
    inputs := #["advance"]
    outputs := alphabet
    readout := readerReadout
    update := readerUpdate }

def target : String := "L"
def targetIdx : Nat := (alphabet.findIdx? (· == target)).getD 0

def searcherStates : Array String :=
  alphabet.map fun s => s!"saw_{s}"

def searcherReadout : Array Nat :=
  (Array.range alphabet.size).map fun i => if i == targetIdx then 1 else 0

def searcherUpdate : Array (Array Nat) :=
  (Array.range alphabet.size).map fun _ => Array.range alphabet.size

def searcher : Moore :=
  { states := searcherStates
    inputs := alphabet
    outputs := #["skip", "found"]
    readout := searcherReadout
    update := searcherUpdate }

def main : IO Unit := do
  let wired := series! reader searcher

  IO.println s!"reader   interface: {reader.interfacePoly.showAggregate}"
  IO.println s!"searcher interface: {searcher.interfacePoly.showAggregate}"
  IO.println s!"wired    interface: {wired.interfacePoly.showAggregate}"

  let ticks := Array.replicate (file.size + 1) 0
  let trace := wired.run 0 ticks
  IO.println ""
  IO.println s!"file       : {String.intercalate "" file.toList}"
  IO.println s!"looking for: {target}"
  IO.println s!"trace      : {wired.showTrace trace}"
