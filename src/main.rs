use petgraph::{graphmap::GraphMap, Directed, Direction, dot::{Dot, Config}};

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
    Actions(Actions)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
enum Actions {
    Cook
}

fn main() {
    let mut deps: GraphMap<Food, f64, Directed> = GraphMap::new();
    deps.add_node(Food::Dishes(Dishes::HotDog));
    deps.add_node(Food::Ingredients(Ingredients::HotDogBun));
    deps.add_node(Food::Ingredients(Ingredients::HotDogWeiner));
    deps.add_node(Food::Ingredients(Ingredients::HotDogWeinerCooked));
    deps.add_node(Food::Actions(Actions::Cook));

    deps.add_edge(
        Food::Ingredients(Ingredients::HotDogWeinerCooked),
        Food::Dishes(Dishes::HotDog),
        1.,
    );
    deps.add_edge(
        Food::Ingredients(Ingredients::HotDogBun),
        Food::Dishes(Dishes::HotDog),
        1.,
    );
    deps.add_edge(
        Food::Ingredients(Ingredients::HotDogWeiner),
        Food::Ingredients(Ingredients::HotDogWeinerCooked),
        1.,
    );
    deps.add_edge(
        Food::Actions(Actions::Cook),
        Food::Ingredients(Ingredients::HotDogWeinerCooked),
        1.,
    );

    println!("{:?}", Dot::with_config(&deps, &[Config::EdgeNoLabel]));

    println!("Ingredients needed to make a hot dog");
    for node in deps.neighbors_directed(Food::Dishes(Dishes::HotDog), Direction::Incoming) {
        dbg!(node);
    }
    println!("");
    println!("These ingredients can make:");
    println!("Hot Dog Weiner can be used to make:");
    for node in deps.neighbors_directed(Food::Ingredients(Ingredients::HotDogWeinerCooked), Direction::Outgoing) {
        dbg!(node);
    }
    println!("Hot Dog Bun can be used to make:");
    for node in deps.neighbors_directed(Food::Ingredients(Ingredients::HotDogBun), Direction::Outgoing) {
        dbg!(node);
    }
}
