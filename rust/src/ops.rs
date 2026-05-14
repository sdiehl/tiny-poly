use crate::poly::{Poly, Position};

pub fn sum(p: &Poly, q: &Poly) -> Poly {
    let mut positions = Vec::with_capacity(p.npositions() + q.npositions());
    for pos in &p.positions {
        positions.push(Position::new(format!("L.{}", pos.name), pos.dirs.clone()));
    }
    for pos in &q.positions {
        positions.push(Position::new(format!("R.{}", pos.name), pos.dirs.clone()));
    }
    Poly::new(positions)
}

pub fn tensor(p: &Poly, q: &Poly) -> Poly {
    let mut positions = Vec::with_capacity(p.npositions() * q.npositions());
    for pp in &p.positions {
        for qq in &q.positions {
            let name = format!("({},{})", pp.name, qq.name);
            let mut dirs = Vec::with_capacity(pp.arity() * qq.arity());
            for dp in &pp.dirs {
                for dq in &qq.dirs {
                    dirs.push(format!("{dp}.{dq}"));
                }
            }
            positions.push(Position::new(name, dirs));
        }
    }
    Poly::new(positions)
}

// Composition P <| Q: positions are (p, choice: P[p] -> Q(1));
// directions are pairs (d, e) with d in P[p] and e in Q[choice(d)].
pub fn compose(p: &Poly, q: &Poly) -> Poly {
    let nq = q.npositions();
    let mut positions = Vec::new();
    for pp in &p.positions {
        let k = pp.arity();
        for choice in cartesian_power(nq, k) {
            let chosen_names: Vec<&str> = choice
                .iter()
                .map(|&j| q.positions[j].name.as_str())
                .collect();
            let name = if k == 0 {
                pp.name.clone()
            } else {
                format!("{}<{}>", pp.name, chosen_names.join(","))
            };
            let mut dirs = Vec::new();
            for (d_idx, d_name) in pp.dirs.iter().enumerate() {
                for e_name in &q.positions[choice[d_idx]].dirs {
                    dirs.push(format!("{d_name}.{e_name}"));
                }
            }
            positions.push(Position::new(name, dirs));
        }
    }
    Poly::new(positions)
}

fn cartesian_power(n: usize, k: usize) -> Vec<Vec<usize>> {
    if k == 0 {
        return vec![vec![]];
    }
    if n == 0 {
        return vec![];
    }
    let mut out: Vec<Vec<usize>> = vec![vec![]];
    for _ in 0..k {
        let mut next = Vec::with_capacity(out.len() * n);
        for v in &out {
            for i in 0..n {
                let mut w = v.clone();
                w.push(i);
                next.push(w);
            }
        }
        out = next;
    }
    out
}
