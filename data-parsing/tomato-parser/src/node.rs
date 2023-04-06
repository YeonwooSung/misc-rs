// define the grammar elements as Node type
#[derive(Debug, Clone)]
pub enum Node {
    Nop, // no operation
    Number(i64), // number value
    Calc(char, Box<Node>, Box<Node>), // calculation expression
    If(Box<Node>, Box<Vec<Node>>, Box<Vec<Node>>), // if statement
    For(String, i64, i64, Box<Vec<Node>>), // for statement
    Print(Box<Node>), // print statement for print out a result of the expression
    PrintStr(String), // print for print out the constant value
    SetVar(String, Box<Node>), // setting a variable
    GetVar(String), // getting a variable
}


impl Node {)
    // A function that returns Node::Calc type
    pub fn calc(op: char, l: Node, r: Node) -> Node {
        Node::Calc(op, Box::new(l), Box::new(r))
    }
    // A function that returns Node::If type
    pub fn if_(cond: Node, t: Vec<Node>, f: Vec<Node>) -> Node {
        Node::If(Box::new(cond), Box::new(t), Box::new(f))
    }
}
