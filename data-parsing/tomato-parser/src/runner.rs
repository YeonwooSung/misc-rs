use std::collections::HashMap;
use crate::parser::tomato;
use crate::node::Node;

// define the program context
struct Context {
    // hashmap to store variables and their values as key-value pairs
    vars: HashMap<String, i64>,
}

// execute each ndoe in the syntax tree
fn run_node(ctx: &mut Context, node: Node) -> i64 {
    // 어떤 타입의 노드인지 판단
    match node {
        Node::Number(v) => v, // return a number
        Node::Calc(op, l, r) => { // calculation expression
            calc_op(op, run_node(ctx, *l), run_node(ctx, *r))
        },
        Node::GetVar(name) => { // getting a number
            match ctx.vars.get(&name) {
                Some(v) => *v,
                None => 0,
            }
        },
        Node::SetVar(name, node) => { // setting a variable
            let val = run_node(ctx, *node);
            ctx.vars.insert(name, val);
            val
        },
        Node::If(cond, true_n, false_n) => { // if statement
            let cond_v = run_node(ctx, *cond);
            if cond_v > 0 {
                run_nodes(ctx, &*true_n)
            } else {
                run_nodes(ctx, &*false_n)
            }
        },
        Node::For(name, start, end, body) => { // for statement
            let mut r = 0;
            let nodes = *body;
            for i in start..=end {
                ctx.vars.insert(name.clone(), i);
                r = run_nodes(ctx, &nodes);
            }
            r
        },
        Node::PrintStr(v) => { println!("{}", v); 0},
        Node::Print(node) => { // print statement
            let v = run_node(ctx, *node);
            println!("{}", v);
            v
        },
        _ => 0,
    }
}

// calculate based on operator
fn calc_op(op: char, val_l:i64, val_r:i64) -> i64 {
    match op {
        '+' => val_l + val_r,
        '-' => val_l - val_r,
        '*' => val_l * val_r,
        '/' => val_l / val_r,
        '%' => val_l % val_r,
        '=' => if val_l == val_r {1} else {0},
        '!' => if val_l != val_r {1} else {0},
        '>' => if val_l > val_r {1} else {0},
        'g' => if val_l >= val_r {1} else {0},
        '<' => if val_l < val_r {1} else {0},
        'l' => if val_l <= val_r {1} else {0},
        _ => 0,
    }
}

// run all nodes
fn run_nodes(ctx: &mut Context, nodes: &Vec<Node>) -> i64 {
    let mut result = 0;
    nodes.iter().for_each(|node| {
        result = run_node(ctx, node.clone())});
    result
}

// execution entry point - a basic REPL
pub fn run(src: &str) -> i64 {
    let nodes = tomato::parse(src).unwrap(); // parse (read & evaluate)
    let mut ctx = Context{vars:HashMap::new()};
    run_nodes(&mut ctx, &nodes) // run (play & loop)
}
