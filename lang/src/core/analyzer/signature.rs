use crate::core::analyzer::analyzer::Analyzer;
use crate::core::converter::stree::STree;

#[derive(Debug, Clone)]
pub struct ClassSignature {
    pub name: String,
    pub functions: Vec<FunctionSignature>,
    pub used: bool
}

impl ClassSignature {
    pub fn new(name: String, functions: Vec<FunctionSignature>, used: bool) -> ClassSignature {
        ClassSignature {
            name,
            functions,
            used
        }
    }

    pub fn used(&mut self) {
        self.used = true;
    }
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub class_name: String,
    pub name: String,
    pub parameters: Vec<VariableType>,
    pub return_type: VariableType,
    pub called: bool
}

impl FunctionSignature {
    pub fn new(class_name: String, name: String, parameters: Vec<VariableType>, return_type: VariableType, called: bool) -> FunctionSignature {
        FunctionSignature {
            class_name,
            name,
            parameters,
            return_type,
            called
        }
    }

    pub fn call(&mut self) {
        self.called = true;
    }

    pub fn key(&self) -> String {
        format!("{}::{}", self.class_name, self.name)
    }
}


#[derive(Debug, Clone)]
pub struct VariableSignature {
    pub var_type: VariableType,
    pub used: bool,
    pub mutable: bool
}

impl VariableSignature {
    pub fn new(var_type: VariableType, used: bool, mutable: bool) -> VariableSignature {
        VariableSignature {
            var_type,
            used,
            mutable
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    INT,
    FLOAT,
    CHAR,
    STRING,
    BOOLEAN,

    FUNCTION,
    CLASS,

    OBJECT,
    VOID,

    UNKNOWN
}

impl Analyzer {
    // TODO: Use Logger

    pub fn collect_signatures(&mut self, tree: &STree) {

        match tree {
            STree::START { classes } => {
                for class in classes {
                    self.collect_class_signatures(class);
                }
            }

            _ => {}
        }
    }

    pub fn collect_class_signatures(&mut self, node: &STree) {
        match node {
            STree::CLASS { scope, name, body } => {
                let func_sigs = self.collect_function_signatures(name.clone(), body);

                let mut used = false;

                for func_sig in func_sigs.clone() {
                    if func_sig.called {
                        used = true;
                    }
                }

                self.classes.insert(
                    name.clone(),
                    ClassSignature::new(
                        name.clone(),
                        func_sigs,
                        used
                    )
                );
            }

            _ => {}
        }
    }


    pub fn collect_function_signatures(&mut self, class_name: String, node: &STree) -> Vec<FunctionSignature> {
        match node {

            STree::CLASS_BODY { variables, functions} => {
                let mut func_sigs: Vec<FunctionSignature> = Vec::new();
                for function in functions {
                    func_sigs.extend(self.collect_function_signatures(class_name.clone(), function));
                }
                func_sigs
            },

            STree::FUNCTION { return_type, name, params, .. } => {
                let mut param_types = Vec::new();
                for (_, token_type) in params {
                    param_types.push(token_type.clone());
                }

                let mut func_sigs: Vec<FunctionSignature> = Vec::new();
                let sig = FunctionSignature::new(
                    class_name.clone(),
                    name.clone(),
                    param_types,
                    return_type.clone(),
                    name == "main"
                );

                func_sigs.push(sig.clone());


                self.functions.insert(
                    sig.key(),
                    sig.clone()
                );

                func_sigs
            },

            _ => Vec::new()
        }
    }
}