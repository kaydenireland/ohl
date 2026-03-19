use inkwell::values::BasicValueEnum;
use crate::core::converter::stree::STree;
use crate::core::ir::codegen::CodeGen;

impl<'ctx> CodeGen<'ctx> {

    pub fn compile_statement(&mut self, node: &STree) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        self.logger.info("compile_statement()");
        self.logger.indent_inc();

        match node {

            STree::RETURN_STMT { expression } => {
                let val = if let Some(expr) = expression {
                    self.compile_expression(expr)?
                } else {
                    BasicValueEnum::IntValue(self.context.i32_type().const_int(0, false))
                };
                self.builder.build_return(Some(&val)).unwrap();
                self.logger.indent_dec();
                Ok(Some(val))
            },

            STree::VAR_STMT { id, expression, var_type, .. } => {
                let val = self.compile_expression(expression)?;
                let func = self.current_fn.unwrap();

                let llvm_type = self.llvm_type(var_type)?;
                let alloca = self.create_entry_block_alloca(func, id, llvm_type);

                self.builder.build_store(alloca, val).unwrap();
                self.variables.insert(id.clone(), (alloca, llvm_type));
                self.logger.indent_dec();
                Ok(Some(val))
            },

            STree::ASSIGN_STMT { id, expression } => {
                let val = self.compile_expression(expression)?;
                let (ptr, expected_typ) = self.variables.get(id).ok_or(format!("Undefined var {}", id))?;

                let store_val = match (val, expected_typ) {
                    (BasicValueEnum::IntValue(i), inkwell::types::BasicTypeEnum::IntType(t))
                    if *t == self.context.bool_type() => {
                        self.builder
                            .build_int_truncate(i, self.context.bool_type(), "to_bool")
                            .unwrap()
                            .into()
                    },
                    _ => val,
                };

                self.builder.build_store(*ptr, store_val).unwrap();
                self.logger.indent_dec();
                Ok(Some(val))
            },

            STree::BLOCK { statements } => {
                let mut last = None;
                for s in statements {
                    last = self.compile_statement(s)?;
                }
                self.logger.indent_dec();
                Ok(last)
            },

            STree::VAR_TYPE { .. } => Ok(None),
            STree::BLANK_STMT => Ok(None),
            STree::NULL => Ok(None),

            STree::PRINT { expression } => {
                let val = self.compile_expression(expression)?;
                self.build_print(&[val])?;
                Ok(None)
            },
            STree::LIT_INT { .. }
            | STree::LIT_FLOAT { .. }
            | STree::LIT_BOOL { .. }
            | STree::ID { .. }
            | STree::EXPR { .. }
            | STree::PRFX_EXPR { .. } => {
                let v = self.compile_expression(node)?;
                Ok(Some(v))
            }

            _ => {
                self.logger.indent_dec();
                Err(format!("Invalid statement node: {:?}", node))
            }
        }
    }

}