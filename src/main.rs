use rand::{Rng, SeedableRng};
use std::collections::{BTreeMap, BTreeSet};

use life::sim::{
    cache::StateCache,
    color::{ColorData, Palette},
    graph::Graph,
    grid_node::{
        paint::{clear_terminal, paint_terminal, reset_terminal},
        Box, GridNode, WrapBounds,
    },
    rules::ConwayRules,
};

fn main() {
    hello();
}

pub fn hello() {
    println!("hello world!!");
    let bounds = WrapBounds(Box {
        min_x: 0,
        min_y: 0,
        max_x: 150,
        max_y: 150,
        // max_x: 345,
        // max_y: 565,
    });
    let mut state: BTreeSet<GridNode> = vec![
        // GridNode(1, 0),
        // GridNode(2, 1),
        // GridNode(0, 2),
        // GridNode(1, 2),
        // GridNode(2, 2),
    ]
    .into_iter()
    .collect();

    let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    for _ in 1..(bounds.0.max_x as u32 * bounds.0.max_y as u32) {
        let node = GridNode(
            rng.gen_range(bounds.0.min_x..bounds.0.max_x),
            rng.gen_range(bounds.0.min_y..bounds.0.max_y),
        );
        state.insert(node);
    }
    let mut palette = Palette(vec![
        ColorData(1.0, 0.0, 0.0),
        ColorData(0.0, 1.0, 0.0),
        ColorData(0.0, 0.0, 1.0),
    ]);
    let mut graph = Graph {
        generation: 0,
        state,
        bounds: bounds.clone(),
        rules: ConwayRules {},
    };
    let mut colors: BTreeMap<GridNode, ColorData> = palette.randomize(&graph.state, &mut rng);
    let _cache = StateCache::<GridNode>::default();

    clear_terminal();
    for (count, state) in graph.iter() {
        if count == 1000 {
            break;
        }
        if count > 0 {
            colors = palette.generate(&state, &colors, &bounds);
        }
        paint_terminal(&colors, bounds.0.max_x, bounds.0.max_y);
        println!("count: {count} {}", state.len());
        // if !cache.insert(&state) {
        //     println!("repeating...");
        //     break;
        // }
    }
    reset_terminal();
    eprintln!("done in {} {}", graph.generation, graph.state.len());

    // for (set_a, set_b) in cache.transitions {
    //     println!("{:?} -> {:?}", set_a, set_b);
    // }
}
