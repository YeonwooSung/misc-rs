peg::parser!( grammar calc() for str {
    // add rule for evaluation
    pub rule eval() -> i64 
        = expr()

    // add rule for addition and subtraction
    rule expr() -> i64
        = l:term() "+" r:expr()   { l + r }
        / l:term() "-" r:expr()   { l - r }
        / term()

    // add rule for multiplication and division
    rule term() -> i64
        = l:value() "*" r:term() { l * r }
        / l:value() "/" r:term() { l / r }
        / v:value()

    // add rule for value or expression in parentheses
    rule value() -> i64
        = number()                  // number
        / "(" v:expr() ")" { v }   // expression in parentheses

    // add rule for numbers
    rule number() -> i64
        = n:$(['0'..='9']+) 
        { n.parse().unwrap() }
});

fn main() {
    println!("{}", calc::eval("1+2*3").unwrap());
    println!("{}", calc::eval("(1+2)*3").unwrap());
    println!("{}", calc::eval("100/2-1").unwrap());    
}
