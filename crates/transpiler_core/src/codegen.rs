use crate::ast::*;

fn to_snake_case(name: &str) -> String {
    let mut result = String::new();
    let mut prev_is_lower = false;
    let mut chars = name.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch.is_uppercase() {
            if prev_is_lower || (result.len() > 0 && chars.peek().map_or(false, |c| c.is_lowercase())) {
                if result.len() > 0 {
                    result.push('_');
                }
            }
            result.push(ch.to_lowercase().next().unwrap());
            prev_is_lower = false;
        } else {
            result.push(ch);
            prev_is_lower = ch.is_lowercase();
        }
    }
    
    result
}

fn convert_name(name: &str) -> String {
    if name.chars().all(|c| c.is_uppercase() || c == '_' || c.is_numeric()) {
        name.to_uppercase()
    } else {
        to_snake_case(name)
    }
}

pub struct Codegen {
    output: String,
    indent_level: usize,
    is_main_file: bool,
}

impl Codegen {
    pub fn new() -> Self {
        Codegen {
            output: String::new(),
            indent_level: 0,
            is_main_file: true,
        }
    }
    
    pub fn new_module() -> Self {
        Codegen {
            output: String::new(),
            indent_level: 0,
            is_main_file: false,
        }
    }

    pub fn generate(&mut self, program: &Program) -> String {
        // Check if there's a main function defined
        let has_main = program.statements.iter().any(|stmt| {
            matches!(stmt, Statement::FunctionDecl(func_decl) if func_decl.name == "main")
        });
        
        // Only add main wrapper if this is the main file and no main is defined
        if self.is_main_file && !has_main {
            self.emit_header();
        }
        
        for statement in &program.statements {
            self.generate_statement(statement);
        }
        
        if self.is_main_file && !has_main {
            self.emit_main_if_needed();
        }
        
        self.output.clone()
    }

    fn generate_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::ImportStmt(import_stmt) => self.generate_import_stmt(import_stmt),
            Statement::ExportStmt(inner) => self.generate_export_stmt(inner),
            Statement::VariableDecl(var_decl) => self.generate_variable_decl(var_decl),
            Statement::FunctionDecl(func_decl) => self.generate_function_decl(func_decl),
            Statement::StructDecl(struct_decl) => self.generate_struct_decl(struct_decl),
            Statement::EnumDecl(enum_decl) => self.generate_enum_decl(enum_decl),
            Statement::PrintStmt(print_stmt) => self.generate_print_stmt(print_stmt),
            Statement::ReturnStmt(ret_stmt) => self.generate_return_stmt(ret_stmt),
            Statement::IfElse(if_else) => self.generate_if_else(if_else),
            Statement::ForLoop(for_loop) => self.generate_for_loop(for_loop),
            Statement::WhileLoop(while_loop) => self.generate_while_loop(while_loop),
            Statement::TryCatch(try_catch) => self.generate_try_catch(try_catch),
            Statement::ThrowStmt(throw_stmt) => self.generate_throw_stmt(throw_stmt),
            Statement::BreakStmt => {
                self.emit_indent();
                self.output.push_str("break;\n");
            }
            Statement::ContinueStmt => {
                self.emit_indent();
                self.output.push_str("continue;\n");
            }
            Statement::ExpressionStmt(expr) => {
                self.emit_indent();
                self.generate_expression(expr);
                self.output.push_str(";\n");
            }
        }
    }

    fn generate_import_stmt(&mut self, import_stmt: &ImportStmt) {
        self.output.push_str("use ");
        
        if import_stmt.is_external {
            let path = import_stmt.path.replace("::", "::");
            
            if import_stmt.imports.len() == 1 && import_stmt.imports[0].alias.is_none() {
                self.output.push_str(&path);
                self.output.push_str("::");
                self.output.push_str(&convert_name(&import_stmt.imports[0].name));
            } else if import_stmt.imports.len() == 1 {
                self.output.push_str(&path);
                self.output.push_str("::");
                self.output.push_str(&convert_name(&import_stmt.imports[0].name));
                if let Some(ref alias) = import_stmt.imports[0].alias {
                    self.output.push_str(" as ");
                    self.output.push_str(&convert_name(alias));
                }
            } else {
                self.output.push_str(&path);
                self.output.push_str("::{");
                for (i, item) in import_stmt.imports.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.output.push_str(&convert_name(&item.name));
                    if let Some(ref alias) = item.alias {
                        self.output.push_str(" as ");
                        self.output.push_str(&convert_name(alias));
                    }
                }
                self.output.push('}');
            }
        } else {
            let path = if import_stmt.path.starts_with("./") {
                let module_path = import_stmt.path.trim_start_matches("./").replace("/", "::");
                if self.is_main_file {
                    module_path
                } else {
                    format!("super::{}", module_path)
                }
            } else {
                import_stmt.path.replace("/", "::")
            };
            
            if import_stmt.imports.len() == 1 {
                self.output.push_str(&path);
                self.output.push_str("::");
                self.output.push_str(&convert_name(&import_stmt.imports[0].name));
                if let Some(ref alias) = import_stmt.imports[0].alias {
                    self.output.push_str(" as ");
                    self.output.push_str(&convert_name(alias));
                }
            } else {
                self.output.push_str(&path);
                self.output.push_str("::{");
                for (i, item) in import_stmt.imports.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.output.push_str(&convert_name(&item.name));
                    if let Some(ref alias) = item.alias {
                        self.output.push_str(" as ");
                        self.output.push_str(&convert_name(alias));
                    }
                }
                self.output.push('}');
            }
        }
        
        self.output.push_str(";\n");
    }
    
    fn generate_export_stmt(&mut self, inner: &Statement) {
        match inner {
            Statement::FunctionDecl(func_decl) => {
                self.emit_indent();
                self.output.push_str("pub ");
                self.output.push_str("fn ");
                self.output.push_str(&to_snake_case(&func_decl.name));
                self.output.push('(');
                
                for (i, param) in func_decl.parameters.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.output.push_str(&to_snake_case(&param.name));
                    self.output.push_str(": ");
                    self.emit_type(&param.param_type);
                }
                
                self.output.push(')');
                
                if func_decl.return_type != Type::Void {
                    self.output.push_str(" -> ");
                    self.emit_type(&func_decl.return_type);
                }
                
                self.output.push_str(" {\n");
                self.indent_level += 1;
                
                for stmt in &func_decl.body {
                    self.generate_statement(stmt);
                }
                
                self.indent_level -= 1;
                self.emit_indent();
                self.output.push_str("}\n\n");
            },
            Statement::StructDecl(struct_decl) => {
                self.emit_indent();
                self.output.push_str("#[derive(Debug, Clone)]\n");
                self.emit_indent();
                self.output.push_str("pub struct ");
                self.output.push_str(&struct_decl.name);
                self.output.push_str(" {\n");
                self.indent_level += 1;
                
                for field in &struct_decl.fields {
                    self.emit_indent();
                    self.output.push_str("pub ");
                    self.output.push_str(&field.name);
                    self.output.push_str(": ");
                    self.emit_type(&field.field_type);
                    self.output.push_str(",\n");
                }
                
                self.indent_level -= 1;
                self.emit_indent();
                self.output.push_str("}\n\n");
            },
            Statement::EnumDecl(enum_decl) => {
                self.emit_indent();
                self.output.push_str("#[derive(Debug, Clone, PartialEq)]\n");
                self.emit_indent();
                self.output.push_str("pub enum ");
                self.output.push_str(&enum_decl.name);
                self.output.push_str(" {\n");
                self.indent_level += 1;
                
                for variant in &enum_decl.variants {
                    self.emit_indent();
                    self.output.push_str(&variant.name);
                    if let Some(ref fields) = variant.fields {
                        if !fields.is_empty() {
                            self.output.push('(');
                            for (i, field_type) in fields.iter().enumerate() {
                                if i > 0 {
                                    self.output.push_str(", ");
                                }
                                self.emit_type(field_type);
                            }
                            self.output.push(')');
                        }
                    }
                    self.output.push_str(",\n");
                }
                
                self.indent_level -= 1;
                self.emit_indent();
                self.output.push_str("}\n\n");
            },
            Statement::VariableDecl(var_decl) => {
                self.emit_indent();
                self.output.push_str("pub ");
                
                if var_decl.is_const {
                    self.output.push_str("const ");
                    self.output.push_str(&var_decl.name.to_uppercase());
                } else {
                    self.output.push_str("static mut ");
                    self.output.push_str(&var_decl.name);
                }
                
                if let Some(var_type) = &var_decl.var_type {
                    self.output.push_str(": ");
                    if var_decl.is_const && *var_type == Type::String {
                        self.output.push_str("&str");
                    } else {
                        self.emit_type(var_type);
                    }
                }
                
                self.output.push_str(" = ");
                
                if matches!(var_decl.value, Expression::StringLiteral(_)) {
                    self.generate_expression(&var_decl.value);
                } else {
                    self.generate_expression(&var_decl.value);
                }
                
                self.output.push_str(";\n");
            },
            _ => {}
        }
    }

    fn generate_variable_decl(&mut self, var_decl: &VariableDecl) {
        self.emit_indent();
        
        if var_decl.is_const {
            self.output.push_str("const ");
            self.output.push_str(&var_decl.name.to_uppercase());
            
            self.output.push_str(": ");
            if let Some(var_type) = &var_decl.var_type {
                if *var_type == Type::String {
                    self.output.push_str("&str");
                } else {
                    self.emit_type(var_type);
                }
            } else {
                match &var_decl.value {
                    Expression::NumberLiteral(_) => self.output.push_str("i32"),
                    Expression::StringLiteral(_) => self.output.push_str("&str"),
                    Expression::BooleanLiteral(_) => self.output.push_str("bool"),
                    _ => self.output.push_str("i32"),
                }
            }
        } else {
            self.output.push_str("let mut ");
            self.output.push_str(&to_snake_case(&var_decl.name));
            
            if let Some(var_type) = &var_decl.var_type {
                self.output.push_str(": ");
                self.emit_type(var_type);
            }
        }
        
        self.output.push_str(" = ");
        
        let needs_to_string = !var_decl.is_const && if let Some(var_type) = &var_decl.var_type {
            matches!(var_decl.value, Expression::StringLiteral(_)) && 
            (*var_type == Type::String || *var_type == Type::Any)
        } else {
            matches!(var_decl.value, Expression::StringLiteral(_))
        };
        
        let is_static_array = if let Some(Type::Array { size: Some(_), .. }) = &var_decl.var_type {
            matches!(var_decl.value, Expression::ArrayLiteral(_))
        } else {
            false
        };
        
        if needs_to_string {
            self.generate_expression(&var_decl.value);
            self.output.push_str(".to_string()");
        } else if is_static_array {
            if let Expression::ArrayLiteral(elements) = &var_decl.value {
                self.output.push('[');
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.generate_expression(elem);
                }
                self.output.push(']');
            }
        } else {
            self.generate_expression(&var_decl.value);
        }
        
        self.output.push_str(";\n");
    }

    fn generate_function_decl(&mut self, func_decl: &FunctionDecl) {
        self.emit_indent();
        self.output.push_str("fn ");
        if func_decl.name == "main" {
            self.output.push_str("main");
        } else {
            self.output.push_str(&to_snake_case(&func_decl.name));
        }
        self.output.push_str("(");
        
        for (i, param) in func_decl.parameters.iter().enumerate() {
            if i > 0 {
                self.output.push_str(", ");
            }
            self.output.push_str(&to_snake_case(&param.name));
            self.output.push_str(": ");
            self.emit_type(&param.param_type);
        }
        
        self.output.push_str(") ");
        
        if func_decl.return_type != Type::Void {
            self.output.push_str("-> ");
            self.emit_type(&func_decl.return_type);
            self.output.push_str(" ");
        }
        
        self.output.push_str("{\n");
        self.indent_level += 1;
        
        for stmt in &func_decl.body {
            self.generate_statement(stmt);
        }
        
        self.indent_level -= 1;
        self.emit_indent();
        self.output.push_str("}\n\n");
    }

    fn generate_print_stmt(&mut self, print_stmt: &PrintStmt) {
        self.emit_indent();
        self.output.push_str("println!(\"{}\"");
        self.output.push_str(", ");
        self.generate_expression(&print_stmt.expression);
        self.output.push_str(");\n");
    }

    fn generate_return_stmt(&mut self, ret_stmt: &ReturnStmt) {
        self.emit_indent();
        self.output.push_str("return ");
        
        if let Some(expr) = &ret_stmt.value {
            self.generate_expression(expr);
        }
        
        self.output.push_str(";\n");
    }

    fn generate_if_else(&mut self, if_else: &IfElseStmt) {
        self.emit_indent();
        self.output.push_str("if ");
        self.generate_expression(&if_else.condition);
        self.output.push_str(" {\n");
        
        self.indent_level += 1;
        for stmt in &if_else.then_body {
            self.generate_statement(stmt);
        }
        self.indent_level -= 1;
        
        self.emit_indent();
        
        if let Some(else_body) = &if_else.else_body {
            self.output.push_str("} else {\n");
            self.indent_level += 1;
            for stmt in else_body {
                self.generate_statement(stmt);
            }
            self.indent_level -= 1;
            self.emit_indent();
            self.output.push_str("}\n");
        } else {
            self.output.push_str("}\n");
        }
    }

    fn generate_for_loop(&mut self, for_loop: &ForLoopStmt) {
        self.emit_indent();
        self.output.push_str("for ");
        self.output.push_str(&for_loop.variable);
        self.output.push_str(" in ");
        self.generate_expression(&for_loop.iterable);
        self.output.push_str(" {\n");
        
        self.indent_level += 1;
        for stmt in &for_loop.body {
            self.generate_statement(stmt);
        }
        self.indent_level -= 1;
        
        self.emit_indent();
        self.output.push_str("}\n");
    }

    fn generate_while_loop(&mut self, while_loop: &WhileLoopStmt) {
        self.emit_indent();
        self.output.push_str("while ");
        self.generate_expression(&while_loop.condition);
        self.output.push_str(" {\n");
        
        self.indent_level += 1;
        for stmt in &while_loop.body {
            self.generate_statement(stmt);
        }
        self.indent_level -= 1;
        
        self.emit_indent();
        self.output.push_str("}\n");
    }

    fn generate_struct_decl(&mut self, struct_decl: &StructDecl) {
        self.emit_indent();
        self.output.push_str("#[derive(Debug, Clone)]\n");
        self.emit_indent();
        self.output.push_str("struct ");
        self.output.push_str(&struct_decl.name);
        self.output.push_str(" {\n");
        
        self.indent_level += 1;
        for field in &struct_decl.fields {
            self.emit_indent();
            self.output.push_str(&field.name);
            self.output.push_str(": ");
            self.emit_type(&field.field_type);
            self.output.push_str(",\n");
        }
        self.indent_level -= 1;
        
        self.emit_indent();
        self.output.push_str("}\n\n");
    }

    fn generate_enum_decl(&mut self, enum_decl: &EnumDecl) {
        self.emit_indent();
        self.output.push_str("#[derive(Debug, Clone, PartialEq)]\n");
        self.emit_indent();
        self.output.push_str("enum ");
        self.output.push_str(&enum_decl.name);
        self.output.push_str(" {\n");
        
        self.indent_level += 1;
        for variant in &enum_decl.variants {
            self.emit_indent();
            self.output.push_str(&variant.name);
            
            if let Some(fields) = &variant.fields {
                self.output.push_str("(");
                for (i, field_type) in fields.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.emit_type(field_type);
                }
                self.output.push_str(")");
            }
            
            self.output.push_str(",\n");
        }
        self.indent_level -= 1;
        
        self.emit_indent();
        self.output.push_str("}\n\n");
    }

    fn generate_try_catch(&mut self, try_catch: &TryCatchStmt) {
        self.emit_indent();
        self.output.push_str("match (|| -> Result<(), Box<dyn std::error::Error>> {\n");
        
        self.indent_level += 1;
        for stmt in &try_catch.try_body {
            self.generate_statement(stmt);
        }
        self.emit_indent();
        self.output.push_str("Ok(())\n");
        self.indent_level -= 1;
        
        self.emit_indent();
        self.output.push_str("})() {\n");
        
        self.indent_level += 1;
        self.emit_indent();
        self.output.push_str("Ok(_) => {},\n");
        self.emit_indent();
        self.output.push_str("Err(");
        if let Some(param) = &try_catch.catch_param {
            self.output.push_str(param);
        } else {
            self.output.push_str("_err");
        }
        self.output.push_str(") => {\n");
        
        self.indent_level += 1;
        for stmt in &try_catch.catch_body {
            self.generate_statement(stmt);
        }
        self.indent_level -= 1;
        
        self.emit_indent();
        self.output.push_str("}\n");
        self.indent_level -= 1;
        
        self.emit_indent();
        self.output.push_str("}\n");
    }

    fn generate_throw_stmt(&mut self, throw_stmt: &ThrowStmt) {
        self.emit_indent();
        self.output.push_str("panic!(\"{}\"");
        self.output.push_str(", ");
        self.generate_expression(&throw_stmt.expression);
        self.output.push_str(");\n");
    }

    fn generate_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::NumberLiteral(n) => {
                self.output.push_str(&n.to_string());
            }
            Expression::StringLiteral(s) => {
                self.output.push('"');
                self.output.push_str(s);
                self.output.push('"');
            }
            Expression::BooleanLiteral(b) => {
                self.output.push_str(if *b { "true" } else { "false" });
            }
            Expression::Identifier(name) => {
                self.output.push_str(&convert_name(name));
            }
            Expression::ArrayLiteral(elements) => {
                self.output.push_str("vec![");
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.generate_expression(elem);
                }
                self.output.push(']');
            }
            Expression::StructLiteral { name, fields } => {
                self.output.push_str(name);
                self.output.push_str(" { ");
                for (i, (field_name, field_value)) in fields.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.output.push_str(field_name);
                    self.output.push_str(": ");
                    self.generate_expression(field_value);
                }
                self.output.push_str(" }");
            }
            Expression::BinaryOp(left, op, right) => {
                match op {
                    BinaryOp::Add => {
                        // Check if this is string concatenation
                        let left_is_string = matches!(**left, Expression::StringLiteral(_));
                        let right_is_string = matches!(**right, Expression::StringLiteral(_));
                        
                        if left_is_string || right_is_string {
                            // String concatenation - check if we can flatten nested concatenations
                            let mut parts = Vec::new();
                            self.collect_string_parts(left, &mut parts);
                            self.collect_string_parts(right, &mut parts);
                            
                            if parts.len() > 1 {
                                // Use format! for multiple parts
                                let format_str = "{}".repeat(parts.len());
                                self.output.push_str(&format!("format!(\"{}\", ", format_str));
                                for (i, _) in parts.iter().enumerate() {
                                    if i > 0 {
                                        self.output.push_str(", ");
                                    }
                                    self.generate_expression(&parts[i]);
                                }
                                self.output.push(')');
                            } else {
                                // Fallback to simple concatenation
                                self.generate_expression(left);
                                self.output.push_str(".to_string() + &");
                                self.generate_expression(right);
                                self.output.push_str(".to_string()");
                            }
                        } else {
                            // Numeric addition
                            self.generate_expression(left);
                            self.output.push_str(" + ");
                            self.generate_expression(right);
                        }
                    }
                    _ => {
                        self.generate_expression(left);
                        self.output.push(' ');
                        self.emit_binary_op(op);
                        self.output.push(' ');
                        self.generate_expression(right);
                    }
                }
            }
            Expression::FunctionCall(name, args) => {
                self.output.push_str(&to_snake_case(name));
                self.output.push('(');
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    if matches!(arg, Expression::StringLiteral(_)) {
                        self.generate_expression(arg);
                        self.output.push_str(".to_string()");
                    } else {
                        self.generate_expression(arg);
                    }
                }
                self.output.push(')');
            }
            Expression::MethodCall { object, method, arguments } => {
                self.generate_expression(object);
                self.output.push('.');
                
                match method.as_str() {
                    "push" => {
                        self.output.push_str("push");
                        self.output.push('(');
                        for (i, arg) in arguments.iter().enumerate() {
                            if i > 0 {
                                self.output.push_str(", ");
                            }
                            self.generate_expression(arg);
                        }
                        self.output.push(')');
                    }
                    "pop" => {
                        self.output.push_str("pop()");
                    }
                    "shift" => {
                        self.output.push_str("remove(0)");
                    }
                    "unshift" => {
                        self.output.push_str("insert(0, ");
                        if !arguments.is_empty() {
                            self.generate_expression(&arguments[0]);
                        }
                        self.output.push(')');
                    }
                    "slice" => {
                        self.output.push('[');
                        if arguments.len() >= 1 {
                            self.generate_expression(&arguments[0]);
                            self.output.push_str(" as usize");
                        } else {
                            self.output.push('0');
                        }
                        self.output.push_str("..");
                        if arguments.len() >= 2 {
                            self.generate_expression(&arguments[1]);
                            self.output.push_str(" as usize");
                        }
                        self.output.push_str("].to_vec()");
                    }
                    "map" | "filter" => {
                        self.output.push_str(method);
                        self.output.push('(');
                        for (i, arg) in arguments.iter().enumerate() {
                            if i > 0 {
                                self.output.push_str(", ");
                            }
                            self.generate_expression(arg);
                        }
                        self.output.push_str(").collect()");
                    }
                    "charAt" => {
                        self.output.push_str("chars().nth(");
                        if !arguments.is_empty() {
                            self.generate_expression(&arguments[0]);
                            self.output.push_str(" as usize");
                        }
                        self.output.push_str(").unwrap_or('\\0')");
                    }
                    "substring" => {
                        self.output.push_str("chars().skip(");
                        if arguments.len() >= 1 {
                            self.generate_expression(&arguments[0]);
                            self.output.push_str(" as usize");
                        } else {
                            self.output.push('0');
                        }
                        self.output.push_str(").take(");
                        if arguments.len() >= 2 {
                            self.output.push('(');
                            self.generate_expression(&arguments[1]);
                            self.output.push_str(" - ");
                            if arguments.len() >= 1 {
                                self.generate_expression(&arguments[0]);
                            } else {
                                self.output.push('0');
                            }
                            self.output.push_str(") as usize");
                        } else {
                            self.output.push_str("usize::MAX");
                        }
                        self.output.push_str(").collect::<String>()");
                    }
                    "indexOf" => {
                        self.output.push_str("find(");
                        if !arguments.is_empty() {
                            self.generate_expression(&arguments[0]);
                        }
                        self.output.push_str(").map(|i| i as i32).unwrap_or(-1)");
                    }
                    "toUpperCase" => {
                        self.output.push_str("to_uppercase()");
                    }
                    "toLowerCase" => {
                        self.output.push_str("to_lowercase()");
                    }
                    "trim" => {
                        self.output.push_str("trim().to_string()");
                    }
                    "split" => {
                        self.output.push_str("split(");
                        if !arguments.is_empty() {
                            self.generate_expression(&arguments[0]);
                        }
                        self.output.push_str(").map(|s| s.to_string()).collect::<Vec<String>>()");
                    }
                    "join" => {
                        self.output.push_str("join(");
                        if !arguments.is_empty() {
                            self.generate_expression(&arguments[0]);
                        } else {
                            self.output.push_str("\", \"");
                        }
                        self.output.push(')');
                    }
                    "reverse" => {
                        self.output.push_str("iter().rev().cloned().collect::<Vec<_>>()");
                    }
                    "sort" => {
                        self.output.push_str("sort()");
                    }
                    "includes" | "contains" => {
                        self.output.push_str("contains(");
                        if !arguments.is_empty() {
                            self.output.push('&');
                            self.generate_expression(&arguments[0]);
                        }
                        self.output.push(')');
                    }
                    _ => {
                        self.output.push_str(method);
                        self.output.push('(');
                        for (i, arg) in arguments.iter().enumerate() {
                            if i > 0 {
                                self.output.push_str(", ");
                            }
                            self.generate_expression(arg);
                        }
                        self.output.push(')');
                    }
                }
            }
            Expression::IndexAccess { object, index } => {
                self.generate_expression(object);
                self.output.push('[');
                self.generate_expression(index);
                self.output.push_str(" as usize");
                self.output.push(']');
            }
            Expression::MemberAccess { object, member } => {
                self.generate_expression(object);
                self.output.push('.');
                if member == "length" {
                    self.output.push_str("len() as i32");
                } else {
                    self.output.push_str(member);
                }
            }
        }
    }

    fn emit_type(&mut self, type_: &Type) {
        match type_ {
            Type::Number => self.output.push_str("i32"),
            Type::String => self.output.push_str("String"),
            Type::Boolean => self.output.push_str("bool"),
            Type::Void => self.output.push_str("()"),
            Type::Any => self.output.push_str("String"),
            Type::Array { element_type, size } => {
                if let Some(size) = size {
                    self.output.push('[');
                    self.emit_type(element_type);
                    self.output.push_str("; ");
                    self.output.push_str(&size.to_string());
                    self.output.push(']');
                } else {
                    self.output.push_str("Vec<");
                    self.emit_type(element_type);
                    self.output.push('>');
                }
            }
            Type::Custom(name) => {
                self.output.push_str(name);
            }
            Type::Inferred => {
            }
        }
    }

    fn emit_binary_op(&mut self, op: &BinaryOp) {
        match op {
            BinaryOp::Add => self.output.push('+'),
            BinaryOp::Subtract => self.output.push('-'),
            BinaryOp::Multiply => self.output.push('*'),
            BinaryOp::Divide => self.output.push('/'),
            BinaryOp::Modulo => self.output.push('%'),
            BinaryOp::Equal => self.output.push_str("=="),
            BinaryOp::NotEqual => self.output.push_str("!="),
            BinaryOp::Greater => self.output.push('>'),
            BinaryOp::GreaterEqual => self.output.push_str(">="),
            BinaryOp::Less => self.output.push('<'),
            BinaryOp::LessEqual => self.output.push_str("<="),
            BinaryOp::And => self.output.push_str("&&"),
            BinaryOp::Or => self.output.push_str("||"),
        }
    }

    fn emit_indent(&mut self) {
        for _ in 0..self.indent_level {
            self.output.push_str("    ");
        }
    }

    fn emit_header(&mut self) {
        self.output.push_str("fn main() {\n");
        self.indent_level = 1;
    }

    fn emit_main_if_needed(&mut self) {
        self.indent_level = 0;
        self.output.push_str("}\n");
    }
    
    fn collect_string_parts<'a>(&self, expr: &'a Expression, parts: &mut Vec<&'a Expression>) {
        match expr {
            Expression::BinaryOp(left, BinaryOp::Add, right) => {
                // Check if this is string concatenation
                let left_is_string = matches!(**left, Expression::StringLiteral(_));
                let right_is_string = matches!(**right, Expression::StringLiteral(_));
                
                if left_is_string || right_is_string {
                    self.collect_string_parts(left, parts);
                    self.collect_string_parts(right, parts);
                } else {
                    parts.push(expr);
                }
            }
            _ => {
                parts.push(expr);
            }
        }
    }
}
