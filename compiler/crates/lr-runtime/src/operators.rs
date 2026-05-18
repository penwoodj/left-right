use gc_arena::{Gc, Mutation};
use lr_vm::value::Value;

/// Arithmetic operators
pub fn op_add<'gc>(_mc: &Mutation<'gc>, left: Value<'gc>, right: Value<'gc>) -> Value<'gc> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::number(a + b),
        (Value::String(a), Value::String(b)) => {
            let combined = format!("{}{}", a, b);
            Value::String(Gc::new(_mc, combined))
        }
        (Value::List(a), Value::List(b)) => {
            let mut result: Vec<Value<'gc>> = a.as_ref().clone();
            result.extend(b.as_ref().clone());
            Value::List(Gc::new(_mc, result))
        }
        (Value::Map(a), Value::Map(b)) => {
            let mut result: Vec<(Value<'gc>, Value<'gc>)> = a.as_ref().clone();
            result.extend(b.as_ref().clone());
            Value::Map(Gc::new(_mc, result))
        }
        _ => Value::undefined(),
    }
}

pub fn op_sub<'gc>(_mc: &Mutation<'gc>, left: Value<'gc>, right: Value<'gc>) -> Value<'gc> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::number(a - b),
        _ => Value::undefined(),
    }
}

pub fn op_mul<'gc>(_mc: &Mutation<'gc>, left: Value<'gc>, right: Value<'gc>) -> Value<'gc> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::number(a * b),
        _ => Value::undefined(),
    }
}

pub fn op_div<'gc>(_mc: &Mutation<'gc>, left: Value<'gc>, right: Value<'gc>) -> Value<'gc> {
    match (left, right) {
        (Value::Number(_), Value::Number(0.0)) => Value::undefined(),
        (Value::Number(a), Value::Number(b)) => Value::number(a / b),
        _ => Value::undefined(),
    }
}

pub fn op_mod<'gc>(_mc: &Mutation<'gc>, left: Value<'gc>, right: Value<'gc>) -> Value<'gc> {
    match (left, right) {
        (Value::Number(_), Value::Number(0.0)) => Value::undefined(),
        (Value::Number(a), Value::Number(b)) => Value::number(a % b),
        _ => Value::undefined(),
    }
}

pub fn op_neg<'gc>(_mc: &Mutation<'gc>, left: Value<'gc>, _right: Value<'gc>) -> Value<'gc> {
    match left {
        Value::Number(n) => Value::number(-n),
        _ => Value::undefined(),
    }
}

/// Comparison operators
pub fn op_eq<'gc>(mc: &Mutation<'gc>, left: Value<'gc>, right: Value<'gc>) -> Value<'gc> {
    Value::boolean(mc, left.deep_eq(&right))
}

pub fn op_ne<'gc>(mc: &Mutation<'gc>, left: Value<'gc>, right: Value<'gc>) -> Value<'gc> {
    Value::boolean(mc, !left.deep_eq(&right))
}

pub fn op_lt<'gc>(mc: &Mutation<'gc>, left: Value<'gc>, right: Value<'gc>) -> Value<'gc> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::boolean(mc, a < b),
        (Value::String(a), Value::String(b)) => Value::boolean(mc, *a < *b),
        _ => Value::undefined(),
    }
}

pub fn op_le<'gc>(mc: &Mutation<'gc>, left: Value<'gc>, right: Value<'gc>) -> Value<'gc> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::boolean(mc, a <= b),
        (Value::String(a), Value::String(b)) => Value::boolean(mc, *a <= *b),
        _ => Value::undefined(),
    }
}

pub fn op_gt<'gc>(mc: &Mutation<'gc>, left: Value<'gc>, right: Value<'gc>) -> Value<'gc> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::boolean(mc, a > b),
        (Value::String(a), Value::String(b)) => Value::boolean(mc, *a > *b),
        _ => Value::undefined(),
    }
}

pub fn op_ge<'gc>(mc: &Mutation<'gc>, left: Value<'gc>, right: Value<'gc>) -> Value<'gc> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Value::boolean(mc, a >= b),
        (Value::String(a), Value::String(b)) => Value::boolean(mc, *a >= *b),
        _ => Value::undefined(),
    }
}

/// Boolean operators
pub fn op_not<'gc>(mc: &Mutation<'gc>, left: Value<'gc>, _right: Value<'gc>) -> Value<'gc> {
    Value::boolean(mc, !left.is_truthy())
}

pub fn op_and<'gc>(mc: &Mutation<'gc>, left: Value<'gc>, right: Value<'gc>) -> Value<'gc> {
    Value::boolean(mc, left.is_truthy() && right.is_truthy())
}

pub fn op_or<'gc>(mc: &Mutation<'gc>, left: Value<'gc>, right: Value<'gc>) -> Value<'gc> {
    Value::boolean(mc, left.is_truthy() || right.is_truthy())
}

/// Map/List operators
pub fn op_get<'gc>(_mc: &Mutation<'gc>, left: Value<'gc>, key: Value<'gc>) -> Value<'gc> {
    match left {
        Value::Map(map) => {
            map.iter()
                .find(|(k, _)| k.deep_eq(&key))
                .map(|(_, v)| *v)
                .unwrap_or(Value::undefined())
        }
        Value::List(list) => match key {
            Value::Number(idx) if idx.fract() == 0.0 => {
                let idx = idx as i64;
                if idx >= 0 && (idx as usize) < list.len() {
                    list[idx as usize]
                } else {
                    Value::undefined()
                }
            }
            _ => Value::undefined(),
        },
        _ => Value::undefined(),
    }
}

pub fn op_get_multiple<'gc>(_mc: &Mutation<'gc>, left: Value<'gc>, keys: Value<'gc>) -> Value<'gc> {
    match (left, keys) {
        (Value::Map(map), Value::List(key_list)) => {
            let mut result: Vec<(Value<'gc>, Value<'gc>)> = Vec::new();
            for key in key_list.iter() {
                if let Some((_, val)) = map.iter().find(|(k, _)| k.deep_eq(key)) {
                    result.push((*key, *val));
                }
            }
            Value::Map(Gc::new(_mc, result))
        }
        _ => Value::undefined(),
    }
}

pub fn op_omit<'gc>(_mc: &Mutation<'gc>, left: Value<'gc>, keys: Value<'gc>) -> Value<'gc> {
    match (left, keys) {
        (Value::Map(map), Value::List(key_list)) => {
            let result: Vec<(Value<'gc>, Value<'gc>)> = map
                .iter()
                .filter(|(k, _)| !key_list.iter().any(|key| key.deep_eq(k)))
                .map(|(k, v)| (*k, *v))
                .collect();
            Value::Map(Gc::new(_mc, result))
        }
        _ => Value::undefined(),
    }
}

pub fn op_size<'gc>(_mc: &Mutation<'gc>, left: Value<'gc>, _right: Value<'gc>) -> Value<'gc> {
    match left {
        Value::String(s) => Value::number(s.len() as f64),
        Value::List(l) => Value::number(l.len() as f64),
        Value::Map(m) => Value::number(m.len() as f64),
        _ => Value::undefined(),
    }
}

pub fn op_reverse_args<'gc>(_mc: &Mutation<'gc>, _left: Value<'gc>, right: Value<'gc>) -> Value<'gc> {
    match right {
        Value::Operator(_op) => {
            Value::undefined()
        }
        _ => Value::undefined(),
    }
}

/// String operators
pub fn op_to_string<'gc>(_mc: &Mutation<'gc>, left: Value<'gc>, _right: Value<'gc>) -> Value<'gc> {
    Value::string(_mc, format!("{}", left))
}

pub fn op_to_lowercase<'gc>(_mc: &Mutation<'gc>, left: Value<'gc>, _right: Value<'gc>) -> Value<'gc> {
    match left {
        Value::String(s) => Value::string(_mc, s.to_lowercase()),
        _ => Value::undefined(),
    }
}

pub fn op_to_uppercase<'gc>(_mc: &Mutation<'gc>, left: Value<'gc>, _right: Value<'gc>) -> Value<'gc> {
    match left {
        Value::String(s) => Value::string(_mc, s.to_uppercase()),
        _ => Value::undefined(),
    }
}

/// Loop operators (placeholders - return undefined)
pub fn op_loop_map<'gc>(_mc: &Mutation<'gc>, _left: Value<'gc>, _right: Value<'gc>) -> Value<'gc> {
    let _ = _mc;
    Value::undefined()
}

pub fn op_loop_filter<'gc>(_mc: &Mutation<'gc>, _left: Value<'gc>, _right: Value<'gc>) -> Value<'gc> {
    let _ = _mc;
    Value::undefined()
}

pub fn op_loop_find<'gc>(_mc: &Mutation<'gc>, _left: Value<'gc>, _right: Value<'gc>) -> Value<'gc> {
    let _ = _mc;
    Value::undefined()
}

pub fn op_loop_every<'gc>(_mc: &Mutation<'gc>, _left: Value<'gc>, _right: Value<'gc>) -> Value<'gc> {
    let _ = _mc;
    Value::undefined()
}

pub fn op_loop_some<'gc>(_mc: &Mutation<'gc>, _left: Value<'gc>, _right: Value<'gc>) -> Value<'gc> {
    let _ = _mc;
    Value::undefined()
}

pub fn op_loop_unique<'gc>(_mc: &Mutation<'gc>, _left: Value<'gc>, _right: Value<'gc>) -> Value<'gc> {
    let _ = _mc;
    Value::undefined()
}

pub fn op_loop_sort<'gc>(_mc: &Mutation<'gc>, _left: Value<'gc>, _right: Value<'gc>) -> Value<'gc> {
    let _ = _mc;
    Value::undefined()
}

#[cfg(test)]
mod tests {
    use super::*;
    use gc_arena::{Arena, Rootable};
    use std::marker::PhantomData;

    #[derive(Debug, Clone, Copy, gc_arena::Collect)]
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
    fn test_op_add_numbers() {
        with_arena(|mc, _| {
            let left = Value::number(5.0);
            let right = Value::number(3.0);
            let result = op_add(mc, left, right);
            assert!(matches!(result, Value::Number(8.0)));
        });
    }

    #[test]
    fn test_op_add_strings() {
        with_arena(|mc, _| {
            let left = Value::string(mc, "hello".to_string());
            let right = Value::string(mc, " world".to_string());
            let result = op_add(mc, left, right);
            if let Value::String(s) = result {
                assert_eq!(&*s, "hello world");
            } else {
                panic!("Expected string");
            }
        });
    }

    #[test]
    fn test_op_add_lists() {
        with_arena(|mc, _| {
            let left = Value::list(mc, vec![Value::number(1.0), Value::number(2.0)]);
            let right = Value::list(mc, vec![Value::number(3.0), Value::number(4.0)]);
            let result = op_add(mc, left, right);
            if let Value::List(l) = result {
                assert_eq!(l.len(), 4);
                assert!(l[0].deep_eq(&Value::number(1.0)));
                assert!(l[1].deep_eq(&Value::number(2.0)));
                assert!(l[2].deep_eq(&Value::number(3.0)));
                assert!(l[3].deep_eq(&Value::number(4.0)));
            } else {
                panic!("Expected list");
            }
        });
    }

    #[test]
    fn test_op_add_maps() {
        with_arena(|mc, _| {
            let key1 = Value::string(mc, "a".to_string());
            let val1 = Value::number(1.0);
            let key2 = Value::string(mc, "b".to_string());
            let val2 = Value::number(2.0);
            let left = Value::map(mc, vec![(key1, val1)]);
            let right = Value::map(mc, vec![(key2, val2)]);
            let result = op_add(mc, left, right);
            if let Value::Map(m) = result {
                assert_eq!(m.len(), 2);
            } else {
                panic!("Expected map");
            }
        });
    }

    #[test]
    fn test_op_sub_numbers() {
        with_arena(|mc, _| {
            let left = Value::number(10.0);
            let right = Value::number(3.0);
            let result = op_sub(mc, left, right);
            assert!(matches!(result, Value::Number(7.0)));
        });
    }

    #[test]
    fn test_op_mul_numbers() {
        with_arena(|mc, _| {
            let left = Value::number(4.0);
            let right = Value::number(3.0);
            let result = op_mul(mc, left, right);
            assert!(matches!(result, Value::Number(12.0)));
        });
    }

    #[test]
    fn test_op_div_numbers() {
        with_arena(|mc, _| {
            let left = Value::number(10.0);
            let right = Value::number(2.0);
            let result = op_div(mc, left, right);
            assert!(matches!(result, Value::Number(5.0)));
        });
    }

    #[test]
    fn test_op_div_by_zero() {
        with_arena(|mc, _| {
            let left = Value::number(10.0);
            let right = Value::number(0.0);
            let result = op_div(mc, left, right);
            assert!(matches!(result, Value::Undefined));
        });
    }

    #[test]
    fn test_op_mod_numbers() {
        with_arena(|mc, _| {
            let left = Value::number(10.0);
            let right = Value::number(3.0);
            let result = op_mod(mc, left, right);
            assert!(matches!(result, Value::Number(1.0)));
        });
    }

    #[test]
    fn test_op_mod_by_zero() {
        with_arena(|mc, _| {
            let left = Value::number(10.0);
            let right = Value::number(0.0);
            let result = op_mod(mc, left, right);
            assert!(matches!(result, Value::Undefined));
        });
    }

    #[test]
    fn test_op_neg() {
        with_arena(|mc, _| {
            let left = Value::number(5.0);
            let right = Value::number(0.0);
            let result = op_neg(mc, left, right);
            assert!(matches!(result, Value::Number(-5.0)));
        });
    }

    #[test]
    fn test_op_eq_numbers() {
        with_arena(|mc, _| {
            let left = Value::number(5.0);
            let right = Value::number(5.0);
            let result = op_eq(mc, left, right);
            assert!(matches!(result, Value::Boolean(true)));
        });
    }

    #[test]
    fn test_op_eq_strings() {
        with_arena(|mc, _| {
            let left = Value::string(mc, "a".to_string());
            let right = Value::string(mc, "a".to_string());
            let result = op_eq(mc, left, right);
            assert!(matches!(result, Value::Boolean(true)));
        });
    }

    #[test]
    fn test_op_ne() {
        with_arena(|mc, _| {
            let left = Value::number(5.0);
            let right = Value::number(3.0);
            let result = op_ne(mc, left, right);
            assert!(matches!(result, Value::Boolean(true)));
        });
    }

    #[test]
    fn test_op_lt_numbers() {
        with_arena(|mc, _| {
            let left = Value::number(3.0);
            let right = Value::number(5.0);
            let result = op_lt(mc, left, right);
            assert!(matches!(result, Value::Boolean(true)));
        });
    }

    #[test]
    fn test_op_lt_strings() {
        with_arena(|mc, _| {
            let left = Value::string(mc, "a".to_string());
            let right = Value::string(mc, "b".to_string());
            let result = op_lt(mc, left, right);
            assert!(matches!(result, Value::Boolean(true)));
        });
    }

    #[test]
    fn test_op_le() {
        with_arena(|mc, _| {
            let left = Value::number(3.0);
            let right = Value::number(3.0);
            let result = op_le(mc, left, right);
            assert!(matches!(result, Value::Boolean(true)));
        });
    }

    #[test]
    fn test_op_gt() {
        with_arena(|mc, _| {
            let left = Value::number(5.0);
            let right = Value::number(3.0);
            let result = op_gt(mc, left, right);
            assert!(matches!(result, Value::Boolean(true)));
        });
    }

    #[test]
    fn test_op_ge() {
        with_arena(|mc, _| {
            let left = Value::number(5.0);
            let right = Value::number(5.0);
            let result = op_ge(mc, left, right);
            assert!(matches!(result, Value::Boolean(true)));
        });
    }

    #[test]
    fn test_op_not() {
        with_arena(|mc, _| {
            let left = Value::boolean(mc, true);
            let right = Value::undefined();
            let result = op_not(mc, left, right);
            assert!(matches!(result, Value::Boolean(false)));
        });
    }

    #[test]
    fn test_op_and() {
        with_arena(|mc, _| {
            let left = Value::boolean(mc, true);
            let right = Value::boolean(mc, false);
            let result = op_and(mc, left, right);
            assert!(matches!(result, Value::Boolean(false)));
        });
    }

    #[test]
    fn test_op_or() {
        with_arena(|mc, _| {
            let left = Value::boolean(mc, true);
            let right = Value::boolean(mc, false);
            let result = op_or(mc, left, right);
            assert!(matches!(result, Value::Boolean(true)));
        });
    }

    #[test]
    fn test_op_size_string() {
        with_arena(|mc, _| {
            let left = Value::string(mc, "hello".to_string());
            let right = Value::undefined();
            let result = op_size(mc, left, right);
            assert!(matches!(result, Value::Number(5.0)));
        });
    }

    #[test]
    fn test_op_size_list() {
        with_arena(|mc, _| {
            let left = Value::list(mc, vec![
                Value::number(1.0),
                Value::number(2.0),
                Value::number(3.0),
            ]);
            let right = Value::undefined();
            let result = op_size(mc, left, right);
            assert!(matches!(result, Value::Number(3.0)));
        });
    }

    #[test]
    fn test_op_size_map() {
        with_arena(|mc, _| {
            let key1 = Value::string(mc, "a".to_string());
            let val1 = Value::number(1.0);
            let key2 = Value::string(mc, "b".to_string());
            let val2 = Value::number(2.0);
            let left = Value::map(mc, vec![(key1, val1), (key2, val2)]);
            let right = Value::undefined();
            let result = op_size(mc, left, right);
            assert!(matches!(result, Value::Number(2.0)));
        });
    }

    #[test]
    fn test_op_get_map() {
        with_arena(|mc, _| {
            let key = Value::string(mc, "a".to_string());
            let val = Value::number(1.0);
            let left = Value::map(mc, vec![(key, val)]);
            let key2 = Value::string(mc, "a".to_string());
            let result = op_get(mc, left, key2);
            assert!(matches!(result, Value::Number(1.0)));
        });
    }

    #[test]
    fn test_op_get_list() {
        with_arena(|mc, _| {
            let left = Value::list(mc, vec![
                Value::number(10.0),
                Value::number(20.0),
                Value::number(30.0),
            ]);
            let idx = Value::number(1.0);
            let result = op_get(mc, left, idx);
            assert!(matches!(result, Value::Number(20.0)));
        });
    }

    #[test]
    fn test_op_get_list_out_of_bounds() {
        with_arena(|mc, _| {
            let left = Value::list(mc, vec![Value::number(10.0), Value::number(20.0)]);
            let idx = Value::number(10.0);
            let result = op_get(mc, left, idx);
            assert!(matches!(result, Value::Undefined));
        });
    }

    #[test]
    fn test_op_get_multiple() {
        with_arena(|mc, _| {
            let key1 = Value::string(mc, "a".to_string());
            let val1 = Value::number(1.0);
            let key2 = Value::string(mc, "b".to_string());
            let val2 = Value::number(2.0);
            let key3 = Value::string(mc, "c".to_string());
            let val3 = Value::number(3.0);
            let left = Value::map(mc, vec![(key1, val1), (key2, val2), (key3, val3)]);
            let keys = Value::list(mc, vec![
                Value::string(mc, "a".to_string()),
                Value::string(mc, "c".to_string()),
            ]);
            let result = op_get_multiple(mc, left, keys);
            if let Value::Map(m) = result {
                assert_eq!(m.len(), 2);
            } else {
                panic!("Expected map");
            }
        });
    }

    #[test]
    fn test_op_omit() {
        with_arena(|mc, _| {
            let key1 = Value::string(mc, "a".to_string());
            let val1 = Value::number(1.0);
            let key2 = Value::string(mc, "b".to_string());
            let val2 = Value::number(2.0);
            let key3 = Value::string(mc, "c".to_string());
            let val3 = Value::number(3.0);
            let left = Value::map(mc, vec![(key1, val1), (key2, val2), (key3, val3)]);
            let keys = Value::list(mc, vec![Value::string(mc, "b".to_string())]);
            let result = op_omit(mc, left, keys);
            if let Value::Map(m) = result {
                assert_eq!(m.len(), 2);
            } else {
                panic!("Expected map");
            }
        });
    }

    #[test]
    fn test_op_to_string() {
        with_arena(|mc, _| {
            let left = Value::number(42.0);
            let right = Value::undefined();
            let result = op_to_string(mc, left, right);
            if let Value::String(s) = result {
                assert_eq!(&*s, "42");
            } else {
                panic!("Expected string");
            }
        });
    }

    #[test]
    fn test_op_to_lowercase() {
        with_arena(|mc, _| {
            let left = Value::string(mc, "HELLO".to_string());
            let right = Value::undefined();
            let result = op_to_lowercase(mc, left, right);
            if let Value::String(s) = result {
                assert_eq!(&*s, "hello");
            } else {
                panic!("Expected string");
            }
        });
    }

    #[test]
    fn test_op_to_uppercase() {
        with_arena(|mc, _| {
            let left = Value::string(mc, "hello".to_string());
            let right = Value::undefined();
            let result = op_to_uppercase(mc, left, right);
            if let Value::String(s) = result {
                assert_eq!(&*s, "HELLO");
            } else {
                panic!("Expected string");
            }
        });
    }

    #[test]
    fn test_op_reverse_args() {
        with_arena(|mc, _| {
            let left = Value::undefined();
            let right = Value::undefined();
            let result = op_reverse_args(mc, left, right);
            assert!(matches!(result, Value::Undefined));
        });
    }

    #[test]
    fn test_loop_operators_return_undefined() {
        with_arena(|mc, _| {
            let left = Value::undefined();
            let right = Value::undefined();
            assert!(matches!(op_loop_map(mc, left, right), Value::Undefined));
            assert!(matches!(op_loop_filter(mc, left, right), Value::Undefined));
            assert!(matches!(op_loop_find(mc, left, right), Value::Undefined));
            assert!(matches!(op_loop_every(mc, left, right), Value::Undefined));
            assert!(matches!(op_loop_some(mc, left, right), Value::Undefined));
            assert!(matches!(op_loop_unique(mc, left, right), Value::Undefined));
            assert!(matches!(op_loop_sort(mc, left, right), Value::Undefined));
        });
    }
}