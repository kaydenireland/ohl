use crate::core::lexer::token::Token;

pub struct MTree {
    pub token: Token,
    pub children: Vec<MTree>
}



impl MTree {
    pub fn new(token: Token) -> MTree {
        MTree {
            token,
            children: vec![]
        }
    }

    pub fn _push(&mut self, tree: MTree) {
        self.children.push(tree);
    }

    pub fn node_string(&self, print_whole: bool) -> String {
        if print_whole {
            format!("{:?}", self.token)
        } else {
            format!("{:?}", self.token.token_type)
        }
    }

    fn print_recursively(&self, level: usize, print_whole: bool) {
        let shift = 2 * level;
        print!("{:1$}", "", shift);
        println!("{}", self.node_string(print_whole));
        for child in &self.children {
            child.print_recursively(level + 1, print_whole);
        }
    }

    pub fn print(&self, print_whole: bool) {
        self.print_recursively(0, print_whole);
    }
}