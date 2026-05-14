// A counter that ticks through 0, 1, 2, ... mod N.
//
// This is the smallest Moore machine that does anything interesting: one
// button (tick), one display (the current number). What it shows in the
// polynomial framing is that the state space and the interface can be
// completely different shapes. Here the state space is "all the numbers
// from 0 to N-1" (N positions, each leading to any other), while the
// interface is "one display, one button" (one position with one
// direction). The Moore machine is the rule that takes "what state am I
// in" and translates it down to "what does the user see, and what does
// the user's input do".
//
// Spivak & Niu, Example 4.6 (Counter), p.85.

use tiny_poly::Moore;

fn main() {
    const N: usize = 8;

    let states: Vec<String> = (0..N).map(|i| i.to_string()).collect();
    let outputs = states.clone();
    let readout: Vec<usize> = (0..N).collect();
    let update: Vec<Vec<usize>> = (0..N).map(|i| vec![(i + 1) % N]).collect();

    let counter = Moore::new(states, vec!["tick".into()], outputs, readout, update);

    println!("state polynomial    S y^S = {}", counter.state_poly());
    println!("interface polynomial B y^A = {}", counter.interface_poly());

    let trace = counter.run(0, &[0; 10]);
    println!();
    println!("ticks  : {}", "tick ".repeat(10).trim_end());
    println!("trace  : {}", counter.show_trace(&trace));
}
