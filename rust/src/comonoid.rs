use crate::lens::Lens;
use crate::ops::compose;
use crate::poly::{Poly, Position};

#[derive(Clone, Debug)]
pub struct Morphism {
    pub name: String,
    pub target: usize,
}

// A small category encoded as: objects, morphisms out of each object (with
// index 0 = identity), and a composition table where
// `table[x][f][g] = index in M_x of "first f, then g"`.
#[derive(Clone, Debug)]
pub struct Category {
    pub objects: Vec<String>,
    pub morphisms: Vec<Vec<Morphism>>,
    pub table: Vec<Vec<Vec<usize>>>,
}

#[derive(Clone, Debug)]
pub struct Comonoid {
    pub poly: Poly,
    pub epsilon: Lens,
    pub delta: Lens,
}

impl Category {
    pub fn poly(&self) -> Poly {
        let positions = self
            .objects
            .iter()
            .zip(&self.morphisms)
            .map(|(obj, ms)| {
                let dirs = ms.iter().map(|m| m.name.clone()).collect();
                Position::new(obj.clone(), dirs)
            })
            .collect();
        Poly::new(positions)
    }

    pub fn epsilon(&self) -> Lens {
        let p = self.poly();
        let n = p.npositions();
        let on_pos = vec![0; n];
        let on_dir = vec![vec![0]; n];
        Lens::new(p, Poly::y(), on_pos, on_dir)
    }

    pub fn delta(&self) -> Lens {
        let p = self.poly();
        let pp = compose(&p, &p);
        let n = self.objects.len();

        let mut on_pos = Vec::with_capacity(self.morphisms.len());
        let mut on_dir = Vec::with_capacity(self.morphisms.len());
        for (x, ms) in self.morphisms.iter().enumerate() {
            let choice: Vec<usize> = ms.iter().map(|m| m.target).collect();
            on_pos.push(self.position_index(n, x, &choice));
            let mut dirs = Vec::new();
            for (f_idx, f) in ms.iter().enumerate() {
                for g_idx in 0..self.morphisms[f.target].len() {
                    dirs.push(self.table[x][f_idx][g_idx]);
                }
            }
            on_dir.push(dirs);
        }
        Lens::new(p, pp, on_pos, on_dir)
    }

    pub fn comonoid(&self) -> Comonoid {
        Comonoid {
            poly: self.poly(),
            epsilon: self.epsilon(),
            delta: self.delta(),
        }
    }

    pub fn check_axioms(&self) -> Result<(), String> {
        for (x, ms) in self.morphisms.iter().enumerate() {
            for (f_idx, f) in ms.iter().enumerate() {
                if self.table[x][f_idx][0] != f_idx {
                    return Err(format!(
                        "right unit fails at {}.{}",
                        self.objects[x], f.name
                    ));
                }
            }
            for g_idx in 0..ms.len() {
                if self.table[x][0][g_idx] != g_idx {
                    return Err(format!("left unit fails at {}", self.objects[x]));
                }
            }
        }
        for (x, ms) in self.morphisms.iter().enumerate() {
            for (f_idx, f) in ms.iter().enumerate() {
                let y = f.target;
                for (g_idx, g) in self.morphisms[y].iter().enumerate() {
                    let z = g.target;
                    for (h_idx, h) in self.morphisms[z].iter().enumerate() {
                        let gf = self.table[x][f_idx][g_idx];
                        let hgf_left = self.table[x][gf][h_idx];
                        let hg = self.table[y][g_idx][h_idx];
                        let hgf_right = self.table[x][f_idx][hg];
                        if hgf_left != hgf_right {
                            return Err(format!(
                                "associativity fails at {}.{}.{}.{}",
                                self.objects[x], f.name, g.name, h.name,
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    // Linear index of (object x, choice of target objects per outgoing arrow)
    // in the position list of P <| P. Objects are ordered as in `self.objects`;
    // for each x the block has size n^arity(x) where n = #objects.
    fn position_index(&self, n: usize, obj_idx: usize, choice: &[usize]) -> usize {
        let mut idx = 0;
        for prev in 0..obj_idx {
            idx += n.pow(self.morphisms[prev].len() as u32);
        }
        let arity = self.morphisms[obj_idx].len();
        let mut offset = 0;
        for (k, &c) in choice.iter().enumerate() {
            offset += c * n.pow((arity - 1 - k) as u32);
        }
        idx + offset
    }
}
