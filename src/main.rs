use petgraph::{graphmap::GraphMap, Directed, Direction};

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
}

fn main() {
    let mut deps: petgraph::graphmap::GraphMap<Food, f64, Directed> = GraphMap::new();
    deps.add_node(Food::Dishes(Dishes::HotDog));
    deps.add_node(Food::Ingredients(Ingredients::HotDogBun));
    deps.add_node(Food::Ingredients(Ingredients::HotDogWeiner));
    deps.add_node(Food::Ingredients(Ingredients::HotDogWeinerCooked));

    deps.add_edge(
        Food::Dishes(Dishes::HotDog),
        Food::Ingredients(Ingredients::HotDogWeinerCooked),
        1.,
    );
    deps.add_edge(
        Food::Dishes(Dishes::HotDog),
        Food::Ingredients(Ingredients::HotDogBun),
        1.,
    );
    deps.add_edge(
        Food::Ingredients(Ingredients::HotDogWeinerCooked),
        Food::Ingredients(Ingredients::HotDogWeiner),
        1.,
    );

    println!("Ingredients needed to make a hot dog");
    for node in deps.neighbors(Food::Dishes(Dishes::HotDog)) {
        dbg!(node);
    }
    println!("");
    println!("These ingredients can make:");
    println!("Hot Dog Weiner can be used to make:");
    for node in deps.neighbors_directed(Food::Ingredients(Ingredients::HotDogWeinerCooked), Direction::Incoming) {
        dbg!(node);
    }
    println!("Hot Dog Bun can be used to make:");
    for node in deps.neighbors_directed(Food::Ingredients(Ingredients::HotDogBun), Direction::Incoming) {
        dbg!(node);
    }
}
