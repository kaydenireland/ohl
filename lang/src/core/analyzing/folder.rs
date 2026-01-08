use crate::core::analyzing::stree::STree;
use crate::core::analyzing::operator::Operator;
use crate::core::logger::Logger;

pub struct ConstantFolder {
    pub log: Logger
}

impl ConstantFolder {

    pub fn new(_debug: bool) -> ConstantFolder {
        ConstantFolder { log: Logger::new(_debug) }
    }

    pub fn run(&mut self, tree: &mut STree) {
        self.fold_constants(tree);
    }

    fn fold_constants(&mut self, node: &mut STree) {
        match node {
            STree::EXPR { left, operator, right } => {
                self.fold_constants(left);
                self.fold_constants(right);

                // unary folding
                if let Some(folded) = self.fold_unary(operator, right) {
                    *node = folded;
                    return;
                }

                // binary folding
                if left.is_literal() && right.is_literal() {
                    if let Some(folded) = self.fold_binary(operator, left, right) {
                        *node = folded;
                    }
                }
            }

            STree::PRFX_EXPR { operator, right } => {
                self.fold_constants(right);
                if let Some(folded) = self.fold_unary(operator, right) {
                    *node = folded;
                }
            }

            STree::BLOCK { statements } => {
                for s in statements {
                    self.fold_constants(s);
                }
            }

            STree::IF_EXPR { condition, then_block, else_block } => {
                self.fold_constants(condition);
                self.fold_constants(then_block);
                if let Some(e) = else_block {
                    self.fold_constants(e);
                }
            }

            STree::WHILE_EXPR { condition, body } => {
                self.fold_constants(condition);
                self.fold_constants(body);
            }

            STree::FUNCTION { body, .. } => self.fold_constants(body),

            STree::START { functions } => {
                for f in functions {
                    self.fold_constants(f);
                }
            }

            _ => {}
        }
    }

    fn fold_binary(&self, operator: &Operator, l: &STree, r: &STree) -> Option<STree> {
        self.log.info("fold_binary()");

        match (operator, l, r) {
            // INT arithmetic
            (Operator::ADD, STree::LIT_INT { value: a }, STree::LIT_INT { value: b }) =>
                Some(STree::LIT_INT { value: a + b }),

            (Operator::SUBTRACT, STree::LIT_INT { value: a }, STree::LIT_INT { value: b }) =>
                Some(STree::LIT_INT { value: a - b }),

            (Operator::MULTIPLY, STree::LIT_INT { value: a }, STree::LIT_INT { value: b }) =>
                Some(STree::LIT_INT { value: a * b }),

            (Operator::DIVIDE, STree::LIT_INT { value: a }, STree::LIT_INT { value: b }) if *b != 0 =>
                Some(STree::LIT_INT { value: a / b }),

            (Operator::REMAINDER, STree::LIT_INT { value: a }, STree::LIT_INT { value: b }) if *b != 0 =>
                Some(STree::LIT_INT { value: a % b }),

            (Operator::POWER, STree::LIT_INT { value: a }, STree::LIT_INT { value: b }) if *b >= 0 =>
                Some(STree::LIT_INT { value: a.pow(*b as u32) }),

            (Operator::ROOT, STree::LIT_INT { value: a }, STree::LIT_INT { value: b })
                if *a >= 0 && *b > 0 =>
                    Some(STree::LIT_INT {
                        value: (*a as f64).powf(1.0 / *b as f64) as i32
                    }),



            // FLOAT arithmetic
            (Operator::ADD, STree::LIT_FLOAT { value: a }, STree::LIT_FLOAT { value: b }) =>
                Some(STree::LIT_FLOAT { value: a + b }),

            (Operator::SUBTRACT, STree::LIT_FLOAT { value: a }, STree::LIT_FLOAT { value: b }) =>
                Some(STree::LIT_FLOAT { value: a - b }),

            (Operator::MULTIPLY, STree::LIT_FLOAT { value: a }, STree::LIT_FLOAT { value: b }) =>
                Some(STree::LIT_FLOAT { value: a * b }),

            (Operator::DIVIDE, STree::LIT_FLOAT { value: a }, STree::LIT_FLOAT { value: b }) if *b != 0.0 =>
                Some(STree::LIT_FLOAT { value: a / b }),

            (Operator::POWER, STree::LIT_FLOAT { value: a }, STree::LIT_FLOAT { value: b }) =>
                Some(STree::LIT_FLOAT { value: a.powf(*b) }),

            (Operator::ROOT, STree::LIT_FLOAT { value: a }, STree::LIT_FLOAT { value: b }) if *a >= 0.0 && *b != 0.0 =>
                Some(STree::LIT_FLOAT { value: a.powf(1.0 / b) }),

            // BOOL logic
            (Operator::AND, STree::LIT_BOOL { value: a }, STree::LIT_BOOL { value: b }) =>
                Some(STree::LIT_BOOL { value: *a && *b }),

            (Operator::OR, STree::LIT_BOOL { value: a }, STree::LIT_BOOL { value: b }) =>
                Some(STree::LIT_BOOL { value: *a || *b }),

            (Operator::XOR, STree::LIT_BOOL { value: a }, STree::LIT_BOOL { value: b }) =>
                Some(STree::LIT_BOOL { value: *a ^ *b }),

            // INT comparisons
            (Operator::LESS_THAN, STree::LIT_INT { value: a }, STree::LIT_INT { value: b }) =>
                Some(STree::LIT_BOOL { value: a < b }),

            (Operator::GREATER_THAN, STree::LIT_INT { value: a }, STree::LIT_INT { value: b }) =>
                Some(STree::LIT_BOOL { value: a > b }),

            (Operator::NOT_LESS_THAN, STree::LIT_INT { value: a }, STree::LIT_INT { value: b }) =>
                Some(STree::LIT_BOOL { value: a >= b }),

            (Operator::NOT_GREATER_THAN, STree::LIT_INT { value: a }, STree::LIT_INT { value: b }) =>
                Some(STree::LIT_BOOL { value: a <= b }),

            // FLOAT comparisons
            (Operator::LESS_THAN, STree::LIT_FLOAT { value: a }, STree::LIT_FLOAT { value: b }) =>
                Some(STree::LIT_BOOL { value: a < b }),

            (Operator::GREATER_THAN, STree::LIT_FLOAT { value: a }, STree::LIT_FLOAT { value: b }) =>
                Some(STree::LIT_BOOL { value: a > b }),

            (Operator::NOT_LESS_THAN, STree::LIT_FLOAT { value: a }, STree::LIT_FLOAT { value: b }) =>
                Some(STree::LIT_BOOL { value: a >= b }),

            (Operator::NOT_GREATER_THAN, STree::LIT_FLOAT { value: a }, STree::LIT_FLOAT { value: b }) =>
                Some(STree::LIT_BOOL { value: a <= b }),

            (Operator::EQUAL, a, b) if a.is_literal() && b.is_literal() =>
                Some(STree::LIT_BOOL { value: a == b }),

            (Operator::NOT_EQUAL, a, b) if a.is_literal() && b.is_literal() =>
                Some(STree::LIT_BOOL { value: a != b }),


            _ => None,
        }
    }

    fn fold_unary(&self, operator: &Operator, node: &STree) -> Option<STree> {
        self.log.info("fold_unary()");

        match (operator, node) {
            // BOOLEAN
            (Operator::NOT, STree::LIT_BOOL { value }) =>
                Some(STree::LIT_BOOL { value: !value }),

            // INT
            (Operator::NEGATIVE, STree::LIT_INT { value }) =>
                Some(STree::LIT_INT { value: -*value }),

            // FLOAT
            (Operator::NEGATIVE, STree::LIT_FLOAT { value }) =>
                Some(STree::LIT_FLOAT { value: -*value }),

            (Operator::RECIPROCAL, STree::LIT_FLOAT { value }) if *value != 0.0 =>
                Some(STree::LIT_FLOAT { value: 1.0 / value }),


    _ => None,
}

    }
}
