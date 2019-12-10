use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: Symbol,
    children: (Option<Rc<RefCell<Node>>>, Option<Rc<RefCell<Node>>>),
}

impl Node {
    pub fn new(value: Symbol, children: (Option<Rc<RefCell<Node>>>, Option<Rc<RefCell<Node>>>)) -> Self {
        Self {
            value,
            children,
        }
    }
}

#[derive(Debug)]
enum Symbol {
    Operator(char),
    Value(u32),
}

fn split_with_matches<'a, F>(s: &'a str, f: F) -> Vec<&'a str>
where
    F: Fn(char) -> bool,
{
    let mut out: Vec<&'a str> = vec![];
    let mut prevpos: usize = 0;

    for (pos, c) in s.bytes().enumerate() {
        if f(c as char) {
            if prevpos != pos {
                out.push(&s[prevpos..pos]);
            }
            out.push(&s[pos..pos + 1]);
            prevpos = pos + 1;
        }
    }
    if prevpos != s.len() {
        out.push(&s[prevpos..]);
    }
    out
}

fn parse(s: &str) -> Node {
    let operators: Vec<char> = vec!['/', '*', '+', '-'];
    let mut root: Option<Node> = None;
    let mut lastNode: Option<std::cell::RefMut<Node>> = None;

    for c in split_with_matches(
        &s.chars().filter(|c| !c.is_whitespace()).collect::<String>(),
        |c: char| operators.contains(&c),
    ) {
        let symbol: Symbol = match c {
            c if operators.contains(&c.parse::<char>().unwrap_or('9')) => {
                Symbol::Operator(c.parse::<char>().unwrap())
            }
            x => Symbol::Value(x.parse::<u32>().unwrap()),
        };

        println!("{:?}", symbol);
        // Empty tree
        if let None = root {
            root = Some(Node::new(symbol, (None, None)));
            continue;
        }

        // New tree
        if let Symbol::Value(_) = (&root).as_ref().unwrap().value {
            root = Some(Node::new(symbol, (Some(Rc::new(RefCell::new(root.unwrap()))), None)));
            continue;
        }
        if let None = (&root).as_ref().unwrap().children.1 {
            (&mut root).as_mut().unwrap().children.1 =
                Some(Rc::new(RefCell::new(Node::new(symbol, (None, None)))));
            continue;
        }

        // Traverse tree - values are leaf nodes
        // Lowest precedence should be at root
        // Traverse operators until we find operator that is higher precedence - we should be parent of that subtree
        if let Symbol::Operator(o) = symbol {
            if let Symbol::Operator(o2) = root.as_ref().unwrap().value {
                if operators.iter().position(|&r| r == o) >= operators.iter().position(|&r| r == o2)
                {
                    root = Some(Node::new(symbol, (Some(Rc::new(RefCell::new(root.unwrap()))), None)));
                    lastNode = Some(RefCell::new(root.as_ref().unwrap()).borrow_mut());
                    continue;
                } else {
                    // TODO: if children are values (leaf nodes)
                    // TODO: loop here
                    let movechild = ((&mut root).as_mut().unwrap().children).1.take();
                    (&mut root).as_mut().unwrap().children.1 =
                        Some(Rc::new(RefCell::new(Node::new(symbol, (movechild, None)))));
                    //lastNode = Some(&*(&root.as_ref().unwrap().children.1.as_ref()).as_ref().unwrap().borrow_mut());
                    lastNode = Some(
                        (*root.as_ref().unwrap().children.1.as_ref().unwrap()).borrow_mut()
                        );

                    continue;
                }
            }
        } else {
            // Symbol is Value not Operator
            // Fill last none with value
            lastNode.unwrap().children.1 = Some(Rc::new(RefCell::new(Node::new(symbol, (None, None)))));
        }
    }

    let out = root.unwrap();
    out
}

fn main() {
    println!("{:?}", parse("25 + 3 +"));
}
