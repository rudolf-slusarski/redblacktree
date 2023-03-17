// #![deny(clippy::pedantic)]

use slab::Slab;
use std::cmp;
use std::ops::{Index, IndexMut};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Pointer(usize);

impl Pointer {
    #[inline]
    pub fn null() -> Pointer {
        Pointer(!0)
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        *self == Pointer::null()
    }
}

impl Index<Pointer> for RedBlackTree {
    type Output = Node;

    fn index(&self, index: Pointer) -> &Node {
        &self.slab[index.0]
    }
}

impl IndexMut<Pointer> for RedBlackTree {
    fn index_mut(&mut self, index: Pointer) -> &mut Node {
        &mut self.slab[index.0]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug)]
pub struct Node {
    pub value: u32,
    pub right: Pointer,
    pub left: Pointer,
    pub parent: Pointer,
    pub color: Color,
}

pub struct RedBlackTree {
    pub slab: Slab<Node>,
    pub root: Pointer,
}

impl RedBlackTree {
    pub fn new() -> Self {
        RedBlackTree {
            slab: Slab::new(),
            root: Pointer::null(),
        }
    }

    pub fn height(&self) -> u32 {
        self.height_below(self.root)
    }

    fn height_below(&self, node: Pointer) -> u32 {
        if node.is_null() {
            0
        } else {
            let left = self.height_below(self[node].left);
            let right = self.height_below(self[node].right);
            cmp::max(left, right) + 1
        }
    }

    pub fn red_count(&self) -> u32 {
        self.red_below(self.root)
    }

    fn red_below(&self, node: Pointer) -> u32 {
        let mut count = 0;
        if self[node].color == Color::Red {
            count += 1;
        }
        if !self[node].right.is_null() {
            count += self.red_below(self[node].right);
        }
        if !self[node].left.is_null() {
            count += self.red_below(self[node].left);
        }

        count
    }

    pub fn insert(&mut self, val: u32) {
        if self.root.is_null() {
            self.root = Pointer(self.slab.insert(Node {
                value: val,
                right: Pointer::null(),
                left: Pointer::null(),
                parent: Pointer::null(),
                color: Color::Black,
            }));
        } else {
            let new_node = self.insert_node(val, self.root);
            if !new_node.is_null() {
                self.insert_fixup(new_node);
            }
        }
    }

    fn insert_fixup(&mut self, node: Pointer) {
        let parent = self[node].parent;
        if self[node].parent.is_null() {
            return self.insert_case1(node);
        }

        if self[parent].color == Color::Black {
            return self.insert_case2(node);
        }

        let uncle = self.uncle(node);

        if uncle.is_null() {
            return self.insert_case4(node);
        }
        if self[uncle].color == Color::Black {
            return self.insert_case4(node);
        }

        return self.insert_case3(node);
    }

    fn insert_case1(&mut self, node: Pointer) {
        self[node].color = Color::Black;
    }

    fn insert_case2(&mut self, _node: Pointer) {
        return;
    }

    fn insert_case3(&mut self, node: Pointer) {
        let parent = self[node].parent;
        let uncle = self.uncle(node);
        let grandparent = self[parent].parent;

        self[parent].color = Color::Black;
        self[uncle].color = Color::Black;
        self[grandparent].color = Color::Red;

        self.insert_fixup(grandparent);
    }

    fn insert_case4(&mut self, node: Pointer) {
        let parent = self[node].parent;
        let grandparent = self[parent].parent;

        let parent_left = self[parent].left;
        let parent_right = self[parent].right;

        let grandparent_left = self[grandparent].left;
        let grandparent_right = self[grandparent].right;

        let mut n = node;

        if !parent_right.is_null()
            && !grandparent_left.is_null()
            && (self[n].value == self[parent_right].value)
            && (self[parent].value == self[grandparent_left].value)
        {
            self.rotate_left(parent);
            n = self[n].left;
        } else if !parent_left.is_null()
            && !grandparent_right.is_null()
            && (self[n].value == self[parent_left].value)
            && (self[parent].value == self[grandparent_right].value)
        {
            self.rotate_right(parent);
            n = self[n].right;
        }

        let parent = self[n].parent;
        let grandparent = self[parent].parent;

        let parent_left = self[parent].left;

        if !parent_left.is_null() && self[n].value == self[parent_left].value {
            self.rotate_right(grandparent);
        } else {
            self.rotate_left(grandparent);
        }

        self[parent].color = Color::Black;
        self[grandparent].color = Color::Red;
    }

    fn uncle(&self, node: Pointer) -> Pointer {
        let parent = self[node].parent;
        if parent.is_null() {
            return Pointer::null();
        }

        let grandparent = self[parent].parent;

        if grandparent.is_null() {
            return Pointer::null();
        }

        let grandparent_left = self[grandparent].left;
        let grandparent_right = self[grandparent].right;

        if grandparent_left.is_null() || grandparent_right.is_null() {
            return Pointer::null();
        }

        if self[parent].value == self[grandparent_left].value {
            return grandparent_right;
        }

        return grandparent_left;
    }

    fn insert_node(&mut self, val: u32, node: Pointer) -> Pointer {
        let node_value = self[node].value;
        let left = self[node].left;
        let right = self[node].right;

        if val == node_value {
            return Pointer::null();
        } else if val > node_value {
            if right.is_null() {
                self[node].right = Pointer(self.slab.insert(Node {
                    value: val,
                    right: Pointer::null(),
                    left: Pointer::null(),
                    parent: node,
                    color: Color::Red,
                }));
                return self[node].right;
            } else {
                return self.insert_node(val, right);
            }
        } else if left.is_null() {
            self[node].left = Pointer(self.slab.insert(Node {
                value: val,
                right: Pointer::null(),
                left: Pointer::null(),
                parent: node,
                color: Color::Red,
            }));
            return self[node].left;
        } else {
            return self.insert_node(val, left);
        }
    }

    fn rotate_left(&mut self, current: Pointer) {
        let right = self[current].right;

        if right.is_null() {
            return;
        }

        let right_left = self[right].left;
        let parent = self[current].parent;

        self[current].right = right_left;

        if !right_left.is_null() {
            self[right_left].parent = current;
        }

        self[current].parent = right;
        self[right].left = current;

        self[right].parent = parent;

        if parent.is_null() {
            self.root = right;
        } else {
            let parent_right = self[parent].right;
            if parent_right.is_null() {
                self[parent].left = right;
            } else if self[parent_right].value == self[current].value {
                self[parent].right = right;
            } else {
                self[parent].left = right;
            }
        }
    }

    fn rotate_right(&mut self, current: Pointer) {
        let left = self[current].left;

        if left.is_null() {
            return;
        }

        let left_right = self[left].right;
        let parent = self[current].parent;

        self[current].left = left_right;

        if !left_right.is_null() {
            self[left_right].parent = current;
        }

        self[current].parent = left;
        self[left].right = current;

        self[left].parent = parent;

        if parent.is_null() {
            self.root = left;
        } else {
            let parent_left = self[parent].left;
            if parent_left.is_null() {
                self[parent].right = left;
            } else if self[parent_left].value == self[current].value {
                self[parent].left = left;
            } else {
                self[parent].right = left;
            }
        }
    }
    pub fn remove(&mut self, val: u32) {
        if !self.get_node(val).is_null() {
            self.remove_cheat(val);
        }
    }

    fn remove_cheat(&mut self, val: u32) {
        let mut new_tree = RedBlackTree::new();
        for i in 0..self.slab.len() {
            if self.slab[i].value != val {
                new_tree.insert(self.slab[i].value);
            }
        }
        self.slab = new_tree.slab;
        self.root = new_tree.root;
    }

    fn get_node(&self, val: u32) -> Pointer {
        let node = self.choose_node(self.root, val);
        if node.is_null() {
            println!("no such node");
        }
        node
    }

    fn choose_node(&self, node: Pointer, val: u32) -> Pointer {
        if node.is_null() {
            return Pointer::null();
        }
        match self[node].value.cmp(&val) {
            cmp::Ordering::Equal => node,
            cmp::Ordering::Less => self.choose_node(self[node].right, val),
            cmp::Ordering::Greater => self.choose_node(self[node].left, val),
        }
    }

    pub fn print(&self) {
        if !&self.root.is_null() {
            let mut lines = Vec::new();
            self.print_node(&mut lines, "", self.root, false);
            for line in lines {
                println!("{line}");
            }
        }
    }

    fn print_node(&self, lines: &mut Vec<String>, prefix: &str, node: Pointer, is_left: bool) {
        let color_str = match self[node].color {
            Color::Red => "\x1b[31mR\x1b[0m",
            Color::Black => "\x1b[30mB\x1b[0m",
        };
        let mut line = String::new();
        line += prefix;
        line += if is_left { "├── " } else { "└── " };
        line += &format!("{} {}", color_str, self[node].value);
        lines.push(line);
        if !&self[node].left.is_null() {
            self.print_node(
                lines,
                &(prefix.to_owned() + if is_left { "│   " } else { "    " }),
                self[node].left,
                true,
            );
        }
        if !&self[node].right.is_null() {
            self.print_node(
                lines,
                &(prefix.to_owned() + if is_left { "│   " } else { "    " }),
                self[node].right,
                false,
            );
        }
    }
}
