use crate::ast::{BinaryOp, Expr, Function, Item, Program, Stmt};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Void,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Void => write!(f, "void"),
        }
    }
}

pub struct Interpreter {
    functions: HashMap<String, Function>,
}

impl Interpreter {
    pub fn new(program: &Program) -> Self {
        let mut functions = HashMap::new();

        for item in &program.items {
            if let Item::Function(function) = item {
                functions.insert(function.name.clone(), function.clone());
            }
        }

        Self { functions }
    }

    pub fn run(&mut self) -> Result<Value, String> {
        self.call_function("main", Vec::new())
    }

    fn call_function(&mut self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        // Built-in function.
        if name == "print" {
            for (index, value) in args.iter().enumerate() {
                if index > 0 {
                    print!(" ");
                }
                print!("{value}");
            }
            println!();
            return Ok(Value::Void);
        }

        let function = self
            .functions
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Undefined function: {name}"))?;

        if function.params.len() != args.len() {
            return Err(format!(
                "Function '{name}' expects {} arguments, got {}",
                function.params.len(),
                args.len()
            ));
        }

        // Each function call gets its own local environment.
        let mut environment = HashMap::new();

        for (parameter, argument) in function.params.iter().zip(args) {
            environment.insert(parameter.name.clone(), argument);
        }

        for statement in &function.body {
            if let Some(value) = self.execute_statement(statement, &mut environment)? {
                return Ok(value);
            }
        }

        Ok(Value::Void)
    }

    fn execute_statement(
        &mut self,
        statement: &Stmt,
        environment: &mut HashMap<String, Value>,
    ) -> Result<Option<Value>, String> {
        match statement {
            Stmt::Let { name, value, .. } => {
                let value = self.evaluate(value, environment)?;
                environment.insert(name.clone(), value);
                Ok(None)
            }

            Stmt::Return(expression) => {
                let value = match expression {
                    Some(expression) => self.evaluate(expression, environment)?,
                    None => Value::Void,
                };

                Ok(Some(value))
            }

            Stmt::Expr(expression) => {
                self.evaluate(expression, environment)?;
                Ok(None)
            }
        }
    }

    fn evaluate(
        &mut self,
        expression: &Expr,
        environment: &HashMap<String, Value>,
    ) -> Result<Value, String> {
        match expression {
            Expr::Number(number) => {
                let number = number
                    .parse::<f64>()
                    .map_err(|_| format!("Invalid number: {number}"))?;

                Ok(Value::Number(number))
            }

            Expr::String(value) => Ok(Value::String(value.clone())),

            Expr::Variable(name) => environment
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable: {name}")),

            Expr::Binary { left, op, right } => {
                let left = self.evaluate(left, environment)?;
                let right = self.evaluate(right, environment)?;

                match (left, right) {
                    (Value::Number(a), Value::Number(b)) => {
                        let result = match op {
                            BinaryOp::Add => a + b,
                            BinaryOp::Subtract => a - b,
                            BinaryOp::Multiply => a * b,
                            BinaryOp::Divide => {
                                if b == 0.0 {
                                    return Err("Division by zero".to_string());
                                }

                                a / b
                            }
                        };

                        Ok(Value::Number(result))
                    }

                    (Value::String(a), Value::String(b)) if matches!(op, BinaryOp::Add) => {
                        Ok(Value::String(a + &b))
                    }

                    _ => Err("Invalid types for binary operation".to_string()),
                }
            }

            Expr::Call { callee, args } => {
                let name = match callee.as_ref() {
                    Expr::Variable(name) => name.clone(),
                    _ => {
                        return Err("Only named functions can be called".to_string());
                    }
                };

                let mut values = Vec::new();

                for argument in args {
                    values.push(self.evaluate(argument, environment)?);
                }

                self.call_function(&name, values)
            }
        }
    }
}
