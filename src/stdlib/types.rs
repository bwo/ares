use std::collections::HashSet;
use std::rc::Rc;

use ::{Value, AresResult, AresError, rc_to_usize};
use super::util::{no_more_or_arity_err, unwrap_or_arity_err};

macro_rules! gen_is_type {
    ($name: ident, $p: ident) => {
        pub fn $name(values: &mut Iterator<Item=Value>) -> AresResult<Value> {
            for item in values {
                if let Value::$p(_) = item {
                } else {
                    return Ok(false.into())
                }
            }
            Ok(true.into())
        }
    }
}

gen_is_type!(is_int, Int);
gen_is_type!(is_float, Float);
gen_is_type!(is_bool, Bool);
gen_is_type!(is_string, String);
gen_is_type!(is_list, List);
gen_is_type!(is_ident, Ident);
gen_is_type!(is_lambda, Lambda);
gen_is_type!(is_foreign_fn, ForeignFn);

pub fn is_executable(values: &mut Iterator<Item=Value>) -> AresResult<Value> {
    for item in values {
        match item {
            Value::Lambda(_) => {},
            Value::ForeignFn(_) => {},
            _ => return Ok(false.into())
        }
    }

    Ok(true.into())
}


pub fn to_int(values: &mut Iterator<Item=Value>) -> AresResult<Value> {
     let res = match try!(unwrap_or_arity_err(values.next(), 0, "exactly 1")) {
         Value::Int(i) => Ok(Value::Int(i)),
         Value::Float(f) => Ok(Value::Int(f as i64)),
         Value::Bool(b) => Ok(Value::Int(if b {1} else {0})),
         Value::String(s) => Ok(Value::Int(s.parse().unwrap())),
         other => Err(AresError::IllegalConversion {
             value: other,
             into: "Int".to_string()
         })
     };
     try!(no_more_or_arity_err(values, 1, "exactly 1"));
     res
}

pub fn to_float(values: &mut Iterator<Item=Value>) -> AresResult<Value> {
     let res = match try!(unwrap_or_arity_err(values.next(), 0, "exactly 1")){
         Value::Int(i) => Ok(Value::Float(i as f64)),
         Value::Float(f) => Ok(Value::Float(f)),
         Value::String(s) => Ok(Value::Float(s.parse().unwrap())),
         other => Err(AresError::IllegalConversion {
             value: other,
             into: "Float".to_string()
         })
     };
     try!(no_more_or_arity_err(values, 1, "exactly 1"));
     res
}

pub fn to_bool(values: &mut Iterator<Item=Value>) -> AresResult<Value> {
     let res = match try!(unwrap_or_arity_err(values.next(), 0, "exactly 1")) {
         Value::Int(0) => Ok(Value::Bool(false)),
         Value::Int(_) => Ok(Value::Bool(true)),
         Value::Float(0.0) => Ok(Value::Bool(false)),
         // TODO: Float(nan) => Ok(false)?
         Value::Float(_) => Ok(Value::Bool(true)),
         Value::Bool(b) => Ok(Value::Bool(b)),
         Value::String(s) => {
             if &**s == "true" {
                 Ok(Value::Bool(true))
             } else if &**s == "false" {
                 Ok(Value::Bool(false))
             } else {
                 Err(AresError::IllegalConversion{
                     value: Value::String(s),
                     into: "Bool".to_string()
                 })
             }
         }
         other => Err(AresError::IllegalConversion {
             value: other,
             into: "Bool".to_string()
         })
     };
     try!(no_more_or_arity_err(values, 1, "exactly 1"));
     res
}

pub fn to_string(values: &mut Iterator<Item=Value>) -> AresResult<Value> {
    let first = try!(unwrap_or_arity_err(values.next(), 0, "exactly 1"));
    try!(no_more_or_arity_err(values, 1, "exactly 1"));
    let s = to_string_helper(&first);
    Ok(Value::String(Rc::new(s)))
}

fn to_string_helper(value: &Value) -> String {
    match value {
        &Value::Int(i) => format!("{}", i),
        &Value::Float(f) => format!("{}", f),
        &Value::String(ref s) => (&**s).clone(),
        &Value::Bool(b) => format!("{}", b),
        &Value::ForeignFn(ref ff) => format!("<#{}>", ff.name),
        &Value::Lambda(ref l) => format!("<@{}>", l.name.as_ref().map(|s| &s[..]).unwrap_or("anonymous")),
        &Value::Ident(ref i) => format!("'{}", i),

        &ref l@Value::List(_) => {
            fn build_buf(cur: &Value, buf: &mut String, seen: &mut HashSet<usize>) {
                match cur {
                    &Value::List(ref l) => {
                        let ptr = rc_to_usize(l);
                        if seen.contains(&ptr) {
                            buf.push_str("...");
                        } else {
                            seen.insert(ptr);
                            buf.push_str("[");
                            for v in l.iter() {
                                build_buf(v, buf, seen);
                                buf.push_str(", ");
                            }
                            // removing trailing comma and space
                            buf.pop();
                            buf.pop();
                            buf.push_str("]");
                        }
                    }
                    other => {
                        buf.push_str(&to_string_helper(&other))
                    }
                }
            }

            let mut inner = String::new();
            let mut seen = HashSet::new();
            build_buf(&l, &mut inner, &mut seen);
            inner
        }
    }
}
