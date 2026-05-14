// An arrow field on a small category.
//
// Take any category C. An arrow field on C is a choice of one outgoing
// arrow per object. That's it. If you stand at object x, the field tells
// you which arrow to take; if you then stand at the object that arrow
// led to, the field tells you the next arrow; and so on. So a field
// gives you a way to "walk" through C, one step per click. This is
// exactly the discrete analogue of a vector field on a manifold, where
// the manifold is replaced by C and the vectors by arrows.
//
// We illustrate on a 4-object linear poset {0, 1, 2, 3} with i -> j
// whenever i <= j. There are 4 + 3 + 2 + 1 = 10 arrows in total. We
// pick a field that always jumps "forward by one if possible, else stay"
// and iterate it from object 0.
//
// The punchline of Spivak & Niu's framing: an arrow field is the same
// data as a retrofunctor `C -> y^N` from C to the natural-number monoid.
// The retrofunctor laws (preserve identities, preserve composition)
// recover "zero steps is do nothing" and "a then b is the same as a+b
// steps", with no further bookkeeping required.
//
// Spivak & Niu, Example 7.71 (Arrow fields), p.262.

use tiny_poly::{Category, Morphism};

fn linear_poset(n: usize) -> Category {
    // Arrow at index k of M_i is "jump by k", landing at i+k. So index 0
    // is the identity, and arrows are listed in order of jump size.
    let objects: Vec<String> = (0..n).map(|i| i.to_string()).collect();
    let morphisms: Vec<Vec<Morphism>> = (0..n)
        .map(|i| {
            (0..(n - i))
                .map(|k| Morphism {
                    name: if k == 0 {
                        format!("id_{i}")
                    } else {
                        format!("e_{i}_{}", i + k)
                    },
                    target: i + k,
                })
                .collect()
        })
        .collect();
    let table: Vec<Vec<Vec<usize>>> = (0..n)
        .map(|i| {
            (0..morphisms[i].len())
                .map(|f| {
                    let mid = morphisms[i][f].target;
                    (0..morphisms[mid].len()).map(|g| f + g).collect()
                })
                .collect()
        })
        .collect();
    Category {
        objects,
        morphisms,
        table,
    }
}

fn main() {
    let c = linear_poset(4);
    c.check_axioms().expect("linear poset is a category");

    // The "forward if you can" field: at each object, choose the smallest
    // non-identity jump if one exists, else the identity.
    let field: Vec<usize> = (0..c.objects.len())
        .map(|i| usize::from(c.morphisms[i].len() > 1))
        .collect();

    println!("category : {}-element linear poset", c.objects.len());
    println!("field    :");
    for (i, &k) in field.iter().enumerate() {
        let m = &c.morphisms[i][k];
        println!(
            "  at {} take {} (-> {})",
            c.objects[i], m.name, c.objects[m.target]
        );
    }

    println!();
    println!("walking from 0, applying the field N times:");
    let mut cursor = 0usize;
    println!("  step 0: at {}", c.objects[cursor]);
    for step in 1..=6 {
        let m = &c.morphisms[cursor][field[cursor]];
        cursor = m.target;
        println!("  step {step}: take {} -> {}", m.name, c.objects[cursor]);
    }

    // The retrofunctor laws are automatic from this construction: 0 steps
    // is the identity, and walking a+b steps is the same as walking a
    // steps then b more from where you ended up. We spot-check the second
    // law for a = 2, b = 3, starting from object 0.
    let (a, b) = (2usize, 3usize);
    let mut left = 0usize;
    for _ in 0..(a + b) {
        left = c.morphisms[left][field[left]].target;
    }
    let mut right = 0usize;
    for _ in 0..a {
        right = c.morphisms[right][field[right]].target;
    }
    for _ in 0..b {
        right = c.morphisms[right][field[right]].target;
    }
    println!();
    println!("retrofunctor law (composition) for a={a}, b={b}:");
    println!("  walk {} steps      lands at {}", a + b, c.objects[left]);
    println!("  walk {a} then {b} steps lands at {}", c.objects[right]);
    assert_eq!(left, right);
    println!("  ok");
}
