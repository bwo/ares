use std::rc::Rc;
use std::cell::RefCell;

use ::{Value, rc_to_usize, write_usize};

pub use super::environment::{Env, Environment};

#[derive(Clone)]
pub struct Procedure {
    pub name: Option<String>,
    pub bodies: Rc<Vec<Value>>,
    param_names: Vec<String>, // TODO: allow this to also be a single identifier for varargs
    environment: Env
}

impl Procedure {
    pub fn new(name: Option<String>, bodies: Rc<Vec<Value>>, param_names: Vec<String>, env: Env) -> Procedure {
        Procedure {
            name: name,
            bodies: bodies,
            param_names: param_names,
            environment: env
        }
    }

    pub fn gen_env<I: Iterator<Item=Value>>(&self, values: I) -> Env {
                    Environment::new_with_data(
                        self.environment.clone(),
                        self.param_names.iter().cloned().zip(values).collect())
    }
}

impl PartialEq for Procedure {
    fn eq(&self, other: &Procedure) -> bool {
        rc_to_usize(&self.bodies) == rc_to_usize(&other.bodies) &&
        rc_to_usize(&self.environment) == rc_to_usize(&other.environment)
    }
}

impl Eq for Procedure {}

impl ::std::fmt::Debug for Procedure {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error>{
        fmt.write_str("<lambda>")
    }
}

impl ::std::hash::Hash for Procedure {
    fn hash<H>(&self, state: &mut H) where H: ::std::hash::Hasher {
        write_usize(rc_to_usize(&self.bodies), state);
        write_usize(rc_to_usize(&self.environment), state);
    }
}
