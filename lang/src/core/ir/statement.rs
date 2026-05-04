use inkwell::values::BasicValueEnum;
use crate::core::converter::stree::STree;
use crate::core::ir::codegen::CodeGen;
use crate::core::ir::statement;

impl<'ctx> CodeGen<'ctx> {

    pub fn compile_statement(&mut self, node: &STree) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        self.logger.info("compile_statement()");
        self.logger.indent_inc();

        match node {

            STree::RETURN_STMT { expression } => {
                let func = self.current_fn.unwrap();
                let ret_type = func.get_type().get_return_type();

                match (ret_type, expression) {
                    (None, Some(_)) => {
                        Err("Cannot return a value from a null (void) function".into())
                    }

                    (None, None) => {
                        self.builder.build_return(None).unwrap();
                        Ok(None)
                    }

                    (Some(_), Some(expr)) => {
                        let val = self.compile_expression(expr)?;
                        self.builder.build_return(Some(&val)).unwrap();
                        Ok(Some(val))
                    }

                    (Some(_), None) => {
                        Err("Return value required for non-null function".into())
                    }
                }
            }

            STree::VAR_DECL { id, expression, var_type, .. } => {
                let val = self.compile_expression(expression)?;
                let func = self.current_fn.unwrap();

                let llvm_type = self.llvm_type(var_type)?;
                let alloca = self.create_entry_block_alloca(func, id, llvm_type);

                self.builder.build_store(alloca, val).unwrap();
                self.variables.insert(id.clone(), (alloca, llvm_type));
                self.logger.indent_dec();
                Ok(Some(val))
            },

            STree::VAR_ASSIGN { id, expression } => {
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

            STree::IF_STMT { condition, then_block, else_block } => {
                self.logger.info("compile_if()");
                self.logger.indent_inc();

                let cond_val = self.compile_expression(condition)?;

                let condition_bool = match cond_val {
                    BasicValueEnum::IntValue(i) if i.get_type() == self.context.bool_type() => i,
                    _ => return Err("Condition must be boolean".into()),
                };

                let function = self.current_fn.unwrap();

                let then_bb = self.context.append_basic_block(function, "then");
                let merge_bb = self.context.append_basic_block(function, "merge");

                let else_bb = else_block
                    .as_ref()
                    .map(|_| self.context.append_basic_block(function, "else"));

                // Initial branch
                match else_bb {
                    Some(else_bb) => {
                        self.builder
                            .build_conditional_branch(condition_bool, then_bb, else_bb)
                            .unwrap();
                    }
                    None => {
                        self.builder
                            .build_conditional_branch(condition_bool, then_bb, merge_bb)
                            .unwrap();
                    }
                }

                // Then
                self.builder.position_at_end(then_bb);
                self.compile_statement(then_block)?;
                if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                    self.builder.build_unconditional_branch(merge_bb).unwrap();
                }

                // Else
                if let Some(else_block) = else_block {
                    let else_bb = else_bb.unwrap();
                    self.builder.position_at_end(else_bb);
                    self.compile_statement(else_block)?;
                    if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                        self.builder.build_unconditional_branch(merge_bb).unwrap();
                    }
                }

                // Merge
                self.builder.position_at_end(merge_bb);

                self.logger.indent_dec();
                Ok(None)
            },

            STree::WHILE_STMT { condition, body } => {
                self.logger.info("compile_while()");
                self.logger.indent_inc();

                let function = self.current_fn.unwrap();
                let cond_bb = self.context.append_basic_block(function, "while_cond");
                let body_bb = self.context.append_basic_block(function, "while_body");
                let end_bb = self.context.append_basic_block(function, "while_end");

                if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                    self.builder.build_unconditional_branch(cond_bb).unwrap();
                }

                // Condition
                self.builder.position_at_end(cond_bb);
                let cond_val = self.compile_expression(condition)?;

                let cond_bool = match cond_val {
                    BasicValueEnum::IntValue(i) if i.get_type() == self.context.bool_type() => i,
                    _ => return Err("While condition must be boolean".into()),
                };
                self.builder
                    .build_conditional_branch(cond_bool, body_bb, body_bb)
                    .unwrap();

                // Body
                self.builder.position_at_end(body_bb);

                self.loop_stack.push((end_bb, cond_bb, cond_bb));
                let result = self.compile_statement(body);
                self.loop_stack.pop();
                result?;

                if self.builder
                    .get_insert_block()
                    .unwrap()
                    .get_terminator()
                    .is_none()
                {
                    self.builder.build_unconditional_branch(cond_bb).unwrap();
                }

                // End
                self.builder.position_at_end(end_bb);

                self.logger.indent_dec();

                Ok(None)
            },

            STree::DO_WHILE_STMT { body, condition } => {
                let function = self.current_fn.unwrap();

                let body_bb = self.context.append_basic_block(function, "do_body");
                let cond_bb = self.context.append_basic_block(function, "do_cond");
                let end_bb  = self.context.append_basic_block(function, "do_end");

                if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                    self.builder.build_unconditional_branch(body_bb).unwrap();
                }

                // Body
                self.builder.position_at_end(body_bb);
                self.loop_stack.push((end_bb, cond_bb, cond_bb));
                let result = self.compile_statement(body);
                self.loop_stack.pop();
                result?;

                if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                    self.builder.build_unconditional_branch(cond_bb).unwrap();
                }

                // Condition
                self.builder.position_at_end(cond_bb);

                let cond_val = self.compile_expression(condition)?;
                let cond_bool = match cond_val {
                    BasicValueEnum::IntValue(i) if i.get_type() == self.context.bool_type() => i,
                    _ => return Err("Do-while condition must be boolean".into()),
                };

                self.builder
                    .build_conditional_branch(cond_bool, body_bb, end_bb)
                    .unwrap();

                // End
                self.builder.position_at_end(end_bb);

                Ok(None)
            },

            STree::BREAK => {
                self.logger.info("compile_break()");

                let (break_target, ..) = self
                    .loop_stack
                    .last()
                    .ok_or("break used outside of loop")?;

                self.builder
                    .build_unconditional_branch(*break_target)
                    .unwrap();

                Ok(None)
            },

            STree::CONTINUE => {
                self.logger.info("compile_continue()");

                let (_, continue_target, _) = self
                    .loop_stack
                    .last()
                    .ok_or("continue used outside of loop")?;

                self.builder
                    .build_unconditional_branch(*continue_target)
                    .unwrap();

                Ok(None)
            },

            STree::REPEAT => {
                self.logger.info("compile_repeat()");

                let (.., repeat_target) = self
                    .loop_stack
                    .last()
                    .ok_or("repeat used outside of loop")?;

                self.builder
                    .build_unconditional_branch(*repeat_target)
                    .unwrap();

                Ok(None)
            },

            STree::FUNCTION_CALL { callee, args } => {
                self.compile_function_call(callee, args)?; // ignore result
                Ok(None)
            },

            STree::BLOCK { statements } => {
                self.logger.info("compile_block()");
                self.logger.indent_inc();
                let mut last = None;
                for s in statements {
                    last = self.compile_statement(s)?;
                }
                self.logger.indent_dec();
                Ok(last)
            },

            STree::VAR_TYPE { .. } => Ok(None),
            STree::BLANK => Ok(None),
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