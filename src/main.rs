use std::collections::{HashMap, HashSet};
use std::fs;

use eframe::{App, CreationContext, NativeOptions, run_native};
use egui_graphs::{
    Graph, LayoutHierarchical, LayoutRandom, LayoutStateHierarchical, LayoutStateRandom,
    SettingsInteraction, SettingsNavigation, SettingsStyle,
};
use node::Node;
use petgraph::Directed;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;

mod graph;
mod node;

pub struct Pathfinder {
    g: Graph<String, u32, Directed>,
}

impl Pathfinder {
    fn new(_: &CreationContext<'_>) -> Self {
        let (graph, _matrix, values) = load_graph();

        let mut g = Graph::from(&graph);

        values.iter().for_each(|node| {
            g.node_mut(node.id).unwrap().set_label(node.name.clone());

            node.neighbours.iter().for_each(|edge| {
                g.edge_mut(edge.2.unwrap())
                    .unwrap()
                    .set_label(edge.1.to_string())
            });
        });

        Self { g }
    }
}

impl App for Pathfinder {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let interaction_settings = &SettingsInteraction::new()
                .with_dragging_enabled(true)
                .with_node_clicking_enabled(true)
                .with_node_selection_enabled(true)
                .with_node_selection_multi_enabled(true)
                .with_edge_clicking_enabled(true)
                .with_edge_selection_enabled(true)
                .with_edge_selection_multi_enabled(true);
            let style_settings = &SettingsStyle::new().with_labels_always(true);
            let navigation_settings = &SettingsNavigation::new()
                .with_fit_to_screen_enabled(false)
                .with_zoom_and_pan_enabled(true);

            ui.add(
                &mut egui_graphs::GraphView::<
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    LayoutStateRandom,
                    LayoutRandom,
                >::new(&mut self.g)
                .with_styles(style_settings)
                .with_interactions(interaction_settings)
                .with_navigations(navigation_settings),
            );
        });
    }
}

fn load_graph() -> (StableGraph<String, u32>, Vec<Vec<u32>>, Vec<Node>) {
    let lines = fs::read_to_string("data/basic.txt")
        .expect("Oops, could not open file.")
        .lines()
        .map(|line| {
            let parsed = line.split(",").collect::<Vec<&str>>();

            (
                parsed[0].to_owned() + ", " + parsed[1],
                parsed[2].to_owned() + ", " + parsed[3],
                parsed[4].parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<(String, String, u32)>>();

    let mut graph: StableGraph<String, u32> = StableGraph::new();

    let ids = lines
        .iter()
        .flat_map(|line| [line.0.clone(), line.1.clone()])
        .collect::<HashSet<String>>()
        .into_iter()
        .collect::<Vec<String>>();

    let mut node_map: HashMap<String, NodeIndex> = HashMap::new();

    ids.iter().for_each(|city| {
        let idx = graph.add_node(city.clone());
        node_map.insert(city.clone(), idx);
    });

    let nodes = lines.iter().fold(
        HashMap::new(),
        |mut acc, curr| -> HashMap<NodeIndex, Node> {
            let curr_id = *node_map.get(&curr.0).unwrap();
            let end_id = *node_map.get(&curr.1).unwrap();

            acc.entry(curr_id)
                .and_modify(|node| node.neighbours.push((end_id, curr.2, None)))
                .or_insert(Node {
                    id: curr_id,
                    name: curr.0.clone(),
                    neighbours: vec![(end_id, curr.2, None)],
                });

            acc
        },
    );

    let len = ids.len();
    let mut matrix: Vec<Vec<u32>> = (0..len).map(|_| vec![u32::MAX; len]).collect();

    let mut values = nodes.into_values().collect::<Vec<Node>>();

    for (i, column) in matrix.iter_mut().enumerate() {
        for (j, val) in column.iter_mut().enumerate() {
            if j == i {
                continue;
            }

            if let Some(begin) = values.iter_mut().find(|node| node.id.index() == j) {
                if let Some(found) = begin.neighbours.iter_mut().find(|n| n.0.index() == i) {
                    let edge_idx = graph.add_edge(begin.id, found.0, found.1);

                    found.2 = Some(edge_idx);

                    *val = found.1;
                }
            }
        }
    }

    (graph, matrix, values)
}

fn main() {
    run_native(
        "Pathfinder",
        NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(Pathfinder::new(cc)))),
    )
    .unwrap();
}
