use std::collections::HashMap;
use ast::Type;
use semantic_analysis::errors::{TError, ErrStack};

type Env = HashMap<String, (Type, bool)>;

#[derive(Clone)]
pub struct TCContext {
    // Identifier -> (Type, IsFromCurrentScope)
    env: Env,
    return_type: Type,
}

impl TCContext {
    pub fn new() -> TCContext {
        TCContext {
            env: Env::new(),
            return_type: Type::TVoid,
        }
    }

    pub fn return_type(&self) -> Type {
        self.return_type.clone()
    }

    pub fn get(&self, id: &String) -> TError<Type> {
        match self.env.get(id) {
            Some(t) => Ok(t.0.clone()),
            None => Err(ErrStack::undeclared(id)),
        }
    }

    pub fn add(&mut self, id: &String, t: &Type) -> TError<()> {
        match self.env.get(id) {
            None |
            Some(&(_, false)) => {
                self.env.insert(id.clone(), (t.clone(), true));
                Ok(())
            },
            _ => Err(ErrStack::redefinition(id))
        }
    }

    pub fn in_new_function<T, F>(&self, ret_type: &Type, fun: F) -> T
        where F: Fn(&mut TCContext) -> T
    {
        let mut new_env = self.make_new_context();
        new_env.return_type = ret_type.clone();
        fun(&mut new_env)
    }

    pub fn in_new_scope<T, F>(&self, fun: F) -> T
        where F: Fn(&mut TCContext) -> T
    {
        let mut new_env = self.make_new_context();
        fun(&mut new_env)
    }

    fn make_new_context(&self) -> TCContext {
        TCContext {
            env: self.clone().env.into_iter().map(|(ref var, ref t)| (var.clone(), (t.0.clone(), false))).collect(),
            return_type: self.return_type.clone(),
        }
    }
}

