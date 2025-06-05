use super::types::{Bounds, Node};
use rand::prelude::IndexedRandom;
use rand::Rng;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub struct ColorData(pub f64, pub f64, pub f64);

impl Default for ColorData {
    fn default() -> Self {
        Self(1.0, 1.0, 1.0)
    }
}

pub struct Palette(pub Vec<ColorData>);

impl Palette {
    pub fn randomize<T: Clone + Ord, R: Rng + ?Sized>(
        &self,
        v: &BTreeSet<T>,
        mut rng: &mut R,
    ) -> BTreeMap<T, ColorData> {
        v.iter()
            .cloned()
            .map(|t| (t, self.0.choose(&mut rng).cloned().unwrap_or_default()))
            .collect()
    }

    pub fn generate<N: Node, B: Bounds<N>>(
        &self,
        v: &BTreeSet<N>,
        prev: &BTreeMap<N, ColorData>,
        bounds: &B,
    ) -> BTreeMap<N, ColorData> {
        v.iter()
            .map(|n| match prev.get(n) {
                Some(c) => (n.clone(), c.clone()),
                None => {
                    let colors: &Vec<&ColorData> = &n
                        .neighbors()
                        .into_iter()
                        .filter_map(|n| bounds.transform(n))
                        .filter_map(|n| prev.get(&n))
                        .collect();
                    let mut color = colors
                        .iter()
                        .fold(&mut ColorData(0.0, 0.0, 0.0), |r, c| {
                            r.0 += c.0;
                            r.1 += c.1;
                            r.2 += c.2;
                            r
                        })
                        .to_owned();
                    color.0 /= colors.len() as f64;
                    color.1 /= colors.len() as f64;
                    color.2 /= colors.len() as f64;
                    (n.clone(), color)
                }
            })
            .collect()
    }
}
