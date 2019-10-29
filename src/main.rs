use petgraph::{
    dot::{Config, Dot},
    graphmap::GraphMap,
    Directed, Direction,
};

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
enum Dishes {
    HotDog,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
enum Ingredients {
    HotDogBun,
    HotDogWeiner,
    HotDogWeinerCooked,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
enum Food {
    Dishes(Dishes),
    Ingredients(Ingredients),
    Actions(Actions),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
enum Actions {
    Cook,
}

struct Cookbook {
    graph: GraphMap<Food, f64, Directed>,
}

impl Cookbook {
    fn new() -> Cookbook {
        let mut graph = GraphMap::new();

        // Add nodes for actions
        graph.add_node(Food::Actions(Actions::Cook));

        // Add nodes for ingredients and dishes
        graph.add_node(Food::Ingredients(Ingredients::HotDogBun));
        graph.add_node(Food::Ingredients(Ingredients::HotDogWeiner));
        graph.add_node(Food::Ingredients(Ingredients::HotDogWeinerCooked));
        graph.add_node(Food::Dishes(Dishes::HotDog));

        // Add edges
        graph.add_edge(
            Food::Actions(Actions::Cook),
            Food::Ingredients(Ingredients::HotDogWeinerCooked),
            1.,
        );
        graph.add_edge(
            Food::Ingredients(Ingredients::HotDogWeiner),
            Food::Ingredients(Ingredients::HotDogWeinerCooked),
            1.,
        );
        graph.add_edge(
            Food::Ingredients(Ingredients::HotDogWeinerCooked),
            Food::Dishes(Dishes::HotDog),
            1.,
        );
        graph.add_edge(
            Food::Ingredients(Ingredients::HotDogBun),
            Food::Dishes(Dishes::HotDog),
            1.,
        );

        Cookbook { graph }
    }

    fn ingredients(&self, food_node: Food) -> Vec<Food> {
        self.graph
            .neighbors_directed(food_node, Direction::Incoming)
            .collect()
    }

    fn makes(&self, food_node: Food) -> Vec<Food> {
        self.graph
            .neighbors_directed(food_node, Direction::Outgoing)
            .collect()
    }
}

fn main() {
    let cookbook = Cookbook::new();

    println!(
        "{:?}",
        Dot::with_config(&cookbook.graph, &[Config::EdgeNoLabel])
    );

    println!("Ingredients needed to make a hot dog");
    for node in cookbook.ingredients(Food::Dishes(Dishes::HotDog)) {
        dbg!(node);
    }
    println!("");
    println!("These ingredients can make:");
    println!("Hot Dog Weiner can be used to make:");
    for node in cookbook.makes(Food::Ingredients(Ingredients::HotDogWeinerCooked)) {
        dbg!(node);
    }
    println!("Hot Dog Bun can be used to make:");
    for node in cookbook.makes(Food::Ingredients(Ingredients::HotDogBun)) {
        dbg!(node);
    }
}
