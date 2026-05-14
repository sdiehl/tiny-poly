// The walking commutative square: the smallest category that is not free.
//
//        h
//   w ------> y
//   |         |
//   | f       | k
//   v         v
//   x ------> z
//        g
//
//   f # g  =  h # k
//
// Four objects, six "named" arrows (id_w, id_x, id_y, id_z, f, g, h, k),
// and the rule that the two paths around the square give the same arrow
// w -> z. Adding that single morphism (call it fg = hk) gives nine
// morphisms total. Going round the long way and going round the short
// way have to land at the same place; that is what "commutative" means.
//
// Why bother? Because the library's check_axioms exists exactly to catch
// the moment when "I thought I wrote down a category but I forgot a
// composite" turns into "this is not a category at all". For a freely
// generated category the check is trivial; for one with relations like
// this square it actually has work to do.
//
// Spivak & Niu, Example 7.68 (Retrofunctors to the walking commutative
// square), p.260.

use tiny_poly::{Category, Morphism};

fn main() {
    let square = Category {
        objects: vec!["w".into(), "x".into(), "y".into(), "z".into()],
        morphisms: vec![
            vec![
                Morphism {
                    name: "id_w".into(),
                    target: 0,
                },
                Morphism {
                    name: "f".into(),
                    target: 1,
                },
                Morphism {
                    name: "h".into(),
                    target: 2,
                },
                Morphism {
                    name: "fg".into(),
                    target: 3,
                },
            ],
            vec![
                Morphism {
                    name: "id_x".into(),
                    target: 1,
                },
                Morphism {
                    name: "g".into(),
                    target: 3,
                },
            ],
            vec![
                Morphism {
                    name: "id_y".into(),
                    target: 2,
                },
                Morphism {
                    name: "k".into(),
                    target: 3,
                },
            ],
            vec![Morphism {
                name: "id_z".into(),
                target: 3,
            }],
        ],
        table: vec![
            vec![vec![0, 1, 2, 3], vec![1, 3], vec![2, 3], vec![3]],
            vec![vec![0, 1], vec![1]],
            vec![vec![0, 1], vec![1]],
            vec![vec![0]],
        ],
    };

    square
        .check_axioms()
        .expect("walking commutative square must be a category");

    let c = square.comonoid();
    println!("polynomial P_C = {}", c.poly.show_named());
    println!();
    println!("epsilon : P_C -> y");
    print!("{}", c.epsilon);
    println!("delta   : P_C -> P_C <| P_C");
    print!("{}", c.delta);

    let n_morph: usize = square.morphisms.iter().map(Vec::len).sum();
    println!("derived facts:");
    println!(
        "  {} objects, {} morphisms total ({} non-identity)",
        square.objects.len(),
        n_morph,
        n_morph - square.objects.len()
    );
    println!("  the long arrow w -> z is named fg, equal to h # k by the square relation");
}
