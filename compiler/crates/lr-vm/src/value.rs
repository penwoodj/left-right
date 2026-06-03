use gc_arena::{Collect, Gc, Mutation};
use std::fmt;
use serde_json;

#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct ClosureData {
    pub body_start: usize,
    pub arg_count: u8,
}

#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct PartialClosureData<'gc> {
    pub body_start: usize,
    pub arg_count: u8,
    pub bound_left: Value<'gc>,
}

#[derive(Debug, Copy, Clone, Collect)]
#[collect(no_drop)]
pub enum Value<'gc> {
    Undefined,
    Boolean(bool),
    Number(f64),
    String(Gc<'gc, String>),
    List(Gc<'gc, Vec<Value<'gc>>>),
    Map(Gc<'gc, Vec<(Value<'gc>, Value<'gc>)>>),
    Operator(Gc<'gc, OperatorData>),
    PartialOperator(Gc<'gc, PartialOperatorData<'gc>>),
    Closure(Gc<'gc, ClosureData>),
    PartialClosure(Gc<'gc, PartialClosureData<'gc>>),
    Error(Gc<'gc, ErrorData<'gc>>),
}

#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct ErrorData<'gc> {
    pub message: Value<'gc>,
    pub properties: Gc<'gc, Vec<(Value<'gc>, Value<'gc>)>>,
}

#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct OperatorData {
    pub name: String,
}

#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct PartialOperatorData<'gc> {
    pub name: String,
    pub left_arg: Value<'gc>,
}

impl<'gc> Value<'gc> {
    pub fn undefined() -> Self {
        Value::Undefined
    }

    pub fn boolean(_mc: &Mutation<'gc>, value: bool) -> Self {
        Value::Boolean(value)
    }

    pub fn number(value: f64) -> Self {
        Value::Number(value)
    }

    pub fn string(mc: &Mutation<'gc>, value: String) -> Self {
        Value::String(Gc::new(mc, value))
    }

    pub fn list(mc: &Mutation<'gc>, value: Vec<Value<'gc>>) -> Self {
        Value::List(Gc::new(mc, value))
    }

    pub fn map(mc: &Mutation<'gc>, value: Vec<(Value<'gc>, Value<'gc>)>) -> Self {
        Value::Map(Gc::new(mc, value))
    }

    pub fn operator(mc: &Mutation<'gc>, name: String) -> Self {
        Value::Operator(Gc::new(mc, OperatorData { name }))
    }

    pub fn partial_operator(mc: &Mutation<'gc>, name: String, left_arg: Value<'gc>) -> Self {
        Value::PartialOperator(Gc::new(mc, PartialOperatorData { name, left_arg }))
    }

    pub fn closure(mc: &Mutation<'gc>, body_start: usize, arg_count: u8) -> Self {
        Value::Closure(Gc::new(mc, ClosureData { body_start, arg_count }))
    }

    pub fn error(mc: &Mutation<'gc>, message: Value<'gc>) -> Self {
        Value::Error(Gc::new(mc, ErrorData {
            message,
            properties: Gc::new(mc, vec![]),
        }))
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Undefined => false,
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0 && !n.is_nan(),
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Map(m) => !m.is_empty(),
            Value::Operator(_) => true,
            Value::PartialOperator(_) => true,
            Value::Closure(_) => true,
            Value::PartialClosure(_) => true,
            Value::Error(_) => true,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Undefined => "undefined",
            Value::Boolean(_) => "boolean",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::List(_) => "list",
            Value::Map(_) => "map",
            Value::Operator(_) => "operator",
            Value::PartialOperator(_) => "partial_operator",
            Value::Closure(_) => "closure",
            Value::PartialClosure(_) => "partial_closure",
            Value::Error(_) => "error",
        }
    }

    /// Serialize this Value to a serde_json::Value for cross-thread transfer.
    /// Only data types (Undefined, Boolean, Number, String, List, Map) are serializable.
    /// Operators, closures, and errors serialize to Null.
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Value::Undefined => serde_json::Value::Null,
            Value::Boolean(b) => serde_json::Value::Bool(*b),
            Value::Number(n) => {
                serde_json::Value::Number(
                    serde_json::Number::from_f64(*n)
                        .unwrap_or_else(|| serde_json::Number::from(0))
                )
            }
            Value::String(s) => serde_json::Value::String(s.to_string()),
            Value::List(l) => {
                serde_json::Value::Array(l.iter().map(|v| v.to_json()).collect())
            }
            Value::Map(m) => {
                let mut obj = serde_json::Map::new();
                for (key, val) in m.iter() {
                    // Only string keys produce valid JSON object keys
                    let key_str = match key {
                        Value::String(s) => s.to_string(),
                        _ => key.to_string(),
                    };
                    obj.insert(key_str, val.to_json());
                }
                serde_json::Value::Object(obj)
            }
            Value::Operator(_)
            | Value::PartialOperator(_)
            | Value::Closure(_)
            | Value::PartialClosure(_)
            | Value::Error(_) => serde_json::Value::Null,
        }
    }

    pub fn deep_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Undefined, Value::Undefined) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => {
                if a.is_nan() && b.is_nan() {
                    true
                } else if a.is_nan() || b.is_nan() {
                    false
                } else {
                    a == b
                }
            }
            (Value::String(a), Value::String(b)) => a == b,
            (Value::List(a), Value::List(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                a.iter().zip(b.iter()).all(|(x, y)| x.deep_eq(y))
            }
            (Value::Map(a), Value::Map(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                for (key_a, val_a) in a.iter() {
                    match b.iter().find(|(key_b, _)| key_b.deep_eq(key_a)) {
                        Some((_, val_b)) => {
                            if !val_a.deep_eq(val_b) {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }
                true
            }
            (Value::Operator(a), Value::Operator(b)) => a.name == b.name,
            (Value::PartialOperator(a), Value::PartialOperator(b)) => {
                a.name == b.name && a.left_arg.deep_eq(&b.left_arg)
            }
            _ => false,
        }
    }

    pub fn loose_eq(&self, other: &Self) -> bool {
        if !self.is_truthy() && !other.is_truthy() {
            return true;
        }
        if !self.is_truthy() || !other.is_truthy() {
            return false;
        }
        match (self, other) {
            (Value::Undefined, Value::Undefined) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => {
                if a.is_nan() && b.is_nan() { true }
                else if a.is_nan() || b.is_nan() { false }
                else { a == b }
            }
            (Value::String(a), Value::String(b)) => a == b,
            (Value::List(a), Value::List(b)) => {
                if a.len() != b.len() { return false; }
                a.iter().zip(b.iter()).all(|(x, y)| x.loose_eq(y))
            }
            (Value::Map(a), Value::Map(b)) => {
                if a.len() != b.len() { return false; }
                for (key_a, val_a) in a.iter() {
                    match b.iter().find(|(key_b, _)| key_b.loose_eq(key_a)) {
                        Some((_, val_b)) => {
                            if !val_a.loose_eq(val_b) { return false; }
                        }
                        None => return false,
                    }
                }
                true
            }
            (Value::Number(a), Value::String(b)) => {
                match b.parse::<f64>() {
                    Ok(n) => {
                        if a.is_nan() && n.is_nan() { true }
                        else if a.is_nan() || n.is_nan() { false }
                        else { *a == n }
                    }
                    Err(_) => false,
                }
            }
            (Value::String(a), Value::Number(b)) => {
                match a.parse::<f64>() {
                    Ok(n) => {
                        if n.is_nan() && b.is_nan() { true }
                        else if n.is_nan() || b.is_nan() { false }
                        else { n == *b }
                    }
                    Err(_) => false,
                }
            }

            (Value::Boolean(a), Value::Number(b)) => {
                let bool_as_num = if *a { 1.0 } else { 0.0 };
                bool_as_num == *b
            }
            (Value::Number(a), Value::Boolean(b)) => {
                let bool_as_num = if *b { 1.0 } else { 0.0 };
                *a == bool_as_num
            }

            (Value::Boolean(a), Value::String(b)) => {
                let bool_str = if *a { "true" } else { "false" };
                bool_str == b.as_str()
            }
            (Value::String(a), Value::Boolean(b)) => {
                let bool_str = if *b { "true" } else { "false" };
                a.as_str() == bool_str
            }

            _ => false,
        }
    }
}

impl<'gc> fmt::Display for Value<'gc> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Undefined => write!(f, "undefined"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::String(s) => write!(f, "{}", s),
            Value::List(l) => {
                write!(f, "[")?;
                for (i, elem) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, "]")
            }
            Value::Map(m) => {
                write!(f, "{{")?;
                for (i, (key, val)) in m.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, val)?;
                }
                write!(f, "}}")
            }
            Value::Operator(op) => write!(f, "<operator:{}>", op.name),
            Value::PartialOperator(op) => write!(f, "<partial_operator:{}>", op.name),
            Value::Closure(c) => write!(f, "<closure:@{} args={}>", c.body_start, c.arg_count),
            Value::PartialClosure(pc) => write!(f, "<partial_closure:@{} args={}>", pc.body_start, pc.arg_count),
            Value::Error(e) => write!(f, "Error: {}", e.message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gc_arena::{Arena, Rootable};
    use std::marker::PhantomData;

    #[derive(Debug, Clone, Copy, Collect)]
    #[collect(no_drop)]
    struct TestRoot<'gc> {
        _marker: PhantomData<&'gc ()>,
    }

    fn with_arena<F, R>(f: F) -> R
    where
        F: for<'gc> FnOnce(&Mutation<'gc>, &TestRoot<'gc>) -> R,
    {
        let arena = Arena::<Rootable![TestRoot<'_>]>::new(|_mc| TestRoot {
            _marker: PhantomData,
        });
        arena.mutate(f)
    }

    #[test]
    fn test_value_undefined() {
        let val = Value::undefined();
        assert!(!val.is_truthy());
        assert_eq!(val.type_name(), "undefined");
        assert_eq!(format!("{}", val), "undefined");
    }

    #[test]
    fn test_value_boolean() {
        with_arena(|mc, _| {
            let t = Value::boolean(mc, true);
            let f = Value::boolean(mc, false);

            assert!(t.is_truthy());
            assert!(!f.is_truthy());
            assert_eq!(t.type_name(), "boolean");
            assert_eq!(format!("{}", t), "true");
            assert_eq!(format!("{}", f), "false");
        });
    }

    #[test]
    fn test_value_number() {
        let int_val = Value::number(42.0);
        let float_val = Value::number(3.25);
        let zero_val = Value::number(0.0);
        let nan_val = Value::number(f64::NAN);

        assert!(int_val.is_truthy());
        assert!(float_val.is_truthy());
        assert!(!zero_val.is_truthy());
        assert!(!nan_val.is_truthy());

        assert_eq!(format!("{}", int_val), "42");
        assert_eq!(format!("{}", float_val), "3.25");
    }

    #[test]
    fn test_value_string() {
        with_arena(|mc, _| {
            let s = Value::string(mc, "hello".to_string());
            let empty = Value::string(mc, "".to_string());

            assert!(s.is_truthy());
            assert!(!empty.is_truthy());
            assert_eq!(s.type_name(), "string");
            assert_eq!(format!("{}", s), "hello");
        });
    }

    #[test]
    fn test_value_list() {
        with_arena(|mc, _| {
            let list = Value::list(mc, vec![Value::number(1.0), Value::number(2.0)]);
            let empty = Value::list(mc, vec![]);

            assert!(list.is_truthy());
            assert!(!empty.is_truthy());
            assert_eq!(format!("{}", list), "[1, 2]");
        });
    }

    #[test]
    fn test_value_map() {
        with_arena(|mc, _| {
            let key = Value::string(mc, "key".to_string());
            let val = Value::number(42.0);
            let map = Value::map(mc, vec![(key, val)]);
            let empty = Value::map(mc, vec![]);

            assert!(map.is_truthy());
            assert!(!empty.is_truthy());
            assert!(format!("{}", map).starts_with("{"));
        });
    }

    #[test]
    fn test_deep_eq() {
        with_arena(|mc, _| {
            let undef1 = Value::undefined();
            let undef2 = Value::undefined();
            assert!(undef1.deep_eq(&undef2));

            let bool1 = Value::boolean(mc, true);
            let bool2 = Value::boolean(mc, true);
            let bool3 = Value::boolean(mc, false);
            assert!(bool1.deep_eq(&bool2));
            assert!(!bool1.deep_eq(&bool3));

            let num1 = Value::number(42.0);
            let num2 = Value::number(42.0);
            let num3 = Value::number(43.0);
            assert!(num1.deep_eq(&num2));
            assert!(!num1.deep_eq(&num3));

            let str1 = Value::string(mc, "hello".to_string());
            let str2 = Value::string(mc, "hello".to_string());
            let str3 = Value::string(mc, "world".to_string());
            assert!(str1.deep_eq(&str2));
            assert!(!str1.deep_eq(&str3));

            let list1 = Value::list(mc, vec![Value::number(1.0), Value::number(2.0)]);
            let list2 = Value::list(mc, vec![Value::number(1.0), Value::number(2.0)]);
            let list3 = Value::list(mc, vec![Value::number(1.0)]);
            assert!(list1.deep_eq(&list2));
            assert!(!list1.deep_eq(&list3));
        });
    }

    #[test]
    fn test_nan_equality() {
        let nan1 = Value::number(f64::NAN);
        let nan2 = Value::number(f64::NAN);
        let num = Value::number(42.0);

        assert!(nan1.deep_eq(&nan2), "NaN should equal NaN with deep_eq");
        assert!(!nan1.deep_eq(&num));
    }
}