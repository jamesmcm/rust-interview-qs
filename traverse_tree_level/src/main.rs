// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

struct Solution {
}


use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut curlevel: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut nextlevel: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut output: Vec<Vec<i32>> = Vec::new();

    if root.is_none() { return Vec::new() }
        
    curlevel.push(root.unwrap());

    loop {
    let mut thisoutput: Vec<i32> = Vec::new();
    while !curlevel.is_empty(){
        let node = curlevel.pop().unwrap();
        thisoutput.push(node.borrow().val);
        if let Some(x) = node.clone().borrow().left.clone() {
            nextlevel.push(x);
        }
        if let Some(x) = node.clone().borrow().right.clone() {
            nextlevel.push(x);
        }
    }
    output.push(thisoutput);
    nextlevel.reverse();
    if nextlevel.is_empty(){
        break;
    } else {
        curlevel = nextlevel.clone();
        nextlevel.clear();
    }
    }
    output
}
}
fn main () -> () {

}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_1() {
        let mut root = TreeNode::new(3);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
        {
        (&mut root.right).as_mut().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        }
        {
        (&mut root.right).as_mut().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        }

        assert_eq!(Solution::level_order(Some(Rc::new(RefCell::new(root)))), vec![vec![3], vec![9, 20], vec![15, 7]]);
    }

    #[test]
    fn test_2() {
        let mut root = TreeNode::new(1);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        {
        (&mut root.right).as_mut().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        }
        {
        (&mut root.left).as_mut().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        }

        assert_eq!(Solution::level_order(Some(Rc::new(RefCell::new(root)))), vec![vec![1], vec![2, 3], vec![4, 5]]);
    }

}
