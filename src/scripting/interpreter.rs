use lazy_static::lazy_static;
use rustc_hash::FxHashMap;
use std::{rc::Rc, str::FromStr, sync::RwLock};

pub type VariableMapType = FxHashMap<Rc<String>, Variable>;
pub type ConditionalFn = Box<dyn FnMut() -> bool>;

pub trait Executable {
    fn execute(&mut self) -> anyhow::Result<()>;
}

#[derive(Debug, PartialEq, Clone)]
pub enum Variable {
    Int(usize),
    Float(f32),
    RcStr(Rc<String>),
    Bool(bool),
    Vector(Vec<Variable>),
    Map(VariableMapType),
}

impl From<&str> for Variable {
    fn from(string: &str) -> Self {
        Self::RcStr(Rc::new(string.into()))
    }
}

impl From<String> for Variable {
    fn from(string: String) -> Self {
        Self::RcStr(Rc::new(string))
    }
}

impl From<usize> for Variable {
    fn from(number: usize) -> Self {
        Self::Int(number)
    }
}

impl From<f32> for Variable {
    fn from(number: f32) -> Self {
        Self::Float(number)
    }
}

impl From<bool> for Variable {
    fn from(boolean: bool) -> Self {
        Self::Bool(boolean)
    }
}

impl From<VariableMapType> for Variable {
    fn from(variables_map: VariableMapType) -> Self {
        Self::Map(variables_map)
    }
}

impl<V: Into<Variable>> From<Vec<V>> for Variable {
    fn from(vec: Vec<V>) -> Self {
        Self::Vector(vec.into_iter().map(|v| v.into()).collect())
    }
}

impl PartialEq<String> for Variable {
    fn eq(&self, other: &String) -> bool {
        use Variable::*;
        match self {
            Int(i) => {
                let num: usize = match other.parse() {
                    Ok(num) => num,
                    Err(_) => return false,
                };
                *i == num
            }
            RcStr(string) => **string == *other,
            Bool(boolean) => {
                let boolean = *boolean;
                if boolean && other == "true" {
                    true
                } else if !boolean && other == "false" {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

pub struct Instruction(Box<dyn FnMut() -> anyhow::Result<()>>);

impl<F: FnMut() -> anyhow::Result<()> + 'static> From<F> for Instruction {
    fn from(function: F) -> Self {
        Self(Box::new(function))
    }
}

impl From<Instruction> for Box<dyn Executable> {
    fn from(instruction: Instruction) -> Self {
        Box::new(instruction)
    }
}

impl Executable for Instruction {
    fn execute(&mut self) -> anyhow::Result<()> {
        (self.0)()?;
        Ok(())
    }
}

pub struct Iterative {
    index: usize,
    executables: Vec<Box<dyn Executable>>,
    key: String,
    variable_map: *mut VariableMapType,
}

impl Iterative {
    pub fn new(key: String, variable_map: *mut VariableMapType) -> Self {
        Self {
            key,
            index: 0,
            variable_map,
            executables: vec![],
        }
    }

    pub fn change_key(&mut self, key: String) {
        self.key = key;
    }

    pub fn push(&mut self, executable: Box<dyn Executable>) {
        self.executables.push(executable);
    }
}

impl From<Iterative> for Box<dyn Executable> {
    fn from(iterative: Iterative) -> Self {
        Box::new(iterative)
    }
}


impl Executable for Iterative {
    fn execute(&mut self) -> anyhow::Result<()> {
        let variable_map = unsafe { &mut *self.variable_map };
        
        let vec = match unsafe { &mut *self.variable_map }.get(&self.key) {
            Some(vec) => match vec {
                Variable::Vector(vec) => vec,
                _ => anyhow::bail!("The Value attained with Key {} is not a Vector", self.key),
            },
            None => anyhow::bail!("Value with Key {} does not exist", self.key),
        };

        for variable in vec {
            let map = match variable {
                Variable::Map(map) => map,
                _ => anyhow::bail!("The Value attained with Key {} is not a Map", self.key),
            };

            for (key, variable) in map {
                variable_map.insert(key.clone(), variable.clone());
            }

            for executable in &mut self.executables {
                if let Err(err) = executable.execute() {
                    println!("{}", err);
                }
            }
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct Conditional {
    pub conditions: Vec<Vec<ConditionalFn>>,
    pub executables: Vec<Box<dyn Executable>>,
    pub else_if_conditionals: Option<Vec<Conditional>>,
    pub else_executables: Option<Vec<Box<dyn Executable>>>,
}

impl Executable for Conditional {
    fn execute(&mut self) -> anyhow::Result<()> {
        for conditions in &mut self.conditions {
            let mut should_execute = false;

            for condition in conditions {
                should_execute = condition();
            }

            if should_execute {
                for instruction in &mut self.executables {
                    instruction.execute()?;
                }
                return Ok(());
            }
        }

        if let Some(else_if_conditionals) = &mut self.else_if_conditionals {
            for conditional in else_if_conditionals {
                if conditional.execute().is_ok() {
                    return Ok(());
                }
            }
        } else if let Some(else_executables) = &mut self.else_executables {
            for instruction in else_executables {
                instruction.execute()?;
            }
            return Ok(());
        }

        anyhow::bail!("Didn't execute anything.");
    }
}

impl From<Conditional> for Box<dyn Executable> {
    fn from(conditional: Conditional) -> Self {
        Box::new(conditional)
    }
}
