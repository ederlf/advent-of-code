use std::collections::HashMap;
use petgraph::graph::Graph;
use petgraph::algo;
use  petgraph::graph::NodeIndex;


#[derive(Debug)]
struct Node {
    value: i32,
}

fn add_edge(current: (i32, i32), new: (i32, i32), graph: &mut Graph<Node, i32>, nodes: &HashMap<(i32, i32), NodeIndex>) {
    let add_node = nodes.get(&new);
    if add_node != None {
        // We set the base 1000 to avoid negative weights, so we don't have to use Bellman-Ford
        let base = 1000;
        let node = *nodes.get(&current).unwrap();
        let add_node_value = *add_node.unwrap();
        let diff = base - (graph[node].value - graph[add_node_value].value);
        if diff <= 1001 {
            graph.add_edge(node, add_node_value , diff);
        }
    }
}


fn hike(input: String, one_source: bool) -> String {
    let mut graph: Graph<Node, i32>= Graph::new();
    let grid: Vec<&[u8]> = input.lines().map(|x| x.as_bytes()).collect();
    let mut nodes = HashMap::new();
    let row_len = grid.len();
    let col_len = grid[0].len();
    let mut destination = (0, 0);
    let mut sources = Vec::new();
    for x in 0..row_len {
        for y in 0..col_len {
            let mut value = grid[x][y];
            if value == 69 {
                value = "z".as_bytes()[0];
                destination = (x as i32, y as i32);
            } else if one_source && value == 83 {
                value = "a".as_bytes()[0];
                sources.push((x as i32, y as i32));
            } else if !one_source && value == 97 {
                sources.push((x as i32, y as i32));
            }
            let node = Node{value:value as i32 };
            nodes.insert((x as i32, y as i32), graph.add_node(node));
        }
    }

    for x in 0..row_len as i32 {
        for y in 0..col_len as i32 {
            let current = (x, y);
            let up = (x-1, y);
            let down = (x+1, y);
            let right = (x, y +1);
            let left = (x, y - 1);

            add_edge(current, up, &mut graph, &nodes);
            add_edge(current, down, &mut graph, &nodes);
            add_edge(current, right, &mut graph, &nodes);
            add_edge(current, left, &mut graph, &nodes);
        }
    }

    let destination_node = *nodes.get(&destination).unwrap();
    let mut min_steps = 1000;
    for source in sources {
        let origin = *nodes.get(&source).unwrap();

        let path = algo::astar(
            &graph,
            origin,
            |n| n == destination_node,
            |e| *e.weight(),
            |_| 0,
        );
        if path != None {
            let full_path = path.unwrap().1;
            let steps = full_path.len() - 1;
            if steps < min_steps {
                min_steps = steps;
            }
        }
    }
    min_steps.to_string()
}

fn part1(input: String) -> String {
    hike(input, true)
}

fn part2(input: String) -> String {
    hike(input, false)
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
