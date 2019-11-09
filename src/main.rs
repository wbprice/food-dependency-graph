use petgraph::{
    dot::{Config, Dot},
    graphmap::GraphMap,
    Directed, Direction,
};

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub enum Dishes {
    HotDog,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub enum Ingredients {
    HotDogBun,
    HotDogWeiner,
    HotDogWeinerCooked,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub enum Food {
    Dishes(Dishes),
    Ingredients(Ingredients),
    Actions(Actions),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub enum Actions {
    CookIngredient,
    ChopIngredient,
}

#[derive(Default)]
pub struct Cookbook {
    graph: GraphMap<Food, f64, Directed>,
}

impl Cookbook {
    pub fn new() -> Cookbook {
        let mut graph = GraphMap::new();

        // Add nodes for actions
        graph.add_node(Food::Actions(Actions::CookIngredient));

        // Add nodes for ingredients and dishes
        graph.add_node(Food::Ingredients(Ingredients::HotDogBun));
        graph.add_node(Food::Ingredients(Ingredients::HotDogWeiner));
        graph.add_node(Food::Ingredients(Ingredients::HotDogWeinerCooked));
        graph.add_node(Food::Dishes(Dishes::HotDog));

        // Add edges
        graph.add_edge(
            Food::Actions(Actions::CookIngredient),
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

    pub fn actions(&self, food_node: Food) -> Vec<Actions> {
        let mut actions: Vec<Actions> = vec![];

        for node in self
            .graph
            .neighbors_directed(food_node, Direction::Incoming)
        {
            if let Food::Actions(action) = node {
                actions.push(action);
            }
        }

        actions
    }

    pub fn ingredients(&self, food_node: Food) -> Vec<Ingredients> {
        let mut ingredients: Vec<Ingredients> = vec![];

        for node in self
            .graph
            .neighbors_directed(food_node, Direction::Incoming)
        {
            if let Food::Ingredients(ingredient) = node {
                ingredients.push(ingredient);
            }
        }

        ingredients
    }

    pub fn makes(&self, food_node: Food) -> Vec<Dishes> {
        let mut dishes: Vec<Dishes> = vec![];

        for node in self
            .graph
            .neighbors_directed(food_node, Direction::Outgoing)
        {
            if let Food::Dishes(dish) = node {
                dishes.push(dish);
            }
        }

        dishes
    }

    pub fn get_dish_from_ingredients(&self, ingredients: Vec<Ingredients>) -> Option<Dishes> {
        if let Some(first_ingredient) = ingredients.first() {
            let other_ingredients = ingredients[1..].to_vec();
            let potential_dishes = self.makes(Food::Ingredients(*first_ingredient));

            for ingredient in other_ingredients.into_iter() {
                let other_potential_dishes = self.makes(Food::Ingredients(ingredient));

                // If this ingredient doesn't have an edge to _any_ dishes, that's an issue.
                if other_potential_dishes.is_empty() {
                    return None;
                }

                // Otherwise, confirm that this ingredient could be used to make a dish in
                // the list of potential dishes
                for dish in other_potential_dishes.iter() {
                    if !potential_dishes.contains(dish) {
                        return None;
                    }
                }
            }

            return Some(potential_dishes[0]);
        }
        None
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
    println!();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_ingredients_needed_for_hot_dog() {
        let cookbook = Cookbook::new();
        let ingredients = cookbook.ingredients(Food::Dishes(Dishes::HotDog));
        assert_eq!(ingredients.len(), 2);
        assert!(ingredients.contains(&Ingredients::HotDogBun));
        assert!(ingredients.contains(&Ingredients::HotDogWeinerCooked));
    }

    #[test]
    fn get_dishes_from_ingredient() {
        let cookbook = Cookbook::new();
        let dishes = cookbook.makes(Food::Ingredients(Ingredients::HotDogBun));
        assert_eq!(dishes.len(), 1);
        assert!(dishes.contains(&Dishes::HotDog));
    }

    #[test]
    fn get_dish_from_ingredients_coherent_dish() {
        let cookbook = Cookbook::new();
        let ingredients = vec![Ingredients::HotDogBun, Ingredients::HotDogWeinerCooked];
        let result = cookbook.get_dish_from_ingredients(ingredients);

        assert!(result.is_some());
        assert_eq!(result.unwrap(), Dishes::HotDog);
    }

    #[test]
    fn get_dish_from_ingredients_incoherent_dish() {
        let cookbook = Cookbook::new();
        let ingredients = vec![Ingredients::HotDogBun, Ingredients::HotDogWeiner];
        let result = cookbook.get_dish_from_ingredients(ingredients);

        assert!(result.is_none());
    }
}
