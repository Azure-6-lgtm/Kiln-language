use crate::ast::{BinaryOp, Expr, Function, Item, Program, Stmt, UnaryOp};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Variable {
    pub value: Value,
    pub mutable: bool,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(BooleanValue),
    Void,
}

#[derive(Debug, Clone)]
pub enum BooleanValue {
    True,
    False,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Boolean(BooleanValue::True) => write!(f, "True"),
            Value::Boolean(BooleanValue::False) => write!(f, "False"),
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
            .ok_or_else(|| format!("Undefined function: {name} [E011]"))?;

        if function.params.len() != args.len() {
            return Err(format!(
                "Function '{name}' expects {} arguments, got {} [E012]",
                function.params.len(),
                args.len()
            ));
        }

        let mut environment: HashMap<String, Variable> = HashMap::new();

        for (parameter, argument) in function.params.iter().zip(args) {
            environment.insert(
                parameter.name.clone(),
                Variable {
                    value: argument,
                    mutable: false,
                },
            );
        }

        let mut result = Value::Void;

        for statement in &function.body {
            if let Some(value) = self.execute_statement(statement, &mut environment)? {
                result = value;
                break;
            }
        }

        match (function.return_type.as_deref(), &result) {
            (Some("int"), Value::Number(n)) if n.fract() == 0.0 => {}
            (Some("float"), Value::Number(_)) => {}
            (Some("string"), Value::String(_)) => {}
            (Some("void"), Value::Void) => {}
            (Some("bool"), Value::Boolean(_)) => {}
            (None, _) => {}
            (Some(expected), actual) => {
                let actual_type = match actual {
                    Value::Number(n) if n.fract() == 0.0 => "int",
                    Value::Number(_) => "float",
                    Value::String(_) => "string",
                    Value::Void => "void",
                    Value::Boolean(_) => "bool",
                };

                return Err(format!(
                    "Function '{}' declared return type '{}' but returned type '{}' [E013]",
                    function.name, expected, actual_type
                ));
            }
        }

        Ok(result)
    }

    fn execute_statement(
        &mut self,
        statement: &Stmt,
        environment: &mut HashMap<String, Variable>,
    ) -> Result<Option<Value>, String> {
        match statement {
            Stmt::Let {
                name,
                mutable,
                value,
            } => {
                let value = self.evaluate(value, environment)?;

                environment.insert(
                    name.clone(),
                    Variable {
                        value,
                        mutable: *mutable,
                    },
                );

                Ok(None)
            }

            Stmt::Return(expression) => {
                let value = match expression {
                    Some(expression) => self.evaluate(expression, environment)?,
                    None => Value::Void,
                };

                Ok(Some(value))
            }

            Stmt::Assign { name, value } => {
                let new_value = self.evaluate(value, environment)?;

                let variable = environment
                    .get_mut(name)
                    .ok_or_else(|| format!("Undefined variable: {name} [E021]"))?;

                if !variable.mutable {
                    return Err(format!(
                        "Cannot assign new value to immutable variable: {name} [E020]"
                    ));
                }

                variable.value = new_value;

                Ok(None)
            }

            Stmt::Expr(expression) => {
                self.evaluate(expression, environment)?;
                Ok(None)
            }

            Stmt::While {
                Condition,
                then_branch,
            } => {
                loop {
                    let condition = self.evaluate(Condition, environment)?;

                    match condition {
                        Value::Boolean(BooleanValue::True) => {
                            for stmt in then_branch {
                                if let Some(value) = self.execute_statement(stmt, environment)? {
                                    return Ok(Some(value));
                                }
                            }
                        }

                        Value::Boolean(BooleanValue::False) => break,

                        _ => {
                            return Err("While condition must be a boolean [E027]".to_string());
                        }
                    }
                }

                Ok(None)
            }

            Stmt::If {
                Condition,
                then_branch,
                else_branch,
            } => {
                let condition = self.evaluate(Condition, environment)?;

                let branch = match condition {
                    Value::Boolean(BooleanValue::True) => Some(then_branch),
                    Value::Boolean(BooleanValue::False) => else_branch.as_ref(),
                    _ => {
                        return Err("If condition must be a boolean [E019]".to_string());
                    }
                };

                if let Some(statements) = branch {
                    for statement in statements {
                        if let Some(value) = self.execute_statement(statement, environment)? {
                            return Ok(Some(value));
                        }
                    }
                }

                Ok(None)
            }
        }
    }

    fn evaluate(
        &mut self,
        expression: &Expr,
        environment: &HashMap<String, Variable>,
    ) -> Result<Value, String> {
        match expression {
            Expr::Number(number) => {
                let number = number
                    .parse::<f64>()
                    .map_err(|_| format!("Invalid number: {number} [E014]"))?;

                Ok(Value::Number(number))
            }

            Expr::String(value) => Ok(Value::String(value.clone())),

            Expr::Variable(name) => environment
                .get(name)
                .map(|variable| variable.value.clone())
                .ok_or_else(|| format!("Undefined variable: {name} [E015]")),

            Expr::Boolean(value) => Ok(boolean_value(*value)),

            Expr::Unary { op, operand } => {
                let value = self.evaluate(operand, environment)?;

                match (op, value) {
                    (UnaryOp::Not, Value::Boolean(BooleanValue::True)) => Ok(boolean_value(false)),

                    (UnaryOp::Not, Value::Boolean(BooleanValue::False)) => Ok(boolean_value(true)),

                    (UnaryOp::Not, _) => {
                        Err("Operator 'not' requires a boolean [E024]".to_string())
                    }
                }
            }

            Expr::Binary { left, op, right } => {
                let left = self.evaluate(left, environment)?;

                match op {
                    BinaryOp::And => {
                        match left {
                            Value::Boolean(BooleanValue::False) => {
                                return Ok(boolean_value(false));
                            }
                            Value::Boolean(BooleanValue::True) => {}
                            _ => {
                                return Err("Operator 'and' requires booleans [E025]".to_string());
                            }
                        }

                        let right = self.evaluate(right, environment)?;

                        match right {
                            Value::Boolean(BooleanValue::True) => Ok(boolean_value(true)),
                            Value::Boolean(BooleanValue::False) => Ok(boolean_value(false)),
                            _ => Err("Operator 'and' requires booleans [E025]".to_string()),
                        }
                    }

                    BinaryOp::Or => {
                        match left {
                            Value::Boolean(BooleanValue::True) => {
                                return Ok(boolean_value(true));
                            }
                            Value::Boolean(BooleanValue::False) => {}
                            _ => {
                                return Err("Operator 'or' requires booleans [E026]".to_string());
                            }
                        }

                        let right = self.evaluate(right, environment)?;

                        match right {
                            Value::Boolean(BooleanValue::True) => Ok(boolean_value(true)),
                            Value::Boolean(BooleanValue::False) => Ok(boolean_value(false)),
                            _ => Err("Operator 'or' requires booleans [E026]".to_string()),
                        }
                    }

                    _ => {
                        let right = self.evaluate(right, environment)?;

                        match (left, right) {
                            (Value::Number(a), Value::Number(b)) => match op {
                                BinaryOp::Add => Ok(Value::Number(a + b)),
                                BinaryOp::Subtract => Ok(Value::Number(a - b)),
                                BinaryOp::Multiply => Ok(Value::Number(a * b)),
                                BinaryOp::Divide => {
                                    if b == 0.0 {
                                        return Err("Division by zero [E016]".to_string());
                                    }

                                    Ok(Value::Number(a / b))
                                }
                                BinaryOp::Equal => Ok(boolean_value(a == b)),
                                BinaryOp::NotEqual => Ok(boolean_value(a != b)),
                                BinaryOp::Less => Ok(boolean_value(a < b)),
                                BinaryOp::LessEqual => Ok(boolean_value(a <= b)),
                                BinaryOp::Greater => Ok(boolean_value(a > b)),
                                BinaryOp::GreaterEqual => Ok(boolean_value(a >= b)),
                                BinaryOp::And | BinaryOp::Or => {
                                    unreachable!("logical operators handled above")
                                }
                            },

                            (Value::String(a), Value::String(b)) => match op {
                                BinaryOp::Add => Ok(Value::String(a + &b)),
                                BinaryOp::Equal => Ok(boolean_value(a == b)),
                                BinaryOp::NotEqual => Ok(boolean_value(a != b)),
                                BinaryOp::And | BinaryOp::Or => {
                                    unreachable!("logical operators handled above")
                                }
                                _ => Err("Invalid string operation [E017]".to_string()),
                            },

                            _ => Err("Invalid types for binary operation [E017]".to_string()),
                        }
                    }
                }
            }

            Expr::Call { callee, args } => {
                let name = match callee.as_ref() {
                    Expr::Variable(name) => name.clone(),
                    _ => {
                        return Err("Only named functions can be called [E018]".to_string());
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

fn boolean_value(value: bool) -> Value {
    Value::Boolean(if value {
        BooleanValue::True
    } else {
        BooleanValue::False
    })
}
