# food-dependency-graph

Sketch describing how to use `petgraph` to model relationships between foods and ingredients with a directed graph.

`petgraph` is used to create a graph:

```
digraph {
    0 [label="Actions(CookIngredient)"]
    1 [label="Ingredients(HotDogBun)"]
    2 [label="Ingredients(HotDogLink)"]
    3 [label="Ingredients(HotDogLinkCooked)"]
    4 [label="Dishes(HotDog)"]
    0 -> 3
    2 -> 3
    3 -> 4
    1 -> 4
}
```

Which can be rendered as:

![food-dependency-graph](https://user-images.githubusercontent.com/2590422/69379628-ccd2c880-0c7e-11ea-87bb-6b4d3f36b1f4.png)

## Usage

```
// Install Rust if you don't have it: https://www.rust-lang.org/tools/install
git clone git@github.com:wbprice/food-dependency-graph.git && cd food-dependency-graph
cargo run
```
