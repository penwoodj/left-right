use gc_arena::Mutation;
use lr_vm::Value;

pub type OperatorFn = for<'a> fn(&'a Mutation<'a>, Value<'a>, Value<'a>) -> Value<'a>;

pub struct OperatorRegistry {
    operators: Vec<(&'static str, OperatorFn)>,
}

impl OperatorRegistry {
    pub fn new() -> Self {
        Self {
            operators: vec![
                ("+", crate::operators::op_add),
                ("-", crate::operators::op_sub),
                ("*", crate::operators::op_mul),
                ("/", crate::operators::op_div),
                ("%", crate::operators::op_mod),
                ("neg", crate::operators::op_neg),
                ("=", crate::operators::op_eq),
                ("!=", crate::operators::op_ne),
                ("<", crate::operators::op_lt),
                ("<=", crate::operators::op_le),
                (">", crate::operators::op_gt),
                (">=", crate::operators::op_ge),
                ("!", crate::operators::op_not),
                ("&", crate::operators::op_and),
                ("|", crate::operators::op_or),
                ("@", crate::operators::op_get),
                ("@&", crate::operators::op_get_multiple),
                ("@-", crate::operators::op_omit),
                ("#", crate::operators::op_size),
                (".", crate::operators::op_reverse_args),
                ("\"", crate::operators::op_to_string),
                ("\"_", crate::operators::op_to_lowercase),
                ("\"^", crate::operators::op_to_uppercase),
                ("$", crate::operators::op_loop_map),
                ("$@", crate::operators::op_loop_filter),
                ("$?", crate::operators::op_loop_find),
                ("$_", crate::operators::op_loop_every),
                ("$|", crate::operators::op_loop_some),
                ("$~", crate::operators::op_loop_unique),
                ("$<", crate::operators::op_loop_sort),
            ],
        }
    }

    pub fn get_fn(&self, name: &str) -> Option<OperatorFn> {
        self.operators.iter().find(|(n, _)| *n == name).map(|(_, f)| *f)
    }

    pub fn get(&self, name: &str) -> Option<(&'static str, OperatorFn)> {
        self.operators.iter().find(|(n, _)| *n == name).copied()
    }

    pub fn operator_names(&self) -> Vec<&'static str> {
        self.operators.iter().map(|(name, _)| *name).collect()
    }
}

impl Default for OperatorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_lookup_fn() {
        let registry = OperatorRegistry::new();
        assert!(registry.get_fn("+").is_some());
        assert!(registry.get_fn("nonexistent").is_none());
    }

    #[test]
    fn test_registry_names() {
        let registry = OperatorRegistry::new();
        let names = registry.operator_names();
        let expected = [
            "+", "-", "*", "/", "%", "neg",
            "=", "!=", "<", "<=", ">", ">=",
            "!", "&", "|",
            "@", "@&", "@-", "#", ".",
            "\"", "\"_", "\"^",
            "$", "$@", "$?", "$_", "$|", "$~", "$<",
        ];
        for name in expected {
            assert!(names.contains(&name), "Missing operator: {}", name);
        }
    }

    #[test]
    fn test_registry_call_nonexistent() {
        let registry = OperatorRegistry::new();
        assert!(registry.get_fn("nonexistent").is_none());
    }
}
