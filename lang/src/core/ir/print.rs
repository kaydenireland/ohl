use inkwell::values::{BasicValueEnum, FunctionValue, IntValue};

use crate::core::ir::codegen::CodeGen;

impl<'ctx> CodeGen<'ctx> {

    pub fn declare_printf(&self) -> FunctionValue<'ctx> {
        // Must use C's printf
        let i8_ptr = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
        let printf_type = self.context.i32_type().fn_type(&[i8_ptr.into()], true); // true = variadic
        self.module.add_function("printf", printf_type, None)
    }

    pub fn build_print(&mut self, values: &[BasicValueEnum<'ctx>]) -> Result<(), String> {
        self.logger.info("build_print()");
        self.logger.indent_inc();

        let printf = self.module
            .get_function("printf")
            .ok_or("printf not declared")?;

        for val in values {
            // Choose format string based on type
            let fmt_str = match val {
                BasicValueEnum::IntValue(i)   => {
                    if i.get_type() == self.context.bool_type() {
                        self.build_bool_print(*i)?;
                        continue;
                    }

                    if i.get_type() == self.context.i16_type() {
                        // char
                        "%c\n"
                    } else {
                        "%d\n"
                    }
                },
                BasicValueEnum::FloatValue(_) => "%f\n",
                BasicValueEnum::PointerValue(_) => "%s\n",

                _ => return Err("Unsupported print type".into()),
            };

            let fmt_global = self.builder.build_global_string_ptr(fmt_str, "fmt").unwrap();
            let fmt_ptr = fmt_global.as_pointer_value();

            // printf needs floats promoted to f64
            let print_val: BasicValueEnum = match val {
                BasicValueEnum::FloatValue(f) => {
                    self.builder
                        .build_float_ext(*f, self.context.f64_type(), "fext")
                        .unwrap()
                        .into()
                },
                BasicValueEnum::IntValue(i) => {
                    if i.get_type() == self.context.i16_type() {
                        // sign-extend char to i32
                        self.builder
                            .build_int_s_extend(*i, self.context.i32_type(), "char_ext")
                            .unwrap()
                            .into()
                    } else {
                        (*i).into()
                    }
                },
                other => *other,
            };

            self.builder.build_call(
                printf,
                &[fmt_ptr.into(), print_val.into()],
                "printf_call"
            ).unwrap();
        }

        self.logger.indent_dec();
        Ok(())
    }

    fn build_bool_print(&mut self, val: IntValue<'ctx>) -> Result<(), String> {
        let printf = self.module.get_function("printf").ok_or("printf not declared")?;

        let true_str  = self.builder.build_global_string_ptr("true\n",  "true_str").unwrap();
        let false_str = self.builder.build_global_string_ptr("false\n", "false_str").unwrap();

        let func = self.current_fn.unwrap();
        let then_bb  = self.context.append_basic_block(func, "bool_true");
        let else_bb  = self.context.append_basic_block(func, "bool_false");
        let merge_bb = self.context.append_basic_block(func, "bool_merge");

        // branch on the bool value
        self.builder.build_conditional_branch(val, then_bb, else_bb).unwrap();

        // true branch
        self.builder.position_at_end(then_bb);
        self.builder.build_call(printf, &[true_str.as_pointer_value().into()], "print_true").unwrap();
        self.builder.build_unconditional_branch(merge_bb).unwrap();

        // false branch
        self.builder.position_at_end(else_bb);
        self.builder.build_call(printf, &[false_str.as_pointer_value().into()], "print_false").unwrap();
        self.builder.build_unconditional_branch(merge_bb).unwrap();

        // continue after
        self.builder.position_at_end(merge_bb);
        Ok(())
    }

}