pub const BUILTINS_IR: &str = include_str!("builtins.ll");

// The code that originally generated the IR above, before I edited it.
/*
fn compile_builtins(&mut self) {
    // Generation code
    let int_type = self.context.i64_type();
    let i32_type = self.context.i32_type();
    let char_type = self.context.i8_type();

    // Add putchar

    let putchar_type = i32_type.fn_type(&[char_type.into()], false);

    self.module
        .add_function("putchar", putchar_type, Linkage::External.into());

    // Add print

    self.type_system
        .declare_builtin_function(
            Type::Void,
            "print".into(),
            vec![Parameter(Type::Int, "input".into())],
        )
        .unwrap();

    let params_type = &[BasicMetadataTypeEnum::IntType(self.context.i64_type())];
    let fn_type = self.context.void_type().fn_type(params_type, false);

    // ----- START IMPLEMENTATION

    let print_function = self.module.add_function("print", fn_type, None);
    let print_function_block = self.context.append_basic_block(print_function, "entry");
    self.builder.position_at_end(print_function_block);

    let old_input_argument = print_function.get_first_param().unwrap();
    let input_argument = self.builder.build_alloca(int_type, "p_input");
    self.builder.build_store(input_argument, old_input_argument);

    let [while_condition_block, while_scope_block, while_end_block] =
        ["while_condition", "while_scope", "while_end"].map(|name| {
            self.context
                .append_basic_block(print_function, self.new_block_name(name).as_str())
        });

    // Build while entrance
    self.builder
        .build_unconditional_branch(while_condition_block);

    // Build while condition
    self.builder.position_at_end(while_condition_block);

    let n = self
        .builder
        .build_load(int_type, input_argument, "")
        .into_int_value();

    let condition =
        self.builder
            .build_int_compare(IntPredicate::NE, n, int_type.const_int(0, false), "");

    self.builder
        .build_conditional_branch(condition, while_scope_block, while_end_block);

    // Build while scope
    self.builder.position_at_end(while_scope_block);

    let remainder =
        self.builder
            .build_int_signed_rem(n, int_type.const_int(10, false), "remainder");

    let remainder_i8 = self.builder.build_cast(
        InstructionOpcode::Trunc,
        remainder,
        char_type,
        "remainder_char",
    );

    let remainder_i8_ascii = self.builder.build_int_add(
        remainder_i8.into_int_value(),
        char_type.const_int(b'0'.into(), false),
        "remainder_i8_ascii",
    );

    let putchar_fn = self.module.get_function("putchar").unwrap();

    self.builder
        .build_call(putchar_fn, &[remainder_i8_ascii.into()], "");

    let new_n_value =
        self.builder
            .build_int_signed_div(n, int_type.const_int(10, false), "new_input_value");
    self.builder.build_store(input_argument, new_n_value);

    self.builder
        .build_unconditional_branch(while_condition_block);

    self.builder.position_at_end(while_end_block);
    self.builder.build_call(
        putchar_fn,
        &[char_type.const_int(b'\n'.into(), false).into()],
        "",
    );

    self.builder.build_return(None);
}
*/
