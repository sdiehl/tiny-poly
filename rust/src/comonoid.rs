use crate::lens::Lens;
use crate::ops::compose;
use crate::poly::{Poly, Position};

#[derive(Clone, Debug)]
pub struct Morphism {
    pub name: String,
    pub target: usize,
}

// A small category encoded as: objects, morphisms out of each object (with index 0 = identity),
// and a composition table table[x][f][g] = index in M_x of "first f, then g".
#[derive(Clone, Debug)]
pub struct Category {
    pub objects: Vec<String>,
    pub morphisms: Vec<Vec<Morphism>>,
    pub table: Vec<Vec<Vec<usize>>>,
}

impl Category {
    pub fn poly(&self) -> Poly {
        let positions = self
            .objects
            .iter()
            .zip(self.morphisms.iter())
            .map(|(obj, ms)| {
                let dirs = ms.iter().map(|m| m.name.clone()).collect();
                Position::new(obj.clone(), dirs)
            })
            .collect();
        Poly::new(positions)
    }

    pub fn epsilon(&self) -> Lens {
        let p = self.poly();
        let y = Poly::y();
        let on_pos = vec![0; p.npositions()];
        let on_dir = (0..p.npositions()).map(|_| vec![0]).collect();
        Lens::new(p, y, on_pos, on_dir)
    }

    pub fn delta(&self) -> Lens {
        let p = self.poly();
        let pp = compose(&p, &p);
        let n = self.objects.len();

        let position_index = |obj_idx: usize, choice: &[usize]| -> usize {
            let mut idx = 0;
            for prev_obj in 0..obj_idx {
                let arity = self.morphisms[prev_obj].len();
                idx += pow_usize(n, arity);
            }
            let arity = self.morphisms[obj_idx].len();
            let mut offset = 0;
            for (k, &c) in choice.iter().enumerate() {
                offset += c * pow_usize(n, arity - 1 - k);
            }
            idx + offset
        };

        let mut on_pos = Vec::with_capacity(n);
        let mut on_dir = Vec::with_capacity(n);
        for (x, ms) in self.morphisms.iter().enumerate() {
            let choice: Vec<usize> = ms.iter().map(|m| m.target).collect();
            on_pos.push(position_index(x, &choice));
            let mut dirs = Vec::new();
            for (f_idx, m_f) in ms.iter().enumerate() {
                let y_obj = m_f.target;
                for (g_idx, _) in self.morphisms[y_obj].iter().enumerate() {
                    let composite = self.table[x][f_idx][g_idx];
                    dirs.push(composite);
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
                let id_y = 0;
                if self.table[x][f_idx][id_y] != f_idx {
                    return Err(format!(
                        "right unit fails at {}.{}",
                        self.objects[x], f.name
                    ));
                }
            }
            let id_x = 0;
            for (g_idx, _) in ms.iter().enumerate() {
                if self.table[x][id_x][g_idx] != g_idx {
                    return Err(format!("left unit fails at {}", self.objects[x]));
                }
            }
        }
        for (x, ms) in self.morphisms.iter().enumerate() {
            for (f_idx, m_f) in ms.iter().enumerate() {
                let y = m_f.target;
                for (g_idx, m_g) in self.morphisms[y].iter().enumerate() {
                    let z = m_g.target;
                    for (h_idx, _) in self.morphisms[z].iter().enumerate() {
                        let gf = self.table[x][f_idx][g_idx];
                        let hgf_left = self.table[x][gf][h_idx];
                        let hg = self.table[y][g_idx][h_idx];
                        let hgf_right = self.table[x][f_idx][hg];
                        if hgf_left != hgf_right {
                            return Err(format!(
                                "associativity fails at {}.{}.{}.{}",
                                self.objects[x], m_f.name, m_g.name, self.morphisms[z][h_idx].name,
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn walking_arrow() -> Self {
        let objects = vec!["a".into(), "b".into()];
        let morphisms = vec![
            vec![
                Morphism {
                    name: "id_a".into(),
                    target: 0,
                },
                Morphism {
                    name: "f".into(),
                    target: 1,
                },
            ],
            vec![Morphism {
                name: "id_b".into(),
                target: 1,
            }],
        ];
        let table = vec![vec![vec![0, 1], vec![1]], vec![vec![0]]];
        Self {
            objects,
            morphisms,
            table,
        }
    }

    pub fn monoid_z2() -> Self {
        let objects = vec!["*".into()];
        let morphisms = vec![vec![
            Morphism {
                name: "e".into(),
                target: 0,
            },
            Morphism {
                name: "t".into(),
                target: 0,
            },
        ]];
        let table = vec![vec![vec![0, 1], vec![1, 0]]];
        Self {
            objects,
            morphisms,
            table,
        }
    }

    pub fn discrete(objs: &[&str]) -> Self {
        let objects = objs.iter().map(|s| (*s).to_string()).collect();
        let morphisms = (0..objs.len())
            .map(|i| {
                vec![Morphism {
                    name: format!("id_{}", objs[i]),
                    target: i,
                }]
            })
            .collect();
        let table = (0..objs.len()).map(|_| vec![vec![0]]).collect();
        Self {
            objects,
            morphisms,
            table,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Comonoid {
    pub poly: Poly,
    pub epsilon: Lens,
    pub delta: Lens,
}

fn pow_usize(base: usize, exp: usize) -> usize {
    let mut r = 1usize;
    for _ in 0..exp {
        r *= base;
    }
    r
}
