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
