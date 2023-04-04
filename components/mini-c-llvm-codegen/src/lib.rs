use std::collections::HashMap;

pub use inkwell::context::Context as LlvmContext;
use inkwell::{builder::Builder, module::Module, types::BasicMetadataTypeEnum};
use mini_c_ast::{
    Declaration, FunctionDeclaration, Ident, Parameter, Program, Type, VariableDeclaration,
};

#[derive(Debug)]
pub enum CompileTimeError {
    RedefinedFunction(Ident),
    FunctionParameterIsVoid {
        function_ident: Ident,
        parameter_ident: Ident,
    },
}

type Result<T, E = CompileTimeError> = std::result::Result<T, E>;

pub struct Compiler<'ctx> {
    context: &'ctx LlvmContext,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    type_system: TypeSystem,
}

impl<'a> Compiler<'a> {
    pub fn new(context: &'a LlvmContext) -> Self {
        Self {
            context: context,
            builder: context.create_builder(),
            module: context.create_module("main_module"),
            type_system: TypeSystem::new(),
        }
    }

    pub fn compile(&mut self, root: &Program) -> Result<(), CompileTimeError> {
        for decl in &root.declarations {
            self.compile_declaration(decl)?;
        }
        Ok(())
    }

    fn compile_declaration(&mut self, declaration: &Declaration) -> Result<(), CompileTimeError> {
        match declaration {
            Declaration::Variable(variable) => self.compile_variable(variable),
            Declaration::Function(function) => self.compile_function(function),
        }
    }

    fn compile_variable(&mut self, variable: &VariableDeclaration) -> Result<(), CompileTimeError> {
        Ok(())
    }

    fn compile_function(&mut self, function: &FunctionDeclaration) -> Result<(), CompileTimeError> {
        self.type_system.declare_function(function)?;

        let FunctionDeclaration(return_type, function_ident, parameters, scope) = function;

        let function = {
            let parameter_types = &parameters
                .iter()
                .map(|Parameter(param_type, param_ident)| {
                    match param_type {
                        Type::Void => {
                            Err(CompileTimeError::FunctionParameterIsVoid {
                                function_ident: function_ident.clone(),
                                parameter_ident: param_ident.clone(),
                            })
                        }
                        Type::Int => Ok(BasicMetadataTypeEnum::IntType(self.context.bool_type())),
                        Type::Bool => Ok(BasicMetadataTypeEnum::IntType(self.context.i64_type())),
                    }
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

        Ok(())
    }
}

struct TypeSystem {
    scopes: Vec<TypeSystemScope>,
    functions: HashMap<Ident, FunctionDeclaration>,
}

struct TypeSystemScope {
    variables: Vec<()>,
}

impl TypeSystem {
    fn new() -> Self {
        Self {
            scopes: vec![],
            functions: HashMap::default(),
        }
    }

    fn declare_function(&mut self, function: &FunctionDeclaration) -> Result<()> {
        let ident = function.ident().clone();

        match self.functions.insert(ident.clone(), function.clone()) {
            Some(previous_declaration) => Err(CompileTimeError::RedefinedFunction(ident)),
            None => Ok(()),
        }
    }
}
