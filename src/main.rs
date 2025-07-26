use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

use eframe::{App, CreationContext, NativeOptions, run_native};
use egui_graphs::{
    Graph, LayoutRandom, LayoutStateRandom, SettingsInteraction, SettingsNavigation, SettingsStyle,
};
use node::Node;
use petgraph::Undirected;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use solvers::dp::DPSolver;
use solvers::ga::ga_trait::GeneticAlgorithm;
use solvers::ga::parallel::ParallelGASolver;
use solvers::ga::sequential::SequentialGASolver;

mod node;
mod solvers;

type GraphTuple = (
    StableGraph<String, (), Undirected>,
    Vec<Vec<u32>>,
    HashMap<NodeIndex, Node>,
);

enum SolutionStrategy {
    HeldKarp,
    GeneticAlgorithm,
    GeneticAlgorithmParallel,
}

pub struct Pathfinder {
    g: Graph<String, (), Undirected>,
    final_cost: String,
    nodes: HashMap<NodeIndex, Node>,
    dp_solver: DPSolver,
    ga_solver: SequentialGASolver,
    parallel_solver: ParallelGASolver,
}

impl Pathfinder {
    fn new(_: &CreationContext<'_>) -> Self {
        let (graph, matrix, nodes) = load_graph();

        let mut g = Graph::from(&graph);

        nodes.values().for_each(|node| {
            g.node_mut(node.id).unwrap().set_label(node.name.clone());

            node.neighbours.iter().for_each(|edge| {
                g.edge_mut(edge.2.unwrap())
                    .unwrap()
                    .set_label(edge.1.to_string())
            });
        });

        Self {
            g,
            final_cost: "".to_string(),
            nodes,
            dp_solver: DPSolver::new(matrix.clone()),
            ga_solver: SequentialGASolver::new(matrix.clone()),
            parallel_solver: ParallelGASolver::new(matrix),
        }
    }

    fn solve(&mut self, strategy: SolutionStrategy) {
        let now = Instant::now();

        let (cost, path) = match strategy {
            SolutionStrategy::HeldKarp => self.dp_solver.solve(),
            SolutionStrategy::GeneticAlgorithm => self.ga_solver.solve(),
            SolutionStrategy::GeneticAlgorithmParallel => self.parallel_solver.solve(),
        };

        println!("ELAPSED: {}ms", now.elapsed().as_millis());
        println!("COST: {cost}");

        self.final_cost = cost.to_string();

        let edges: Vec<usize> = path
            .windows(2)
            .filter_map(|pair| {
                if let Some(val) = self.nodes.get(&NodeIndex::new(pair[0])) {
                    val.neighbours.iter().find_map(|n| {
                        if n.0.index() == pair[1] {
                            Some(n.2.unwrap().index())
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            })
            .collect();

        self.nodes
            .values()
            .flat_map(|node| node.get_edge_idxs(&edges))
            .for_each(|edge| {
                let _ = self.g.remove_edge(edge);
            });
    }
}

impl App for Pathfinder {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::SidePanel::right("right_panel")
            .min_width(250.)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label("Solve using:");

                    ui.vertical(|ui| {
                        if ui.button("Held-Karp").clicked() {
                            self.solve(SolutionStrategy::HeldKarp);
                        };
                    });

                    ui.vertical(|ui| {
                        if ui.button("Genetic Algorithm").clicked() {
                            self.solve(SolutionStrategy::GeneticAlgorithm);
                        };
                    });

                    ui.vertical(|ui| {
                        if ui.button("Genetic Algorithm Parallel").clicked() {
                            self.solve(SolutionStrategy::GeneticAlgorithmParallel);
                        };
                    });

                    if !self.final_cost.is_empty() {
                        ui.label(format!("COST: {}", self.final_cost));
                    }
                });
            });

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

fn load_graph() -> GraphTuple {
    let lines = fs::read_to_string("data/100.txt")
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

    let mut graph: StableGraph<String, (), Undirected> = StableGraph::default();

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

    let mut nodes = lines.iter().fold(
        HashMap::new(),
        |mut acc, curr| -> HashMap<NodeIndex, Node> {
            let curr_id = *node_map.get(&curr.0).unwrap();
            let end_id = *node_map.get(&curr.1).unwrap();

            acc.entry(end_id)
                .and_modify(|node| node.neighbours.push((curr_id, curr.2, None)))
                .or_insert(Node {
                    id: end_id,
                    name: curr.1.clone(),
                    neighbours: vec![(curr_id, curr.2, None)],
                });

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
    let mut matrix: Vec<Vec<u32>> = (0..len).map(|_| vec![0; len]).collect();

    for (i, column) in matrix.iter_mut().enumerate() {
        for (j, val) in column.iter_mut().enumerate() {
            if j == i {
                continue;
            }

            if let Some(begin) = nodes.get_mut(&NodeIndex::new(i)) {
                if let Some(found) = begin.neighbours.iter_mut().find(|n| n.0.index() == j) {
                    let edge_idx = match graph.find_edge_undirected(begin.id, found.0) {
                        Some(edge) => edge.0,
                        None => graph.add_edge(begin.id, found.0, ()),
                    };

                    found.2 = Some(edge_idx);

                    *val = found.1;
                }
            }
        }
    }

    (graph, matrix, nodes)
}

fn main() {
    run_native(
        "Pathfinder",
        NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(Pathfinder::new(cc)))),
    )
    .unwrap();
}
