use std::collections::HashMap;
use crate::parser::tomato;
use crate::node::Node;

struct Context {
    vars: HashMap<String, i32>,
    output: String,
}

fn run_node(ctx: &mut Context, node: Node) -> i32 {
    match node {
        Node::Number(v) => v,
        Node::Calc(op, l, r) => {
            calc_op(op, run_node(ctx, *l), run_node(ctx, *r))
        },
        Node::GetVar(name) => {
            match ctx.vars.get(&name) {
                Some(v) => *v,
                None => 0,
            }
        },
        Node::SetVar(name, node) => {
            let val = run_node(ctx, *node);
            ctx.vars.insert(name, val);
            val
        },
        Node::If(cond, true_n, false_n) => {
            let cond_v = run_node(ctx, *cond);
            if cond_v > 0 {
                run_nodes(ctx, &*true_n)
            } else {
                run_nodes(ctx, &*false_n)
            }
        },
        Node::For(name, start, end, body) => {
            let mut r = 0;
            let nodes = *body;
            for i in start..=end {
                ctx.vars.insert(name.clone(), i);
                r = run_nodes(ctx, &nodes);
            }
            r
        },
        Node::PrintStr(v) => {
            ctx.output += &format!("{}\n", v);
            0 
        },
        Node::Print(node) => {
            let v = run_node(ctx, *node);
            ctx.output += &format!("{}\n", v);
            v
        },
        _ => 0,
    }
}

fn calc_op(op: char, val_l:i32, val_r:i32) -> i32 {
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
fn run_nodes(ctx: &mut Context, nodes: &Vec<Node>) -> i32 {
    let mut result = 0;
    nodes.iter().for_each(|node| {
        result = run_node(ctx, node.clone())});
    result
}
pub fn run(src: &str) -> String {
    let nodes = match tomato::parse(src) {
        Ok(res) => res,
        Err(e) => return e.to_string(),
    };
    let mut ctx = Context{
        vars:HashMap::new(), 
        output:String::new()
    };
    let r = run_nodes(&mut ctx, &nodes);
    if ctx.output == "" {
        return format!("{}", r);
    } else {
        return ctx.output.clone();
    }
}

