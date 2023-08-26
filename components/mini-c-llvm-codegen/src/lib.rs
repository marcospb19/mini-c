mod builtins;

use std::{collections::HashMap, time::Instant};

use either::Either;
use indent::indent_by;
pub use inkwell::context::Context as LlvmContext;
use inkwell::{
    builder::Builder,
    memory_buffer::MemoryBuffer,
    module::Module,
    types::BasicMetadataTypeEnum,
    values::{
        BasicMetadataValueEnum, BasicValue, BasicValueEnum, InstructionValue, IntValue,
        PointerValue,
    },
    IntPredicate, OptimizationLevel,
};
use mini_c_ast::{
    AssignmentExpression, BinaryExpression, BinaryOperator, Declaration, Expression,
    FunctionCallExpression, FunctionDeclaration, Ident, IfStatement, Parameter, Program,
    ReturnStatement, Scope, Statement, Type, Value, VariableDeclaration,
    VariableReferenceExpression, WhileStatement,
};

#[derive(Debug)]
pub enum CompileTimeError {
    CallingUndefinedFunction(Ident),
    AssigningToUndefinedVariable(Ident),
    VariableAssignmentWithMismatchingType {
        ident: Ident,
        expected_type: Type,
        got_type: Type,
    },
    WhileConditionIsNotBool,
    FunctionArgumentPassedWithMismatchingType {
        ident: Ident,
        argument_index: usize,
        expected_type: Type,
        got_type: Type,
    },
    RedefinedFunction {
        previous: FunctionDeclaration,
        new: FunctionDeclaration,
    },
    RedefinedVariable {
        previous: VariableDeclaration,
        new: VariableDeclaration,
    },
    FunctionParameterIsVoid {
        function_ident: Ident,
        parameter_ident: Ident,
    },
    VariableIsVoid(Ident),
    MissingReturnInNonMainNonVoidFunction(Ident),
    PassingVoidAsFunctionArgument {
        function_ident: Ident,
        index: usize,
    },
}

type Result<T, E = CompileTimeError> = std::result::Result<T, E>;

#[must_use]
pub struct Compiler {
    // Needed LLVM interface
    context: &'static LlvmContext,
    module: Module<'static>,
    builder: Builder<'static>,
    // Our storage for LLVM stuff
    vars: HashMap<Ident, PointerValue<'static>>,
    function_block_counter: usize,
    // Our type system
    type_system: TypeSystem,
}

impl Compiler {
    pub fn new() -> Self {
        let context = LlvmContext::create();
        let context = Box::leak(context.into());

        let ir = builtins::BUILTINS_IR;
        let ir = MemoryBuffer::create_from_memory_range_copy(ir.as_bytes(), "main_module_filename");

        let module = context
            .create_module_from_ir(ir)
            .expect("Failed to load builtins from LLVM IR");

        Self {
            context,
            builder: context.create_builder(),
            module,
            vars: HashMap::new(),
            function_block_counter: 0,
            type_system: TypeSystem::new(),
        }
    }

    pub fn compile(&mut self, root: &Program) -> Result<()> {
        // Register the `print` function into the type system
        self.type_system
            .declare_builtin_function(
                Type::Void,
                "print".into(),
                vec![Parameter(Type::Int, "input".into())],
            )
            .unwrap();
        self.type_system
            .declare_builtin_function(
                Type::Void,
                "println".into(),
                vec![Parameter(Type::Int, "input".into())],
            )
            .unwrap();

        for decl in &root.declarations {
            self.compile_declaration(decl)?;
        }

        Ok(())
    }

    pub fn run(&self) {
        println!("--------- Generated LLVM IR: ---------");

        println!("{}", self.module.print_to_string().to_string());
        self.module.print_to_file("llvm-ir.ll").unwrap();

        println!("-------------------------------------------");

        if let Err(msg) = self.module.verify() {
            eprintln!("LLVM IR check failed: {}", indent_by(8, msg.to_string()))
        }

        let engine = self
            .module
            .create_jit_execution_engine(OptimizationLevel::Aggressive)
            .unwrap();
        let main_function = unsafe {
            engine
                .get_function::<unsafe extern "C" fn() -> i64>("main")
                .expect("Error loading main function")
        };

        let instant = Instant::now();

        unsafe {
            println!("{}", main_function.call());
        }

        println!("elapsed = {:?}", instant.elapsed());
    }

    fn compile_declaration(&mut self, declaration: &Declaration) -> Result<()> {
        match declaration {
            Declaration::Variable(decls) => {
                decls.iter().try_for_each(|decl| {
                    self.compile_variable_declaration(decl.clone()).map(|_| ())
                })
            }
            Declaration::Function(function) => self.compile_function(function),
        }
    }

    fn compile_function(&mut self, function: &FunctionDeclaration) -> Result<()> {
        self.function_block_counter = 0;
        self.type_system.declare_function(function)?;
        self.type_system.scopes.push_scope();

        let FunctionDeclaration(return_type, function_ident, parameters, scope) = function;

        let function = {
            let parameter_types = &parameters
                .iter()
                .map(|Parameter(param_type, param_ident)| {
                    let typ = match param_type {
                        Type::Void => {
                            return Err(CompileTimeError::FunctionParameterIsVoid {
                                function_ident: function_ident.clone(),
                                parameter_ident: param_ident.clone(),
                            });
                        }
                        Type::Int => self.context.i64_type(),
                        Type::Bool => self.context.bool_type(),
                    };
                    Ok(BasicMetadataTypeEnum::IntType(typ))
                })
                .collect::<Result<Vec<_>>>()?;

            let function_type = match return_type {
                Type::Void => self.context.void_type().fn_type(parameter_types, false),
                Type::Int => self.context.i64_type().fn_type(parameter_types, false),
                Type::Bool => self.context.bool_type().fn_type(parameter_types, false),
            };

            self.module
                .add_function(function_ident, function_type, None)
        };

        let function_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(function_block);

        let llvm_function_param_iter = function.get_param_iter();
        for (Parameter(typ, ident), llvm_param) in
            parameters.iter().cloned().zip(llvm_function_param_iter)
        {
            let variable_declaration = VariableDeclaration {
                typ,
                ident,
                array_len: None,
            };

            let pointer = self.compile_variable_declaration(variable_declaration)?;

            self.builder.build_store(pointer, llvm_param);
        }
        self.compile_scope(scope)?;

        let Scope(_, statements) = scope;

        let is_last_statement_return = matches!(statements.last(), Some(Statement::Return(..)));
        if !is_last_statement_return {
            let is_return_type_void = matches!(return_type, Type::Void);

            match (is_return_type_void, function_ident.as_str()) {
                (true, _) => drop(self.builder.build_return(None)),
                (false, "main") => {
                    self.builder
                        .build_return(Some(&self.context.i64_type().const_int(0, false)));
                }
                (false, _) => {
                    return Err(CompileTimeError::MissingReturnInNonMainNonVoidFunction(
                        function_ident.to_string(),
                    ));
                }
            }
        }

        self.type_system.scopes.pop_scope();

        Ok(())
    }

    fn compile_return(&mut self, statement: &ReturnStatement) -> Result<()> {
        let ReturnStatement(expression) = statement;

        let expr = expression
            .as_ref()
            .map(|expr| self.compile_expression(expr).map(LlvmExpr::to_value))
            .transpose()?
            .flatten();

        let expr = expr.as_ref().map(|x| x as &dyn BasicValue);

        self.builder.build_return(expr);
        Ok(())
    }

    fn compile_scope(&mut self, scope: &Scope) -> Result<()> {
        let Scope(variable_declarations, statements) = scope;
        self.type_system.scopes.push_scope();

        for decl in variable_declarations {
            self.compile_variable_declaration(decl.clone())?;
        }
        for statement in statements {
            self.compile_statement(statement)?;
        }

        self.type_system.scopes.pop_scope();
        Ok(())
    }

    fn compile_variable_declaration(
        &mut self,
        declaration: VariableDeclaration,
    ) -> Result<PointerValue<'static>> {
        if declaration.array_len.is_some() {
            unimplemented!("We didn't implemented arrays compilation and handling yet");
        }

        let alloca_pointer = match declaration.typ {
            Type::Void => return Err(CompileTimeError::VariableIsVoid(declaration.ident.clone())),
            Type::Int => {
                self.builder
                    .build_alloca(self.context.i64_type(), &declaration.ident)
            }
            Type::Bool => {
                self.builder
                    .build_alloca(self.context.bool_type(), &declaration.ident)
            }
        };

        self.vars.insert(declaration.ident.clone(), alloca_pointer);
        self.type_system.scopes.try_declare_variable(declaration)?;

        Ok(alloca_pointer)
    }

    fn compile_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::If(statement) => self.compile_if(statement)?,
            Statement::For(_statement) => todo!(),
            Statement::While(statement) => self.compile_while(statement)?,
            Statement::Break => todo!(),
            Statement::Return(statement) => self.compile_return(statement)?,
            Statement::Scope(scope) => self.compile_scope(scope)?,
            Statement::Expression(expression) => {
                self.compile_expression(expression)?;
            }
        }

        Ok(())
    }

    fn compile_if(&mut self, statement: &IfStatement) -> Result<()> {
        let IfStatement(condition, then_scope, else_scope) = statement;

        let LlvmExpr::Bool(condition) = self.compile_expression(condition)? else {
            return Err(CompileTimeError::WhileConditionIsNotBool);
        };

        let last_function = self.module.get_last_function().unwrap();
        let [if_then_block, if_end_block] = ["if_then", "if_end"].map(|name| {
            self.context
                .append_basic_block(last_function, self.new_block_name(name).as_str())
        });

        match else_scope {
            None => {
                self.builder
                    .build_conditional_branch(condition, if_then_block, if_end_block);
            }
            Some(else_scope) => {
                let if_else_block = self
                    .context
                    .append_basic_block(last_function, self.new_block_name("if_else").as_str());

                self.builder
                    .build_conditional_branch(condition, if_then_block, if_else_block);

                self.builder.position_at_end(if_else_block);
                self.compile_scope(else_scope)?;
                self.builder.build_unconditional_branch(if_end_block);
            }
        }

        self.builder.position_at_end(if_then_block);
        self.compile_scope(then_scope)?;
        if if_then_block.get_terminator().is_none() {
            self.builder.build_unconditional_branch(if_end_block);
        }

        self.builder.position_at_end(if_end_block);

        Ok(())
    }

    fn compile_while(&mut self, statement: &WhileStatement) -> Result<()> {
        let WhileStatement(condition, scope) = &statement;

        let last_function = self.module.get_last_function().unwrap();
        let [while_condition_block, while_scope_block, while_end_block] =
            ["while_condition", "while_scope", "while_end"].map(|name| {
                self.context
                    .append_basic_block(last_function, self.new_block_name(name).as_str())
            });

        // Build while entrance
        self.builder
            .build_unconditional_branch(while_condition_block);

        // Build while condition
        self.builder.position_at_end(while_condition_block);

        let condition = condition
            .as_ref()
            .unwrap_or(&Expression::Value(Value::Bool(true)));

        let LlvmExpr::Bool(condition) = self.compile_expression(condition)? else {
            return Err(CompileTimeError::WhileConditionIsNotBool);
        };

        self.builder
            .build_conditional_branch(condition, while_scope_block, while_end_block);

        // Build while scope
        self.builder.position_at_end(while_scope_block);

        self.compile_scope(scope)?;

        self.builder
            .build_unconditional_branch(while_condition_block);
        self.builder.position_at_end(while_end_block);

        Ok(())
    }

    fn compile_expression(&mut self, expression: &Expression) -> Result<LlvmExpr> {
        let value = match expression {
            Expression::Value(value) => self.compile_expression_value(value)?,
            Expression::Assignment(assignment) => self.compile_expression_assignment(assignment)?,
            Expression::Binary(expression) => self.compile_expression_binary(expression)?,
            Expression::Unary(_ /*Box<UnaryExpression>*/) => todo!(),
            Expression::FunctionCall(call) => self.compile_expression_function_call(call)?,
            Expression::VariableReference(variable_reference) => {
                self.compile_expression_variable_reference(variable_reference)?
            }
        };

        Ok(value)
    }

    fn compile_expression_value(&mut self, value: &Value) -> Result<LlvmExpr> {
        Ok(match value {
            Value::Int(int) => LlvmExpr::Int(self.context.i64_type().const_int(*int as u64, false)),
            Value::Bool(boo) => {
                LlvmExpr::Bool(self.context.bool_type().const_int(*boo as u64, false))
            }
        })
    }

    fn compile_expression_assignment(
        &mut self,
        assignment: &AssignmentExpression,
    ) -> Result<LlvmExpr> {
        let AssignmentExpression(variable_reference, expression) = assignment;

        let value = self.compile_expression(expression)?;

        let var_ident = match variable_reference {
            VariableReferenceExpression::Normal(ident) => ident,
            _ => todo!("Arrays are not supportedk"),
        };

        let got_type = value.to_type();
        let VariableDeclaration {
            typ: expected_type, ..
        } = self
            .type_system
            .scopes
            .get_variable(var_ident)
            .ok_or_else(|| CompileTimeError::AssigningToUndefinedVariable(var_ident.to_string()))?;

        if got_type != *expected_type {
            return Err(CompileTimeError::VariableAssignmentWithMismatchingType {
                ident: var_ident.to_string(),
                expected_type: *expected_type,
                got_type,
            });
        }

        // Index safety: checked above that the variable exists in the typesystem
        let var_pointer = self.vars[var_ident];

        // Unwrap safety: we checked that `value` is not void by comparing it's type to a
        // variable type (and variables can't be void)
        self.builder
            .build_store(var_pointer, value.to_value().unwrap());
        Ok(value)
    }

    fn compile_expression_binary(&mut self, expression: &BinaryExpression) -> Result<LlvmExpr> {
        let BinaryExpression(lhs, operator, rhs) = expression;

        let lhs = self.compile_expression(lhs)?;
        let rhs = self.compile_expression(rhs)?;

        if lhs.to_type() != rhs.to_type() {
            todo!("types mismatch, return error here");
        }

        let lhs = lhs.to_value().unwrap();
        let rhs = rhs.to_value().unwrap();

        let operation_value = match operator {
            BinaryOperator::Add => self.builder.build_int_add(lhs, rhs, ""),
            BinaryOperator::Sub => self.builder.build_int_sub(lhs, rhs, ""),
            BinaryOperator::Mul => self.builder.build_int_mul(lhs, rhs, ""),
            BinaryOperator::Div => self.builder.build_int_signed_div(lhs, rhs, ""),
            BinaryOperator::Modulo => self.builder.build_int_signed_rem(lhs, rhs, ""),
            // And => self.builder.build_int_xxxxx(lhs, rhs, ""),
            // Or => self.builder.build_int_xxxxx(lhs, rhs, ""),
            BinaryOperator::Equals => {
                self.builder
                    .build_int_compare(IntPredicate::EQ, lhs, rhs, "")
            }
            BinaryOperator::NotEquals => {
                self.builder
                    .build_int_compare(IntPredicate::NE, lhs, rhs, "")
            }
            BinaryOperator::Greater => {
                self.builder
                    .build_int_compare(IntPredicate::SGT, lhs, rhs, "")
            }
            BinaryOperator::GreaterOrEquals => {
                self.builder
                    .build_int_compare(IntPredicate::SGE, lhs, rhs, "")
            }
            BinaryOperator::Less => {
                self.builder
                    .build_int_compare(IntPredicate::SLT, lhs, rhs, "")
            }
            BinaryOperator::LessOrEquals => {
                self.builder
                    .build_int_compare(IntPredicate::SLE, lhs, rhs, "")
            }
            op => todo!("Implement binary operation for {op:?}"),
        };

        Ok(LlvmExpr::from(operation_value))
    }

    fn compile_expression_function_call(
        &mut self,
        call: &FunctionCallExpression,
    ) -> Result<LlvmExpr> {
        let FunctionCallExpression(function_ident, arguments) = call;

        let Some(function) = self.module.get_function(function_ident) else {
            return Err(CompileTimeError::CallingUndefinedFunction(
                function_ident.to_string(),
            ));
        };

        let arguments = arguments
            .iter()
            .enumerate()
            .map(|(index, argument)| {
                self.compile_expression(argument).and_then(|value| {
                    value.to_metadata_value_enum().ok_or_else(|| {
                        CompileTimeError::PassingVoidAsFunctionArgument {
                            function_ident: function_ident.to_string(),
                            index,
                        }
                    })
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let FunctionDeclaration(_, _, parameters, _) = &self
            .type_system
            .functions
            .get(function_ident)
            .expect(&format!(
                "Undefined function {function_ident}, is it builtin?"
            ));

        parameters
            .iter()
            .map(|Parameter(param_type, _)| param_type)
            .zip(&arguments)
            .enumerate()
            .try_for_each(|(argument_index, (param_type, argument_type))| {
                let argument_type = LlvmExpr::from(*argument_type).to_type();

                (*param_type == argument_type).then_some(()).ok_or_else(|| {
                    CompileTimeError::FunctionArgumentPassedWithMismatchingType {
                        ident: function_ident.to_string(),
                        argument_index,
                        expected_type: *param_type,
                        got_type: argument_type,
                    }
                })
            })?;

        let call_site = self
            .builder
            .build_call(function, arguments.as_slice(), "")
            .try_as_basic_value();

        Ok(LlvmExpr::from(call_site))
    }

    fn compile_expression_variable_reference(
        &mut self,
        variable_reference: &VariableReferenceExpression,
    ) -> Result<LlvmExpr> {
        let ident = match variable_reference {
            VariableReferenceExpression::Normal(ident) => ident,
            _ => todo!("Arrays are not supported"),
        };

        let var_pointer = self.vars[ident];
        let pointee_type = self.context.i64_type();

        Ok(self
            .builder
            .build_load(pointee_type, var_pointer, ident)
            .into())
    }

    fn new_block_name(&mut self, name: &str) -> String {
        self.function_block_counter += 1;
        format!("block_{}_{}", name, self.function_block_counter)
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

struct TypeSystem {
    scopes: TypeSystemScopes,
    functions: HashMap<Ident, FunctionDeclaration>,
}

impl TypeSystem {
    fn new() -> Self {
        Self {
            scopes: TypeSystemScopes::new(),
            functions: HashMap::default(),
        }
    }

    fn declare_function(&mut self, declaration: &FunctionDeclaration) -> Result<()> {
        let ident = declaration.ident().clone();

        match self.functions.insert(ident.clone(), declaration.clone()) {
            Some(previous_declaration) => {
                Err(CompileTimeError::RedefinedFunction {
                    previous: previous_declaration,
                    new: declaration.clone(),
                })
            }
            None => Ok(()),
        }
    }

    fn declare_builtin_function(
        &mut self,
        typ: Type,
        ident: Ident,
        parameters: Vec<Parameter>,
    ) -> Result<()> {
        let dummy_scope = Scope(vec![], vec![]);

        let declaration = FunctionDeclaration(typ, ident, parameters, dummy_scope);

        self.declare_function(&declaration)
    }
}

struct TypeSystemScopes {
    scopes_of_variables: Vec<Vec<VariableDeclaration>>,
}

impl TypeSystemScopes {
    fn new() -> Self {
        Self {
            scopes_of_variables: vec![],
        }
    }

    fn push_scope(&mut self) {
        self.scopes_of_variables.push(vec![])
    }

    fn pop_scope(&mut self) {
        self.scopes_of_variables.pop();
    }

    fn get_variable(&self, ident: &Ident) -> Option<&VariableDeclaration> {
        self.scopes_of_variables
            .iter()
            .rev()
            .flat_map(|scope| scope.iter())
            .find(|var| &var.ident == ident)
    }

    fn try_declare_variable(&mut self, declaration: VariableDeclaration) -> Result<()> {
        match self.get_variable(&declaration.ident) {
            Some(previous_declaration) => {
                Err(CompileTimeError::RedefinedVariable {
                    previous: previous_declaration.clone(),
                    new: declaration,
                })
            }
            None => {
                self.scopes_of_variables
                    .last_mut()
                    .unwrap()
                    .push(declaration);
                Ok(())
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum LlvmExpr {
    Void,
    Int(IntValue<'static>),
    Bool(IntValue<'static>),
}

impl LlvmExpr {
    fn to_type(self) -> Type {
        match self {
            Self::Void => Type::Void,
            Self::Int(_) => Type::Int,
            Self::Bool(_) => Type::Bool,
        }
    }

    fn to_value(self) -> Option<IntValue<'static>> {
        match self {
            Self::Void => None,
            Self::Int(value) | Self::Bool(value) => Some(value),
        }
    }

    fn to_metadata_value_enum(self) -> Option<BasicMetadataValueEnum<'static>> {
        match self {
            Self::Void => None,
            Self::Int(value) | Self::Bool(value) => Some(value.into()),
        }
    }
}

type TryAsBasicValueReturnType<'a> = Either<BasicValueEnum<'a>, InstructionValue<'a>>;
impl From<TryAsBasicValueReturnType<'static>> for LlvmExpr {
    fn from(value: TryAsBasicValueReturnType<'static>) -> Self {
        match value {
            Either::Left(value) => Self::from(value.into_int_value()),
            Either::Right(_) => LlvmExpr::Void,
        }
    }
}

impl From<BasicMetadataValueEnum<'static>> for LlvmExpr {
    fn from(value: BasicMetadataValueEnum<'static>) -> Self {
        value.into_int_value().into()
    }
}

impl From<BasicValueEnum<'static>> for LlvmExpr {
    fn from(value: BasicValueEnum<'static>) -> Self {
        value.into_int_value().into()
    }
}

impl From<IntValue<'static>> for LlvmExpr {
    fn from(value: IntValue<'static>) -> Self {
        if value.get_type().get_bit_width() == 1 {
            Self::Bool(value)
        } else {
            Self::Int(value)
        }
    }
}
