// B-streams as a category.
//
// A B-stream is an infinite sequence of letters from some alphabet B.
// Shifting a stream forward by n positions is an "action" of the natural
// numbers on the set of streams, and any action gives a category: the
// objects are the streams, and the arrows out of a stream are the things
// you can shift it by.
//
// Truly infinite streams won't fit in a finite category, so we tile the
// example: take period-3 streams over {0,1} and the orbit of "001"
// under shift. That gives three streams (001, 010, 100) and a shift by
// {0,1,2} between them, which is exactly the cyclic group Z/3Z acting
// on its own three rotations.
//
// What the polynomial framing buys us: this category is a comonoid in
// (Poly, <|). The epsilon arrow reads off "the current letter" of a
// stream; the delta arrow says "walking a + b steps is the same as
// walking a steps then b steps". Those two laws are all there is to a
// stream, categorically speaking.
//
// Spivak & Niu, Example 7.45 (The category of B-streams), p.253, here
// quotiented to a finite cyclic action.

use tiny_poly::{Category, Morphism};

fn main() {
    let streams = ["001", "010", "100"];
    let shift_names = ["s0", "s1", "s2"];
    let n = streams.len();

    let morphisms: Vec<Vec<Morphism>> = (0..n)
        .map(|i| {
            (0..n)
                .map(|k| Morphism {
                    name: shift_names[k].into(),
                    target: (i + k) % n,
                })
                .collect()
        })
        .collect();
    let table: Vec<Vec<Vec<usize>>> = (0..n)
        .map(|_| {
            (0..n)
                .map(|f| (0..n).map(|g| (f + g) % n).collect())
                .collect()
        })
        .collect();

    let streams_cat = Category {
        objects: streams.iter().map(|s| (*s).to_string()).collect(),
        morphisms,
        table,
    };

    streams_cat
        .check_axioms()
        .expect("Z/3Z acting on its rotations is a category");

    let c = streams_cat.comonoid();
    println!("polynomial P_C = {}", c.poly.show_named());
    println!();
    println!("epsilon (read current position):");
    print!("{}", c.epsilon);
    println!("delta (split one shift into two consecutive shifts):");
    print!("{}", c.delta);

    println!("walking through stream 001 with shifts (1, 1, 1):");
    let mut here = 0usize;
    println!("  start  : {}", streams[here]);
    for step in 1..=3 {
        here = (here + 1) % n;
        println!("  shift  : +1");
        println!(
            "  stream : {} ({})",
            streams[here],
            if step % n == 0 {
                "back to start"
            } else {
                "rotated"
            }
        );
    }
}
