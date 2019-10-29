# food-dependency-graph

Sketch describing how to use `petgraph` to model relationships between foods and ingredients with a directed graph.

`petgraph` is used to create a graph:

```
digraph {
    0 [label="Dishes(HotDog)"]
    1 [label="Ingredients(HotDogBun)"]
    2 [label="Ingredients(HotDogWeiner)"]
    3 [label="Ingredients(HotDogWeinerCooked)"]
    4 [label="Actions(Cook)"]
    3 -> 0
    1 -> 0
    2 -> 3
    4 -> 3
}
```

Which can be rendered as:

![graphviz](https://user-images.githubusercontent.com/2590422/67796146-dd17cf00-fa55-11e9-95ea-272d88ec50b8.png)

## Usage

```
// Install Rust if you don't have it: https://www.rust-lang.org/tools/install
git clone git@github.com:wbprice/food-dependency-graph.git && cd food-dependency-graph
cargo run
```
