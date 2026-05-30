use crate::value::{ClosureData, PartialClosureData, Value};
use gc_arena::{Collect, Gc, Mutation, Rootable};
use lr_bytecode::{Chunk, Constant, Instruction, Opcode};
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
struct CatchFrame {
    handler_start: usize,
    #[allow(dead_code)]
    end_label: usize,
}

#[derive(Debug, Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct VMRoot<'gc> {
    _marker: PhantomData<&'gc ()>,
}

pub struct Frame<'gc> {
    registers: Vec<Value<'gc>>,
    bindings: HashMap<String, Value<'gc>>,
    args: [Value<'gc>; 2],
    pc: usize,
    catch_stack: Vec<CatchFrame>,
    thrown_value: Option<Value<'gc>>,
}

impl<'gc> Frame<'gc> {
    pub fn new() -> Self {
        Self {
            registers: vec![Value::undefined(); 256],
            bindings: HashMap::new(),
            args: [Value::undefined(), Value::undefined()],
            pc: 0,
            catch_stack: Vec::new(),
            thrown_value: None,
        }
    }

    pub fn get(&self, reg: u8) -> Value<'gc> {
        self.registers[reg as usize]
    }

    pub fn set(&mut self, reg: u8, value: Value<'gc>) {
        self.registers[reg as usize] = value;
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn advance(&mut self) {
        self.pc += 1;
    }

    pub fn jump(&mut self, offset: i16) {
        self.pc = (self.pc as i64 + offset as i64) as usize;
    }

    pub fn set_arg(&mut self, index: usize, value: Value<'gc>) {
        if index < 2 { self.args[index] = value; }
    }

    pub fn get_arg(&self, index: usize) -> Value<'gc> {
        if index < 2 { self.args[index] } else { Value::undefined() }
    }
}

impl<'gc> Default for Frame<'gc> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct VM {
    arena: gc_arena::Arena<Rootable![VMRoot<'_>]>,
}

impl VM {
    fn find_closure_return(code: &[Instruction], body_start: usize) -> usize {
        let mut pc = body_start;
        while pc < code.len() {
            match code[pc].opcode() {
                Opcode::MakeClosure => {
                    let nested_body = (code[pc].b() as usize) | ((code[pc].c() as usize) << 8);
                    pc = Self::find_closure_return(code, nested_body) + 1;
                }
                Opcode::Return => return pc,
                _ => pc += 1,
            }
        }
        pc
    }

    pub fn new() -> Self {
        Self {
            arena: gc_arena::Arena::<Rootable![VMRoot<'_>]>::new(|_| VMRoot {
                _marker: PhantomData,
            }),
        }
    }

    pub fn execute(&mut self, chunk: &Chunk) -> Result<String, VMError> {
        self.arena.mutate(|mc, _root| {
            let mut frame = Frame::new();
            let code = &chunk.code;
            let constants = &chunk.constants;

            let result = self.run_dispatch(mc, &mut frame, code, constants)?;
            Ok(result.to_string())
        })
    }

    fn call_map_function<'a>(
        &self,
        mc: &Mutation<'a>,
        entries: Gc<'a, Vec<(Value<'a>, Value<'a>)>>,
        arg: Value<'a>,
        _parent_frame: &Frame<'a>,
    ) -> Result<Value<'a>, VMError> {
        let mut result = Value::undefined();
        let mut result_map = Vec::new();

        for (key, value) in entries.iter() {
            let resolved_value = if let Value::String(s) = value {
                if s.as_str() == "_<" {
                    arg
                } else if s.as_str() == "_>" {
                    Value::undefined()
                } else {
                    *value
                }
            } else {
                *value
            };
            
            if let Value::String(k) = key {
                if k.as_str() != "_<" && k.as_str() != "_>" {
                    result_map.push((*key, resolved_value));
                }
            } else {
                result_map.push((*key, resolved_value));
            }
            
            result = resolved_value;
        }

        if !result_map.is_empty() {
            Ok(Value::map(mc, result_map))
        } else {
            Ok(result)
        }
    }

    fn run_closure_body<'a>(
        &self,
        mc: &Mutation<'a>,
        code: &[Instruction],
        constants: &[Constant],
        body_start: usize,
        arg_count: u8,
        arg: Value<'a>,
        right_arg: Option<Value<'a>>,
    ) -> Result<Value<'a>, VMError> {
        let mut closure_frame = Frame::new();
        if arg_count >= 1 {
            closure_frame.set_arg(0, arg);
        }
        if arg_count >= 2 {
            if let Some(ra) = right_arg {
                closure_frame.set_arg(1, ra);
            }
        }
        closure_frame.pc = body_start;
        self.run_dispatch(mc, &mut closure_frame, code, constants)
    }

    fn call_map_operator<'a>(
        &self,
        mc: &Mutation<'a>,
        entries: &Gc<'a, Vec<(Value<'a>, Value<'a>)>>,
        left: &Value<'a>,
        right: &Value<'a>,
    ) -> Result<Value<'a>, VMError> {
        match (left, right) {
            (Value::Map(_), Value::String(op_name)) => {
                match op_name.as_str() {
                    "@" => Ok(Value::partial_operator(mc, "@".to_string(), *left)),
                    "#" => Ok(Value::number(entries.len() as f64)),
                    "?" => Ok(Value::partial_operator(mc, "?".to_string(), *left)),
                    "!" => Ok(Value::boolean(mc, !left.is_truthy())),
                    "!!" => Ok(Value::partial_operator(mc, "!!".to_string(), *left)),
                    "-" => Ok(Value::partial_operator(mc, "-".to_string(), *left)),
                    "+" => Ok(Value::partial_operator(mc, "+".to_string(), *left)),
                    "==" | "=" => Ok(Value::partial_operator(mc, op_name.to_string(), *left)),
                    "!=" => Ok(Value::partial_operator(mc, "!=".to_string(), *left)),
                    "@&" => Ok(Value::partial_operator(mc, "@&".to_string(), *left)),
                    _ => Err(VMError::TypeError(format!(
                        "Unknown map operator: {}", op_name
                    ))),
                }
            }
            _ => Err(VMError::TypeError(format!(
                "Cannot call: left={} right={}",
                left.type_name(),
                right.type_name()
            ))),
        }
    }

    fn run_dispatch<'a>(
        &self,
        mc: &Mutation<'a>,
        frame: &mut Frame<'a>,
        code: &[Instruction],
        constants: &[Constant],
    ) -> Result<Value<'a>, VMError> {
        while frame.pc() < code.len() {
            let inst = code[frame.pc()];
            let opcode = inst.opcode();

            match opcode {
                // Control flow
                Opcode::Nop => {
                    frame.advance();
                }
                Opcode::Return => {
                    let result = frame.get(inst.a());
                    return Ok(result);
                }
                Opcode::Jump => {
                    let offset = inst.a() as i8 as i16;
                    frame.jump(offset);
                }
                Opcode::JumpIfTrue => {
                    let cond = frame.get(inst.a());
                    let offset = inst.b() as i8 as i16;
                    if cond.is_truthy() {
                        frame.jump(offset);
                    } else {
                        frame.advance();
                    }
                }
                Opcode::JumpIfFalse => {
                    let cond = frame.get(inst.a());
                    let offset = inst.b() as i8 as i16;
                    if !cond.is_truthy() {
                        frame.jump(offset);
                    } else {
                        frame.advance();
                    }
                }
                Opcode::Call => {
                    let left = frame.get(inst.b());
                    let right = frame.get(inst.c());

                    // `!!!` throw operator: value !!! throws value as exception
                    if let Value::String(op) = &right {
                        if op.as_str() == "!!!" {
                            frame.thrown_value = Some(left);
                            if let Some(catch_frame) = frame.catch_stack.last().cloned() {
                                let thrown = frame.thrown_value.take();
                                frame.set(inst.a(), thrown.unwrap_or(Value::undefined()));
                                frame.pc = catch_frame.handler_start;
                                continue;
                            } else {
                                let value_str = left.to_string();
                                return Err(VMError::Runtime(format!("Uncaught exception: {}", value_str)));
                            }
                        }
                    }

                    // Error[expr] constructor
                    if let Value::String(name) = &left {
                        if name.as_str() == "Error" {
                            if let Value::List(args) = &right {
                                if !args.is_empty() {
                                    let message = args[0];
                                    frame.set(inst.a(), Value::error(mc, message));
                                    frame.advance();
                                    continue;
                                }
                            }
                        }
                    }

                    let result = match (&left, &right) {
                        (Value::Closure(_closure_data), _arg) => {
                            return Err(VMError::TypeError(
                                "Closures must be used in infix position (data first): `5 { _< + 1 }`, not `{ _< + 1 } 5`".to_string()
                            ));
                        }
                        (Value::PartialOperator(partial), Value::Closure(closure_data)) => {
                            // PartialOperator with Closure right — handle loop ops ($|, $&, $, $?, etc.)
                            match (&partial.left_arg, partial.name.as_str()) {
                                (Value::List(items), "$") => {
                                    let mut results = Vec::with_capacity(items.len());
                                    for item in items.iter() {
                                        let mapped = self.run_closure_body(mc, code, constants, closure_data.body_start, closure_data.arg_count, *item, None)?;
                                        results.push(mapped);
                                    }
                                    Value::list(mc, results)
                                }
                                (Value::List(items), "$?") => {
                                    let mut results = Vec::with_capacity(items.len());
                                    for item in items.iter() {
                                        let check = self.run_closure_body(mc, code, constants, closure_data.body_start, closure_data.arg_count, *item, None)?;
                                        if check.is_truthy() {
                                            results.push(*item);
                                        }
                                    }
                                    Value::list(mc, results)
                                }
                                (Value::List(items), "$_") => {
                                    let mut results = Vec::with_capacity(items.len());
                                    for item in items.iter() {
                                        let mapped = self.run_closure_body(mc, code, constants, closure_data.body_start, closure_data.arg_count, *item, None)?;
                                        match mapped {
                                            Value::List(sub_items) => results.extend(sub_items.iter().copied()),
                                            other => results.push(other),
                                        }
                                    }
                                    Value::list(mc, results)
                                }
                                (Value::List(items), "$|") => {
                                    let mut result = Value::boolean(mc, false);
                                    for item in items.iter() {
                                        let check = self.run_closure_body(mc, code, constants, closure_data.body_start, closure_data.arg_count, *item, None)?;
                                        if check.is_truthy() {
                                            result = Value::boolean(mc, true);
                                            break;
                                        }
                                    }
                                    result
                                }
                                (Value::List(items), "$&") => {
                                    let mut result = Value::boolean(mc, true);
                                    for item in items.iter() {
                                        let check = self.run_closure_body(mc, code, constants, closure_data.body_start, closure_data.arg_count, *item, None)?;
                                        if !check.is_truthy() {
                                            result = Value::boolean(mc, false);
                                            break;
                                        }
                                    }
                                    result
                                }
                                (Value::List(items), "$?|") => {
                                    for item in items.iter() {
                                        let check = self.run_closure_body(mc, code, constants, closure_data.body_start, closure_data.arg_count, *item, None)?;
                                        if check.is_truthy() {
                                            return Ok(*item);
                                        }
                                    }
                                    Value::undefined()
                                }
                                (Value::List(items), "$~") => {
                                    use std::collections::HashSet;
                                    let mut seen = HashSet::new();
                                    let mut result = Vec::new();
                                    for item in items.iter() {
                                        let key = self.run_closure_body(mc, code, constants, closure_data.body_start, closure_data.arg_count, *item, None)?;
                                        let key_str = key.to_string();
                                        if seen.insert(key_str) {
                                            result.push(*item);
                                        }
                                    }
                                    Value::list(mc, result)
                                }
                                (Value::List(items), "$>") => {
                                    use std::collections::HashMap;
                                    let mut groups: HashMap<String, Vec<Value<'a>>> = HashMap::new();
                                    for item in items.iter() {
                                        let key = self.run_closure_body(mc, code, constants, closure_data.body_start, closure_data.arg_count, *item, None)?;
                                        let key_str = key.to_string();
                                        groups.entry(key_str).or_insert_with(Vec::new).push(*item);
                                    }
                                    let result: Vec<(Value<'a>, Value<'a>)> = groups.into_iter().map(|(k, v)| {
                                        (Value::string(mc, k), Value::list(mc, v))
                                    }).collect();
                                    Value::map(mc, result)
                                }
                                (Value::List(items), "$%") => {
                                    let mut pairs: Vec<(Value<'a>, Value<'a>)> = Vec::new();
                                    for item in items.iter() {
                                        let key = self.run_closure_body(mc, code, constants, closure_data.body_start, closure_data.arg_count, *item, None)?;
                                        pairs.push((key, *item));
                                    }
                                    pairs.sort_by(|a, b| {
                                        match (&a.0, &b.0) {
                                            (Value::Number(an), Value::Number(bn)) => an.partial_cmp(bn).unwrap_or(std::cmp::Ordering::Equal),
                                            (Value::String(as_), Value::String(bs)) => as_.cmp(bs),
                                            _ => std::cmp::Ordering::Equal,
                                        }
                                    });
                                    let result: Vec<Value<'a>> = pairs.into_iter().map(|(_, item)| item).collect();
                                    Value::list(mc, result)
                                }
                                (_, "!!") => {
                                    if partial.left_arg.is_truthy() {
                                        self.run_closure_body(mc, code, constants, closure_data.body_start, closure_data.arg_count, partial.left_arg, None)?
                                    } else {
                                        Value::undefined()
                                    }
                                }
                                _ => return Err(VMError::TypeError(format!(
                                    "Cannot apply partial operator {} to closure", partial.name
                                ))),
                            }
                        }
                        (Value::List(args), Value::Closure(closure_data)) => {
                            match args.len() {
                                0 => Value::Closure(*closure_data),
                                1 => {
                                    if closure_data.arg_count >= 2 {
                                        Value::PartialClosure(Gc::new(mc, PartialClosureData {
                                            body_start: closure_data.body_start,
                                            arg_count: closure_data.arg_count,
                                            bound_left: args[0],
                                        }))
                                    } else {
                                        self.run_closure_body(mc, code, constants, closure_data.body_start, closure_data.arg_count, args[0], None)?
                                    }
                                }
                                2 => {
                                    self.run_closure_body(mc, code, constants, closure_data.body_start, closure_data.arg_count, args[0], Some(args[1]))?
                                }
                                _ => return Err(VMError::TypeError(format!(
                                    "Cannot apply {} args to closure (max 2)", args.len()
                                ))),
                            }
                        }
                        (arg, Value::Closure(closure_data)) => {
                            if closure_data.arg_count >= 2 {
                                // Diadic closure: left arg is bound, awaiting right arg
                                Value::PartialClosure(Gc::new(mc, PartialClosureData {
                                    body_start: closure_data.body_start,
                                    arg_count: closure_data.arg_count,
                                    bound_left: *arg,
                                }))
                            } else {
                                self.run_closure_body(mc, code, constants, closure_data.body_start, closure_data.arg_count, *arg, None)?
                            }
                        }
                        (Value::PartialClosure(pc), right_arg) => {
                            // Execute body with both args: bound_left as arg[0], right_arg as arg[1]
                            self.run_closure_body(mc, code, constants, pc.body_start, pc.arg_count, pc.bound_left, Some(*right_arg))?
                        }
                        (Value::Map(entries), _) => {
                            if entries.iter().any(|(k, _)| {
                                if let Value::String(s) = k { s.as_str() == "_<" || s.as_str() == "_>" } else { false }
                            }) {
                                self.call_map_function(mc, entries.clone(), right, frame)?
                            } else {
                                self.call_map_operator(mc, entries, &left, &right)?
                            }
                        }
                        (Value::String(s), Value::String(op_name)) => {
                            match op_name.as_str() {
                                "+" | "<>" | "><" | "~" | "==" | "=" | "!=" | "?><" => Value::partial_operator(mc, op_name.to_string(), Value::string(mc, s.to_string())),
                                "#" => Value::number(s.len() as f64),
                                "-" => Value::partial_operator(mc, "-".to_string(), Value::string(mc, s.to_string())),
                                "_" => Value::string(mc, s.to_lowercase()),
                                "?" => Value::partial_operator(mc, "?".to_string(), Value::string(mc, s.to_string())),
                                "!" => Value::boolean(mc, !Value::string(mc, s.to_string()).is_truthy()),
                                "|" => Value::partial_operator(mc, "|".to_string(), Value::string(mc, s.to_string())),
                                "!!" => Value::partial_operator(mc, "!!".to_string(), Value::string(mc, s.to_string())),
                                "^" => Value::string(mc, s.to_uppercase()),
                                "^_" => {
                                    let mut chars = s.chars();
                                    match chars.next() {
                                        None => Value::string(mc, s.to_string()),
                                        Some(first) => Value::string(mc,
                                            format!("{}{}", first.to_uppercase(), chars.as_str().to_lowercase())),
                                    }
                                }
                                "?\"" => Value::boolean(mc, true),
                                "?#" => Value::boolean(mc, false),
                                "/json" => {
                                    match serde_json::from_str::<serde_json::Value>(&s) {
                                        Ok(json_val) => json_to_lr_value(mc, &json_val),
                                        Err(e) => return Err(VMError::Runtime(format!("JSON parse error: {}", e))),
                                    }
                                }
                                _ => {
                                    return Err(VMError::Runtime(format!(
                                        "Unknown operator for strings: {}",
                                        op_name
                                    )))
                                }
                            }
                        }
                        (Value::List(l), Value::String(op_name)) => {
                            match op_name.as_str() {
                                "@" => Value::partial_operator(mc, "@".to_string(), Value::List(*l)),
                                "#" => Value::number(l.len() as f64),
                                "?" => Value::partial_operator(mc, "?".to_string(), Value::List(*l)),
                                "!" => Value::boolean(mc, !Value::List(*l).is_truthy()),
                                "!!" => Value::partial_operator(mc, "!!".to_string(), Value::List(*l)),
                                "|" => Value::partial_operator(mc, "|".to_string(), Value::List(*l)),
                                "+" | "_" => Value::partial_operator(mc, op_name.to_string(), Value::list(mc, l.as_ref().clone())),
                                "-" => Value::partial_operator(mc, "-".to_string(), Value::list(mc, l.as_ref().clone())),
                                "<>" | "><" => Value::partial_operator(mc, op_name.to_string(), Value::list(mc, l.as_ref().clone())),
                                "?><" => Value::partial_operator(mc, "?><".to_string(), Value::list(mc, l.as_ref().clone())),
                                "==" | "=" => Value::partial_operator(mc, op_name.to_string(), Value::List(*l)),
                                "$" => Value::partial_operator(mc, "$".to_string(), Value::List(*l)),
                                "$?" => Value::partial_operator(mc, "$?".to_string(), Value::List(*l)),
                                "$_" => Value::partial_operator(mc, "$_".to_string(), Value::List(*l)),
                                "$|" => Value::partial_operator(mc, "$|".to_string(), Value::List(*l)),
                                "$&" => Value::partial_operator(mc, "$&".to_string(), Value::List(*l)),
                                "$?|" => Value::partial_operator(mc, "$?|".to_string(), Value::List(*l)),
                                "$~" => Value::partial_operator(mc, "$~".to_string(), Value::List(*l)),
                                "$>" => Value::partial_operator(mc, "$>".to_string(), Value::List(*l)),
                                "$%" => Value::partial_operator(mc, "$%".to_string(), Value::List(*l)),
                                "$@" => Value::partial_operator(mc, "$@".to_string(), Value::List(*l)),
                                "$+" | "$-" | "$*" | "$/" => Value::partial_operator(mc, op_name.to_string(), Value::List(*l)),
                                "$?>" | "$?<" | "$?>=" | "$?<=" | "$?+" | "$?-" => Value::partial_operator(mc, op_name.to_string(), Value::List(*l)),
                                "$?!" => {
                                    let filtered: Vec<Value<'a>> = l.iter().filter(|v| v.is_truthy()).copied().collect();
                                    Value::list(mc, filtered)
                                }
                                "$\"" => {
                                    let strings: Vec<Value<'a>> = l.iter().map(|v| {
                                        match v {
                                            Value::String(s) => Value::string(mc, s.to_string()),
                                            Value::Number(n) => {
                                                if n.fract() == 0.0 {
                                                    Value::string(mc, (*n as i64).to_string())
                                                } else {
                                                    Value::string(mc, n.to_string())
                                                }
                                            }
                                            Value::Boolean(b) => Value::string(mc, b.to_string()),
                                            _ => Value::string(mc, format!("{}", v)),
                                        }
                                    }).collect();
                                    Value::list(mc, strings)
                                }
                                _ => return Err(VMError::TypeError(format!(
                                    "Unknown list operator: {}", op_name
                                ))),
                            }
                        }
                        (Value::Number(n), Value::String(op_name)) => {
                            match op_name.as_str() {
                                "+" => Value::partial_operator(mc, "+".to_string(), Value::Number(*n)),
                                "-" => Value::partial_operator(mc, "-".to_string(), Value::Number(*n)),
                                "*" => Value::partial_operator(mc, "*".to_string(), Value::Number(*n)),
                                "/" => Value::partial_operator(mc, "/".to_string(), Value::Number(*n)),
                                "%" => Value::partial_operator(mc, "%".to_string(), Value::Number(*n)),
                                "^" => Value::partial_operator(mc, "^".to_string(), Value::Number(*n)),
                                "==" | "=" => Value::partial_operator(mc, op_name.to_string(), Value::Number(*n)),
                                "!=" => Value::partial_operator(mc, "!=".to_string(), Value::Number(*n)),
                                "<" => Value::partial_operator(mc, "<".to_string(), Value::Number(*n)),
                                ">" => Value::partial_operator(mc, ">".to_string(), Value::Number(*n)),
                                "<=" => Value::partial_operator(mc, "<=".to_string(), Value::Number(*n)),
                                ">=" => Value::partial_operator(mc, ">=".to_string(), Value::Number(*n)),
                                "&" => Value::partial_operator(mc, "&".to_string(), Value::Number(*n)),
                                "|" => Value::partial_operator(mc, "|".to_string(), Value::Number(*n)),
                                "?" => Value::partial_operator(mc, "?".to_string(), Value::Number(*n)),
                                "!" => Value::boolean(mc, !Value::Number(*n).is_truthy()),
                                "!!" => Value::partial_operator(mc, "!!".to_string(), Value::Number(*n)),
                                "?\"" => Value::boolean(mc, false),
                                "?#" => Value::boolean(mc, true),
                                _ => {
                                    return Err(VMError::Runtime(format!(
                                        "Unknown operator: {}",
                                        op_name
                                    )))
                                }
                            }
                        }
                        (Value::String(op_name), _) => {
                            match op_name.as_str() {
                                "!" => Value::boolean(mc, !right.is_truthy()),
                                "?" => Value::partial_operator(mc, "?".to_string(), right),
                                "?\"" => Value::boolean(mc, matches!(right, Value::String(_))),
                                "?#" => Value::boolean(mc, matches!(right, Value::Number(_))),
                                "!!" => Value::partial_operator(mc, "!!".to_string(), right),
                                _ => return Err(VMError::TypeError(format!(
                                    "Cannot call: left=string right={}", right.type_name()
                                ))),
                            }
                        }
                        (Value::Boolean(b), Value::String(op_name)) => {
                            match op_name.as_str() {
                                "!" => Value::boolean(mc, !Value::boolean(mc, *b).is_truthy()),
                                "&" | "|" | "?" | "!!" => Value::partial_operator(mc, op_name.to_string(), Value::boolean(mc, *b)),
                                "?\"" => Value::boolean(mc, false), // booleans are not strings
                                "?#" => Value::boolean(mc, false), // booleans are not numbers
                                "==" | "!=" | "=" => Value::partial_operator(mc, op_name.to_string(), Value::boolean(mc, *b)),
                                _ => return Err(VMError::TypeError(format!(
                                    "Unknown boolean operator: {}", op_name
                                ))),
                            }
                        }
                        (Value::PartialOperator(partial), _) => {
                            match &right {
                                Value::Boolean(right_val) => {
                                    if let Value::Boolean(left_val) = &partial.left_arg {
                                        match partial.name.as_str() {
                                            "&" => Value::boolean(mc, *left_val && *right_val),
                                            "|" => if *left_val { partial.left_arg } else { right },
                                            "==" | "=" => Value::boolean(mc, *left_val == *right_val),
                                            "!=" => Value::boolean(mc, *left_val != *right_val),
                                            "?" => {
                                                if *left_val { right } else { partial.left_arg }
                                            }
                                            _ => return Err(VMError::Runtime(format!(
                                                "Unknown partial boolean operator: {}", partial.name
                                            ))),
                                        }
                                    } else {
                                        return Err(VMError::TypeError(format!(
                                            "Partial operator {} left arg is not a boolean", partial.name
                                        )));
                                    }
                                }
                                _ => {
                                    match (&partial.left_arg, &right, partial.name.as_str()) {
                                        (Value::Number(lv), Value::Number(rv), op) => {
                                            match op {
                                                "+" => Value::number(*lv + *rv),
                                                "-" => Value::number(*lv - *rv),
                                                "*" => Value::number(*lv * *rv),
                                                "/" => {
                                                    if *rv == 0.0 { return Err(VMError::Runtime("Division by zero".to_string())); }
                                                    Value::number(*lv / *rv)
                                                }
                                                "%" => {
                                                    if *rv == 0.0 { return Err(VMError::Runtime("Division by zero".to_string())); }
                                                    Value::number(*lv % *rv)
                                                }
                                                "^" => Value::number(lv.powf(*rv)),
                                                "==" | "=" => Value::boolean(mc, lv == rv),
                                                "!=" => Value::boolean(mc, lv != rv),
                                                "<" => Value::boolean(mc, lv < rv),
                                                ">" => Value::boolean(mc, lv > rv),
                                                "<=" => Value::boolean(mc, lv <= rv),
                                                ">=" => Value::boolean(mc, lv >= rv),
                                                "&" => Value::boolean(mc, partial.left_arg.is_truthy() && right.is_truthy()),
                                                "|" => if partial.left_arg.is_truthy() { partial.left_arg } else { right },
                                                _ => return Err(VMError::Runtime(format!("Unknown partial operator: {}", partial.name))),
                                            }
                                        }
                                        (Value::String(ls), Value::String(rs), "+" | "_") => {
                                            let combined = format!("{}{}", ls, rs);
                                            Value::string(mc, combined)
                                        }
                                        (Value::String(ls), Value::String(rs), "==" | "=") => {
                                            Value::boolean(mc, ls == rs)
                                        }
                                        (Value::String(ls), Value::String(rs), "!=") => {
                                            Value::boolean(mc, ls != rs)
                                        }
                                        (Value::String(ls), Value::Number(n), "+") => {
                                            let s = if n.fract() == 0.0 {
                                                (*n as i64).to_string()
                                            } else {
                                                n.to_string()
                                            };
                                            Value::string(mc, format!("{}{}", ls, s))
                                        }
                                        (Value::String(ls), Value::Boolean(b), "+") => {
                                            Value::string(mc, format!("{}{}", ls, b))
                                        }
                                        (Value::String(s), Value::List(args), "~") => {
                                            if args.len() >= 2 {
                                                if let (Value::String(old), Value::String(new)) = (&args[0], &args[1]) {
                                                    Value::string(mc, s.replace(old.as_str(), new.as_str()))
                                                } else {
                                                    return Err(VMError::Runtime("~ operator requires [old_string, new_string]".to_string()));
                                                }
                                            } else {
                                                return Err(VMError::Runtime("~ operator requires [old_string, new_string]".to_string()));
                                            }
                                        }
                                        (Value::String(ls), _, "^") => {
                                            Value::string(mc, ls.to_uppercase())
                                        }
                                        (Value::String(ls), _, "^_") => {
                                            let mut chars = ls.chars();
                                            match chars.next() {
                                                None => Value::string(mc, ls.to_string()),
                                                Some(first) => Value::string(mc, format!("{}{}", first.to_uppercase(), chars.as_str().to_lowercase())),
                                            }
                                        }
                                        (Value::String(ls), _, "_") => {
                                            Value::string(mc, ls.to_lowercase())
                                        }
                                        (Value::String(ls), Value::String(rs), "<>") => {
                                            let parts: Vec<Value<'a>> = ls.split(rs.as_str())
                                                .map(|s| Value::string(mc, s.to_string()))
                                                .collect();
                                            Value::list(mc, parts)
                                        }
                                        (Value::List(items), Value::String(sep), "<>") => {
                                            let joined: String = items.iter().enumerate().map(|(i, v)| {
                                                let s = match v {
                                                    Value::String(s) => s.to_string(),
                                                    Value::Number(n) => {
                                                        if n.fract() == 0.0 { (*n as i64).to_string() } else { n.to_string() }
                                                    }
                                                    Value::Boolean(b) => b.to_string(),
                                                    _ => format!("{}", v),
                                                };
                                                if i > 0 { format!("{}{}", sep, s) } else { s }
                                            }).collect();
                                            Value::string(mc, joined)
                                        }
                                        (Value::List(items), Value::String(sep), "><") => {
                                            let joined: String = items.iter().enumerate().map(|(i, v)| {
                                                let s = match v {
                                                    Value::String(s) => s.to_string(),
                                                    Value::Number(n) => {
                                                        if n.fract() == 0.0 { (*n as i64).to_string() } else { n.to_string() }
                                                    }
                                                    Value::Boolean(b) => b.to_string(),
                                                    _ => format!("{}", v),
                                                };
                                                if i > 0 { format!("{}{}", sep, s) } else { s }
                                            }).collect();
                                            Value::string(mc, joined)
                                        }
                                        // $@ map-each-property: pluck single key from each map in list
                                        (Value::List(items), Value::String(key), "$@") => {
                                            let results: Vec<Value<'a>> = items.iter().map(|item| {
                                                match item {
                                                    Value::Map(entries) => {
                                                        entries.iter()
                                                            .find(|(k, _)| {
                                                                if let Value::String(ks) = k { ks == key } else { false }
                                                            })
                                                            .map(|(_, v)| *v)
                                                            .unwrap_or(Value::undefined())
                                                    }
                                                    _ => Value::undefined(),
                                                }
                                            }).collect();
                                            Value::list(mc, results)
                                        }
                                        // $@ map-each-property: pluck multiple keys from each map in list
                                        (Value::List(items), Value::List(keys), "$@") => {
                                            let results: Vec<Value<'a>> = items.iter().map(|item| {
                                                match item {
                                                    Value::Map(entries) => {
                                                        let filtered: Vec<(Value<'a>, Value<'a>)> = keys.iter()
                                                            .filter_map(|key| {
                                                                if let Value::String(ks) = key {
                                                                    entries.iter()
                                                                        .find(|(k, _)| {
                                                                            if let Value::String(ek) = k { ek == ks } else { false }
                                                                        })
                                                                        .map(|(k, v)| (*k, *v))
                                                                } else {
                                                                    None
                                                                }
                                                            })
                                                            .collect();
                                                        Value::map(mc, filtered)
                                                    }
                                                    _ => Value::map(mc, vec![]),
                                                }
                                            }).collect();
                                            Value::list(mc, results)
                                        }
                                        (Value::Map(entries), Value::String(key), "@") => {
                                            entries.iter()
                                                .find(|(k, _)| {
                                                    if let Value::String(ks) = k { ks == key } else { false }
                                                })
                                                .map(|(_, v)| *v)
                                                .unwrap_or(Value::undefined())
                                        }
                                        (Value::Error(err_data), Value::String(key), "@") => {
                                            if key.as_str() == "message" {
                                                err_data.message
                                            } else {
                                                err_data.properties.iter()
                                                    .find(|(k, _)| {
                                                        if let Value::String(ks) = k { ks == key } else { false }
                                                    })
                                                    .map(|(_, v)| *v)
                                                    .unwrap_or(Value::undefined())
                                            }
                                        }
                                        (Value::Map(entries), Value::String(key), "-") => {
                                            let filtered: Vec<(Value<'a>, Value<'a>)> = entries.iter()
                                                .filter(|(k, _)| {
                                                    if let Value::String(ks) = k { ks != key } else { true }
                                                })
                                                .map(|(k, v)| (*k, *v))
                                                .collect();
                                            Value::map(mc, filtered)
                                        }
                                        (Value::Map(entries), Value::List(keys), "@") => {
                                            let mut current = Value::Map(*entries);
                                            for key in keys.iter() {
                                                current = match (&current, key) {
                                                    (Value::Map(map_entries), Value::String(key_name)) => {
                                                        map_entries.iter()
                                                            .find(|(k, _)| {
                                                                if let Value::String(ks) = k { ks == key_name } else { false }
                                                            })
                                                            .map(|(_, v)| *v)
                                                            .unwrap_or(Value::undefined())
                                                    }
                                                    (Value::List(items), Value::Number(idx)) => {
                                                        items.iter().nth(*idx as usize)
                                                            .copied()
                                                            .unwrap_or(Value::undefined())
                                                    }
                                                    _ => Value::undefined(),
                                                };
                                            }
                                            current
                                        }
                                        (Value::Map(entries), Value::Number(idx), "@") => {
                                            let pair = entries.iter().nth(*idx as usize);
                                            pair.map(|(k, v)| {
                                                let mut result = Vec::new();
                                                result.push(*k);
                                                result.push(*v);
                                                result
                                            }).map(|v| Value::list(mc, v))
                                            .unwrap_or(Value::undefined())
                                        }
                                        (Value::List(items), Value::Number(idx), "@") => {
                                            items.iter().nth(*idx as usize)
                                                .copied()
                                                .unwrap_or(Value::undefined())
                                        }
                                        (Value::List(items), Value::List(indices), "@") => {
                                            let mut current = Value::List(*items);
                                            for idx in indices.iter() {
                                                current = match (&current, idx) {
                                                    (Value::List(list_items), Value::Number(n)) => {
                                                        list_items.iter().nth(*n as usize)
                                                            .copied()
                                                            .unwrap_or(Value::undefined())
                                                    }
                                                    (Value::Map(map_entries), Value::String(key)) => {
                                                        map_entries.iter()
                                                            .find(|(k, _)| {
                                                                if let Value::String(ks) = k { ks == key } else { false }
                                                            })
                                                            .map(|(_, v)| *v)
                                                            .unwrap_or(Value::undefined())
                                                    }
                                                    _ => Value::undefined(),
                                                };
                                            }
                                            current
                                        }
                                        (Value::List(left_items), Value::List(right_items), "+") => {
                                            let mut combined = left_items.to_vec();
                                            combined.extend(right_items.iter());
                                            Value::list(mc, combined)
                                        }
                                        (Value::List(items), _, "+") => {
                                            let mut combined = items.to_vec();
                                            combined.push(right);
                                            Value::list(mc, combined)
                                        }
                                        (Value::Number(n), Value::List(items), "+") => {
                                            let mut combined = vec![Value::number(*n)];
                                            combined.extend(items.iter());
                                            Value::list(mc, combined)
                                        }
                                        (Value::List(items), Value::List(removals), "-") => {
                                            let filtered: Vec<Value<'a>> = items.iter()
                                                .filter(|item| !removals.iter().any(|r| {
                                                    match (item, r) {
                                                        (Value::Number(a), Value::Number(b)) => (a - b).abs() < f64::EPSILON,
                                                        (Value::String(a), Value::String(b)) => a == b,
                                                        (Value::Boolean(a), Value::Boolean(b)) => a == b,
                                                        _ => false,
                                                    }
                                                }))
                                                .copied()
                                                .collect();
                                            Value::list(mc, filtered)
                                        }
                                        (Value::List(items), value, "-") => {
                                            let filtered: Vec<Value<'a>> = items.iter()
                                                .filter(|item| {
                                                    match (item, &value) {
                                                        (Value::Number(a), Value::Number(b)) => (a - b).abs() >= f64::EPSILON,
                                                        (Value::String(a), Value::String(b)) => a != b,
                                                        (Value::Boolean(a), Value::Boolean(b)) => a != b,
                                                        _ => true,
                                                    }
                                                })
                                                .copied()
                                                .collect();
                                            Value::list(mc, filtered)
                                        }
                                        (Value::String(s), Value::String(remove), "-") => {
                                            Value::string(mc, s.replace(remove.as_str(), ""))
                                        }
                                        (Value::Map(left_entries), Value::Map(right_entries), "+") => {
                                            let mut merged = left_entries.to_vec();
                                            for (k, v) in right_entries.iter() {
                                                if let Some(pos) = merged.iter().position(|(mk, _)| {
                                                    if let (Value::String(a), Value::String(b)) = (mk, k) { a == b } else { false }
                                                }) {
                                                    merged[pos].1 = *v;
                                                } else {
                                                    merged.push((*k, *v));
                                                }
                                            }
                                            Value::map(mc, merged)
                                        }
                                        (Value::Number(n), Value::String(s), "+") => {
                                            let ns = if n.fract() == 0.0 { (*n as i64).to_string() } else { n.to_string() };
                                            Value::string(mc, format!("{}{}", ns, s))
                                        }
                                        (Value::Boolean(b), Value::String(s), "+") => {
                                            Value::string(mc, format!("{}{}", b, s))
                                        }
                                        (_, Value::String(s), "+") => {
                                            Value::string(mc, format!("{:?}{}", partial.left_arg, s))
                                        }
                                        (Value::List(left_items), Value::List(right_items), "==" | "=") => {
                                            if left_items.len() != right_items.len() {
                                                Value::boolean(mc, false)
                                            } else {
                                                let eq = left_items.iter().zip(right_items.iter())
                                                    .all(|(a, b)| match (a, b) {
                                                        (Value::Number(a), Value::Number(b)) => a == b,
                                                        (Value::Boolean(a), Value::Boolean(b)) => a == b,
                                                        (Value::String(a), Value::String(b)) => a == b,
                                                        (Value::Undefined, Value::Undefined) => true,
                                                        _ => false,
                                                    });
                                                Value::boolean(mc, eq)
                                            }
                                        }
                                        (Value::Map(left_entries), Value::Map(right_entries), "==" | "=") => {
                                            if left_entries.len() != right_entries.len() {
                                                Value::boolean(mc, false)
                                            } else {
                                                let eq = left_entries.iter().all(|(k, v)| {
                                                    right_entries.iter().any(|(rk, rv)| {
                                                        let keys_match = match (k, rk) {
                                                            (Value::String(a), Value::String(b)) => a == b,
                                                            _ => false,
                                                        };
                                                        if !keys_match { return false; }
                                                        match (v, rv) {
                                                            (Value::Number(a), Value::Number(b)) => a == b,
                                                            (Value::Boolean(a), Value::Boolean(b)) => a == b,
                                                            (Value::String(a), Value::String(b)) => a == b,
                                                            (Value::Undefined, Value::Undefined) => true,
                                                            _ => false,
                                                        }
                                                    })
                                                });
                                                Value::boolean(mc, eq)
                                            }
                                        }
                                        (Value::Map(left_entries), Value::Map(right_entries), "!=") => {
                                            if left_entries.len() != right_entries.len() {
                                                Value::boolean(mc, true)
                                            } else {
                                                let eq = left_entries.iter().all(|(k, v)| {
                                                    right_entries.iter().any(|(rk, rv)| {
                                                        let keys_match = match (k, rk) {
                                                            (Value::String(a), Value::String(b)) => a == b,
                                                            _ => false,
                                                        };
                                                        if !keys_match { return false; }
                                                        match (v, rv) {
                                                            (Value::Number(a), Value::Number(b)) => a == b,
                                                            (Value::Boolean(a), Value::Boolean(b)) => a == b,
                                                            (Value::String(a), Value::String(b)) => a == b,
                                                            (Value::Undefined, Value::Undefined) => true,
                                                            _ => false,
                                                        }
                                                    })
                                                });
                                                Value::boolean(mc, !eq)
                                            }
                                        }
                                        (Value::Map(entries), Value::List(keys), "@&") => {
                                            let picked: Vec<(Value<'a>, Value<'a>)> = entries.iter().filter(|(k, _)| {
                                                keys.iter().any(|key| match (k, key) {
                                                    (Value::String(a), Value::String(b)) => a == b,
                                                    _ => false,
                                                })
                                            }).map(|(k, v)| (*k, *v)).collect();
                                            Value::map(mc, picked)
                                        }
                                        (Value::List(items), value, "?><") => {
                                            let contains = items.iter().any(|item| item.deep_eq(&value));
                                            Value::boolean(mc, contains)
                                        }
                                        (Value::String(s), Value::String(sub), "?><") => {
                                            Value::boolean(mc, s.contains(sub.as_str()))
                                        }
                                        (Value::List(items), Value::String(map_op), "$") => {
                                            Value::partial_operator(mc, format!("${}", map_op), Value::List(*items))
                                        }
                                        (Value::List(items), Value::Number(n), op) if op.starts_with('$') && !op.starts_with("$?") => {
                                            let inner_op = &op[1..];
                                            let result: Vec<Value<'a>> = items.iter().map(|item| match (item, inner_op) {
                                                (Value::Number(v), "+") => Value::number(*v + *n),
                                                (Value::Number(v), "*") => Value::number(*v * *n),
                                                (Value::Number(v), "-") => Value::number(*v - *n),
                                                (Value::Number(v), "/") => {
                                                    if *n == 0.0 {
                                                        return Value::undefined();
                                                    }
                                                    Value::number(*v / *n)
                                                }
                                                (Value::Number(v), "%") => {
                                                    if *n == 0.0 {
                                                        return Value::undefined();
                                                    }
                                                    Value::number(*v % *n)
                                                }
                                                (_, _) => *item,
                                            }).collect();
                                            Value::list(mc, result)
                                        }
                                        (Value::List(items), Value::String(compare_op), "$?") => {
                                            Value::partial_operator(mc, format!("$?{}", compare_op), Value::List(*items))
                                        }
                                        (Value::List(items), Value::Number(n), op) if op.starts_with("$?") => {
                                            let compare_op = &op[2..];
                                            let result: Vec<Value<'a>> = items.iter().filter(|item| match (item, compare_op) {
                                                (Value::Number(v), ">") => *v > *n,
                                                (Value::Number(v), ">=") => *v >= *n,
                                                (Value::Number(v), "<") => *v < *n,
                                                (Value::Number(v), "<=") => *v <= *n,
                                                (Value::Number(v), "=" | "==") => *v == *n,
                                                (Value::Number(v), "!=") => *v != *n,
                                                (Value::Number(v), "+") => *v > *n,  // $?+ = filter where > n
                                                (Value::Number(v), "-") => *v < *n,  // $?- = filter where < n
                                                _ => false,
                                            }).copied().collect();
                                            Value::list(mc, result)
                                        }
                                        (left, _, "?") => {
                                            if left.is_truthy() {
                                                right
                                            } else {
                                                *left
                                            }
                                        }
                                        (left, _, "?\"") => {
                                            Value::boolean(mc, matches!(left, Value::String(_)))
                                        }
                                        (left, _, "?#") => {
                                            Value::boolean(mc, matches!(left, Value::Number(_)))
                                        }
                                        (_, _, "?:else") => {
                                            right
                                        }
                                        (_, _, "|") => {
                                            if partial.left_arg.is_truthy() {
                                                partial.left_arg
                                            } else {
                                                right
                                            }
                                        }
                                        _ => return Err(VMError::TypeError(format!(
                                            "Cannot apply partial operator {} to {}", partial.name, right.type_name()
                                        ))),
                                    }
                                }
                            }
                        }
                        (Value::Operator(op), _) => {
                            match &right {
                                Value::Number(n) => {
                                    match op.name.as_str() {
                                        "+" => Value::partial_operator(mc, "+".to_string(), Value::Number(*n)),
                                        "-" => Value::partial_operator(mc, "-".to_string(), Value::Number(*n)),
                                        "*" => Value::partial_operator(mc, "*".to_string(), Value::Number(*n)),
                                        "/" => Value::partial_operator(mc, "/".to_string(), Value::Number(*n)),
                                        "%" => Value::partial_operator(mc, "%".to_string(), Value::Number(*n)),
                                        "^" => Value::partial_operator(mc, "^".to_string(), Value::Number(*n)),
                                        "==" | "=" => Value::partial_operator(mc, op.name.to_string(), Value::Number(*n)),
                                        "!=" => Value::partial_operator(mc, "!=".to_string(), Value::Number(*n)),
                                        "<" => Value::partial_operator(mc, "<".to_string(), Value::Number(*n)),
                                        ">" => Value::partial_operator(mc, ">".to_string(), Value::Number(*n)),
                                        "<=" => Value::partial_operator(mc, "<=".to_string(), Value::Number(*n)),
                                        ">=" => Value::partial_operator(mc, ">=".to_string(), Value::Number(*n)),
                                        "&" => Value::partial_operator(mc, "&".to_string(), Value::Number(*n)),
                                        "|" => Value::partial_operator(mc, "|".to_string(), Value::Number(*n)),
                                        _ => {
                                            return Err(VMError::Runtime(format!(
                                                 "Unknown operator: {}",
                                                 op.name
                                             )))
                                         }
                                     }
                                 }
                                 _ => {
                                     return Err(VMError::TypeError(format!(
                                         "Cannot apply operator {} to {}",
                                         op.name,
                                         right.type_name()
                                     )))
                                 }
                             }
                         }
                          (Value::Undefined, Value::String(op_name)) => {
                              match op_name.as_str() {
                                  "|" => Value::partial_operator(mc, "|".to_string(), Value::undefined()),
                                  "?" => Value::partial_operator(mc, "?".to_string(), Value::undefined()),
                                  "!!" => Value::partial_operator(mc, "!!".to_string(), Value::undefined()),
                                  _ => return Err(VMError::TypeError(format!(
                                      "Cannot call: left=undefined right={}", op_name
                                  ))),
                              }
                          }
                          _ => {
                              if let Value::Error(_) = &left {
                                  if let Value::String(op) = &right {
                                      if op.as_str() == "@" {
                                          let po = Value::partial_operator(mc, "@".to_string(), left);
                                          frame.set(inst.a(), po);
                                          frame.advance();
                                          continue;
                                      }
                                  }
                              }
                              return Err(VMError::TypeError(format!(
                                  "Cannot call: left={} right={}",
                                  left.type_name(),
                                  right.type_name()
                              )))
                          }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::TailCall => {
                    frame.advance();
                }

                // Load/Store
                Opcode::LoadConstant => {
                    let const_idx = inst.c() as usize;
                    let constant = constants
                        .get(const_idx)
                        .ok_or(VMError::ConstantIndexOutOfBounds(const_idx))?;

                    let value = match constant {
                        Constant::Undefined => Value::undefined(),
                        Constant::Boolean(b) => Value::boolean(mc, *b),
                        Constant::Number(n) => Value::number(*n),
                        Constant::String(s) => Value::string(mc, s.clone()),
                    };

                    frame.set(inst.a(), value);
                    frame.advance();
                }
                Opcode::LoadRegister => {
                    let value = frame.get(inst.b());
                    frame.set(inst.a(), value);
                    frame.advance();
                }
                Opcode::StoreRegister => {
                    let value = frame.get(inst.a());
                    frame.set(inst.b(), value);
                    frame.advance();
                }
                Opcode::LoadLocal => {
                    let value = frame.get(inst.b());
                    frame.set(inst.a(), value);
                    frame.advance();
                }
                Opcode::StoreLocal => {
                    let value = frame.get(inst.a());
                    frame.set(inst.b(), value);
                    frame.advance();
                }

                // Arithmetic
                Opcode::Add => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    let result = match (&b, &c) {
                        (Value::Number(b), Value::Number(c)) => Value::number(*b + *c),
                        (Value::String(b), Value::String(c)) => {
                            let combined = format!("{}{}", b, c);
                            Value::string(mc, combined)
                        }
                        (Value::Number(n), Value::String(s)) => {
                            let ns = if n.fract() == 0.0 { (*n as i64).to_string() } else { n.to_string() };
                            Value::string(mc, format!("{}{}", ns, s))
                        }
                        (Value::String(s), Value::Number(n)) => {
                            let ns = if n.fract() == 0.0 { (*n as i64).to_string() } else { n.to_string() };
                            Value::string(mc, format!("{}{}", s, ns))
                        }
                        (Value::List(a), Value::List(b_list)) => {
                            let mut combined = a.to_vec();
                            combined.extend(b_list.iter());
                            Value::list(mc, combined)
                        }
                        (Value::Map(a), Value::Map(b_map)) => {
                            let mut merged = a.to_vec();
                            for (k, v) in b_map.iter() {
                                if let Some(pos) = merged.iter().position(|(mk, _)| {
                                    if let (Value::String(a), Value::String(b)) = (mk, k) { a == b } else { false }
                                }) { merged[pos].1 = *v; } else { merged.push((*k, *v)); }
                            }
                            Value::map(mc, merged)
                        }
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "Add requires compatible types, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Sub => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    let result = match (&b, &c) {
                        (Value::Number(b), Value::Number(c)) => Value::number(*b - *c),
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "Sub requires numbers, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Mul => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    let result = match (&b, &c) {
                        (Value::Number(b), Value::Number(c)) => Value::number(*b * *c),
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "Mul requires numbers, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Div => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    let result = match (&b, &c) {
                        (Value::Number(b), Value::Number(c)) => {
                            if *c == 0.0 {
                                return Err(VMError::Runtime("Division by zero".to_string()));
                            }
                            Value::number(*b / *c)
                        }
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "Div requires numbers, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Mod => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    let result = match (&b, &c) {
                        (Value::Number(b), Value::Number(c)) => {
                            if *c == 0.0 {
                                return Err(VMError::DivisionByZero);
                            }
                            Value::number(*b % *c)
                        }
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "Mod requires numbers, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }

                // Comparison
                Opcode::Eq => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());
                    let result = Value::boolean(mc, b.deep_eq(&c));
                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Ne => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());
                    let result = Value::boolean(mc, !b.deep_eq(&c));
                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Lt => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    let result = match (&b, &c) {
                        (Value::Number(b), Value::Number(c)) => Value::boolean(mc, *b < *c),
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "Lt requires numbers, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Le => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    let result = match (&b, &c) {
                        (Value::Number(b), Value::Number(c)) => Value::boolean(mc, *b <= *c),
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "Le requires numbers, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Gt => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    let result = match (b, c) {
                        (Value::Number(b), Value::Number(c)) => Value::boolean(mc, b > c),
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "Gt requires numbers, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Ge => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    let result = match (b, c) {
                        (Value::Number(b), Value::Number(c)) => Value::boolean(mc, b >= c),
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "Ge requires numbers, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }

                // Boolean
                Opcode::Not => {
                    let b = frame.get(inst.b());
                    let result = Value::boolean(mc, !b.is_truthy());
                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::And => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());
                    let result = Value::boolean(mc, b.is_truthy() && c.is_truthy());
                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Or => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());
                    let result = Value::boolean(mc, b.is_truthy() || c.is_truthy());
                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::ToBoolean => {
                    let b = frame.get(inst.b());
                    let result = Value::boolean(mc, b.is_truthy());
                    frame.set(inst.a(), result);
                    frame.advance();
                }

                // Map operations
                Opcode::MapNew => {
                    frame.set(inst.a(), Value::map(mc, vec![]));
                    frame.advance();
                }
                Opcode::MapGet => {
                    let map_val = frame.get(inst.b());
                    let key = frame.get(inst.c());

                    match map_val {
                        Value::Map(map) => {
                            let result = map
                                .iter()
                                .find(|(k, _)| k.deep_eq(&key))
                                .map(|(_, v)| *v)
                                .unwrap_or(Value::undefined());
                            frame.set(inst.a(), result);
                        }
                        Value::List(list) => {
                            match key {
                                Value::Number(idx) if idx.fract() == 0.0 => {
                                    let idx = idx as i64;
                                    if idx >= 0 && (idx as usize) < list.len() {
                                        frame.set(inst.a(), list[idx as usize]);
                                    } else {
                                        frame.set(inst.a(), Value::undefined());
                                    }
                                }
                                _ => {
                                    return Err(VMError::TypeError(format!(
                                        "List index must be integer number, got {}",
                                        key.type_name()
                                    )))
                                }
                            }
                        }
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "MapGet requires map or list, got {}",
                                map_val.type_name()
                            )))
                        }
                    }
                    frame.advance();
                }
                Opcode::MapSet => {
                    let map_val = frame.get(inst.b());
                    let _key = frame.get(inst.c());
                    let _value = frame.get(inst.a());

                    match map_val {
                        Value::Map(_) => {
                            return Err(VMError::Runtime(
                                "MapSet on Gc map not yet supported (immutable)".to_string(),
                            ))
                        }
                        Value::List(_) => {
                            return Err(VMError::Runtime(
                                "MapSet on Gc list not yet supported (immutable)".to_string(),
                            ))
                        }
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "MapSet requires map or list, got {}",
                                map_val.type_name()
                            )))
                        }
                    }
                }
                Opcode::MapMerge => {
                    let dest = inst.a();
                    let left_reg = inst.b();
                    let right_reg = inst.c();
                    let left_val = frame.get(left_reg);
                    let right_val = frame.get(right_reg);

                    match (left_val, right_val) {
                        (Value::Map(left_entries), Value::Map(right_entries)) => {
                            let mut merged = left_entries.to_vec();
                            for (k, v) in right_entries.iter() {
                                if let Some(pos) = merged.iter().position(|(mk, _)| {
                                    if let (Value::String(a), Value::String(b)) = (mk, k) { a == b } else { false }
                                }) {
                                    merged[pos].1 = *v;
                                } else {
                                    merged.push((*k, *v));
                                }
                            }
                            frame.set(dest, Value::map(mc, merged));
                            frame.advance();
                        }
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "MapMerge requires two maps, got {} and {}",
                                left_val.type_name(), right_val.type_name()
                            )))
                        }
                    }
                }
                Opcode::MapPick => {
                    return Err(VMError::UnimplementedOpcode(Opcode::MapPick))
                }
                Opcode::MapOmit => {
                    return Err(VMError::UnimplementedOpcode(Opcode::MapOmit))
                }
                Opcode::MapBuild => {
                    let first_reg = inst.b();
                    let entry_count = inst.c() as usize;

                    let mut entries = Vec::with_capacity(entry_count);
                    for i in 0..entry_count {
                        let key_reg = first_reg as usize + i * 2;
                        let val_reg = first_reg as usize + i * 2 + 1;
                        let key = frame.get(key_reg as u8);
                        let value = frame.get(val_reg as u8);
                        entries.push((key, value));
                    }

                    frame.set(inst.a(), Value::map(mc, entries));
                    frame.advance();
                }

                // List operations
                Opcode::ListNew => {
                    frame.set(inst.a(), Value::list(mc, vec![]));
                    frame.advance();
                }
                Opcode::ListGet => {
                    let list_val = frame.get(inst.b());
                    let idx_val = frame.get(inst.c());

                    match list_val {
                        Value::List(list) => {
                            match idx_val {
                                Value::Number(idx) if idx.fract() == 0.0 => {
                                    let idx = idx as i64;
                                    if idx >= 0 && (idx as usize) < list.len() {
                                        frame.set(inst.a(), list[idx as usize]);
                                    } else {
                                        frame.set(inst.a(), Value::undefined());
                                    }
                                }
                                _ => {
                                    return Err(VMError::TypeError(format!(
                                        "List index must be integer number, got {}",
                                        idx_val.type_name()
                                    )))
                                }
                            }
                        }
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "ListGet requires list, got {}",
                                list_val.type_name()
                            )))
                        }
                    }
                    frame.advance();
                }
                Opcode::ListSet => {
                    return Err(VMError::Runtime(
                        "ListSet on Gc list not yet supported (immutable)".to_string(),
                    ))
                }
                Opcode::ListPush => {
                    return Err(VMError::Runtime(
                        "ListPush on Gc list not yet supported (immutable)".to_string(),
                    ))
                }
                Opcode::ListPop => {
                    return Err(VMError::Runtime(
                        "ListPop on Gc list not yet supported (immutable)".to_string(),
                    ))
                }
                Opcode::ListLen => {
                    let list_val = frame.get(inst.b());

                    match list_val {
                        Value::List(list) => {
                            frame.set(inst.a(), Value::number(list.len() as f64));
                        }
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "ListLen requires list, got {}",
                                list_val.type_name()
                            )))
                        }
                    }
                    frame.advance();
                }
                Opcode::ListBuild => {
                    let first_reg = inst.b();
                    let count = inst.c() as usize;

                    let mut elements = Vec::with_capacity(count);
                    for i in 0..count {
                        let elem_reg = first_reg as usize + i;
                        elements.push(frame.get(elem_reg as u8));
                    }

                    frame.set(inst.a(), Value::list(mc, elements));
                    frame.advance();
                }

                // String operations
                Opcode::StringConcat => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    let b_str = match b {
                        Value::String(s) => (*s).clone(),
                        Value::Number(n) => {
                            if n.fract() == 0.0 {
                                (n as i64).to_string()
                            } else {
                                n.to_string()
                            }
                        }
                        Value::Boolean(b_val) => b_val.to_string(),
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "StringConcat requires string-convertible values, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    let c_str = match c {
                        Value::String(s) => (*s).clone(),
                        Value::Number(n) => {
                            if n.fract() == 0.0 {
                                (n as i64).to_string()
                            } else {
                                n.to_string()
                            }
                        }
                        Value::Boolean(b_val) => b_val.to_string(),
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "StringConcat requires string-convertible values, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    let combined = format!("{}{}", b_str, c_str);
                    frame.set(inst.a(), Value::string(mc, combined));
                    frame.advance();
                }
                Opcode::StringLen => {
                    let str_val = frame.get(inst.b());

                    match str_val {
                        Value::String(s) => {
                            frame.set(inst.a(), Value::number(s.len() as f64));
                        }
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "StringLen requires string, got {}",
                                str_val.type_name()
                            )))
                        }
                    }
                    frame.advance();
                }
                Opcode::StringSlice => {
                    return Err(VMError::UnimplementedOpcode(Opcode::StringSlice))
                }
                Opcode::StringToUpper => {
                    return Err(VMError::UnimplementedOpcode(Opcode::StringToUpper))
                }
                Opcode::StringToLower => {
                    return Err(VMError::UnimplementedOpcode(Opcode::StringToLower))
                }
                Opcode::StringCapitalize => {
                    return Err(VMError::UnimplementedOpcode(Opcode::StringCapitalize))
                }

                // Loop operators
                Opcode::LoopMap => return Err(VMError::UnimplementedOpcode(Opcode::LoopMap)),
                Opcode::LoopFilter => {
                    return Err(VMError::UnimplementedOpcode(Opcode::LoopFilter))
                }
                Opcode::LoopFlatMap => {
                    return Err(VMError::UnimplementedOpcode(Opcode::LoopFlatMap))
                }
                Opcode::LoopUniqueBy => {
                    return Err(VMError::UnimplementedOpcode(Opcode::LoopUniqueBy))
                }
                Opcode::LoopGroupBy => {
                    return Err(VMError::UnimplementedOpcode(Opcode::LoopGroupBy))
                }
                Opcode::LoopEachToString => {
                    let list_reg = inst.a();
                    let dest_reg = inst.b();
                    let list_val = frame.get(list_reg);
                    if let Value::List(items) = list_val {
                        let strings: Vec<Value<'a>> = items.iter().map(|v| {
                            match v {
                                Value::String(s) => Value::string(mc, s.to_string()),
                                Value::Number(n) => {
                                    if n.fract() == 0.0 {
                                        Value::string(mc, (*n as i64).to_string())
                                    } else {
                                        Value::string(mc, n.to_string())
                                    }
                                }
                                Value::Boolean(b) => Value::string(mc, b.to_string()),
                                _ => Value::string(mc, format!("{}", v)),
                            }
                        }).collect();
                        frame.set(dest_reg, Value::list(mc, strings));
                    } else {
                        return Err(VMError::TypeError(format!(
                            "LoopEachToString requires a list, got {}", list_val.type_name()
                        )));
                    }
                    frame.advance();
                }
                Opcode::LoopEvery => {
                    return Err(VMError::UnimplementedOpcode(Opcode::LoopEvery))
                }
                Opcode::LoopSome => {
                    return Err(VMError::UnimplementedOpcode(Opcode::LoopSome))
                }
                Opcode::LoopFind => {
                    return Err(VMError::UnimplementedOpcode(Opcode::LoopFind))
                }
                Opcode::LoopSort => {
                    return Err(VMError::UnimplementedOpcode(Opcode::LoopSort))
                }
                Opcode::LoopCompact => {
                    return Err(VMError::UnimplementedOpcode(Opcode::LoopCompact))
                }

                // Error handling
                Opcode::Throw => {
                    let value = frame.get(inst.a());
                    frame.thrown_value = Some(value);
                    if let Some(catch_frame) = frame.catch_stack.last() {
                        frame.pc = catch_frame.handler_start;
                        frame.thrown_value = None;
                    } else {
                        let value_str = value.to_string();
                        return Err(VMError::Runtime(format!("Uncaught exception: {}", value_str)));
                    }
                }
                Opcode::Catch => {
                    let handler_offset = inst.b() as i8 as i16;
                    let handler_start = (frame.pc() as i64 + handler_offset as i64) as usize;
                    let end_label = (frame.pc() as i64 + 2) as usize;
                    frame.catch_stack.push(CatchFrame { handler_start, end_label });
                    frame.advance();
                }
                Opcode::CatchEnd => {
                    frame.catch_stack.pop();
                    frame.advance();
                }
                Opcode::MakeClosure => {
                    let body_start = (inst.b() as usize) | ((inst.c() as usize) << 8);
                    let closure_end = Self::find_closure_return(code, body_start);

                    let max_arg = code[body_start..closure_end]
                        .iter()
                        .filter(|i| i.opcode() == Opcode::LoadArg)
                        .map(|i| i.b())
                        .max()
                        .unwrap_or(0);
                    let arg_count = max_arg + 1;
                    let closure_data = ClosureData { body_start, arg_count };
                    let closure = Value::Closure(Gc::new(mc, closure_data));
                    frame.set(inst.a(), closure);
                    frame.pc = closure_end + 1;
                }
                Opcode::LoadArg => {
                    let arg_idx = inst.b() as usize;
                    let arg_val = frame.get_arg(arg_idx);
                    if matches!(arg_val, Value::Undefined) {
                        let fallback = if arg_idx == 0 { "_<".to_string() } else { "_>".to_string() };
                        frame.set(inst.a(), Value::string(mc, fallback));
                    } else {
                        frame.set(inst.a(), arg_val);
                    }
                    frame.advance();
                }

                // Async
                Opcode::MakeAsync => {
                    return Err(VMError::UnimplementedOpcode(Opcode::MakeAsync))
                }
                Opcode::Await => return Err(VMError::UnimplementedOpcode(Opcode::Await)),

                // Special
                Opcode::Push => return Err(VMError::UnimplementedOpcode(Opcode::Push)),
                Opcode::Pop => return Err(VMError::UnimplementedOpcode(Opcode::Pop)),
                Opcode::Dup => return Err(VMError::UnimplementedOpcode(Opcode::Dup)),
                Opcode::ReverseArgs => {
                    let a = inst.a();
                    let b = inst.b();
                    if a != b {
                        let va = frame.get(a);
                        let vb = frame.get(b);
                        frame.set(a, vb);
                        frame.set(b, va);
                    }
                    frame.advance();
                }
                Opcode::SilentExec => {
                    return Err(VMError::UnimplementedOpcode(Opcode::SilentExec))
                }
                Opcode::Import => {
                    let source_reg = inst.b();
                    let source = frame.get(source_reg);
                    let name = source.to_string();
                    frame.set(inst.a(), Value::map(mc, vec![
                        (Value::string(mc, "module".to_string()), source),
                        (Value::string(mc, "source".to_string()), Value::string(mc, name)),
                    ]));
                    frame.advance();
                }
                Opcode::Export => {
                    frame.advance();
                }
                Opcode::BindName => {
                    let const_idx = inst.b() as usize;
                    let const_val = constants
                        .get(const_idx)
                        .ok_or(VMError::ConstantIndexOutOfBounds(const_idx))?;
                    
                    if let Constant::String(name) = const_val {
                        let value = frame.get(inst.c());
                        frame.bindings.insert(name.clone(), value);
                    } else {
                        return Err(VMError::TypeError("BindName requires string constant".to_string()));
                    }
                    frame.advance();
                }
                Opcode::LookupName => {
                    let const_idx = inst.b() as usize;
                    let const_val = constants
                        .get(const_idx)
                        .ok_or(VMError::ConstantIndexOutOfBounds(const_idx))?;
                    
                    if let Constant::String(name) = const_val {
                        if let Some(&value) = frame.bindings.get(name) {
                            frame.set(inst.a(), value);
                        } else {
                            return Err(VMError::Runtime(format!("Undefined binding: {}", name)));
                        }
                    } else {
                        return Err(VMError::TypeError("LookupName requires string constant".to_string()));
                    }
                    frame.advance();
                }
            }
        }

        Ok(frame.get(0))
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum VMError {
    #[error("Invalid opcode: {0}")]
    InvalidOpcode(u8),
    #[error("Unimplemented opcode: {0:?}")]
    UnimplementedOpcode(Opcode),
    #[error("Constant index out of bounds: {0}")]
    ConstantIndexOutOfBounds(usize),
    #[error("Type error: {0}")]
    TypeError(String),
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Runtime error: {0}")]
    Runtime(String),
}

fn json_to_lr_value<'gc>(mc: &Mutation<'gc>, val: &serde_json::Value) -> Value<'gc> {
    match val {
        serde_json::Value::Null => Value::undefined(),
        serde_json::Value::Bool(b) => Value::boolean(mc, *b),
        serde_json::Value::Number(n) => {
            if let Some(f) = n.as_f64() {
                Value::number(f)
            } else {
                Value::number(0.0)
            }
        }
        serde_json::Value::String(s) => Value::string(mc, s.clone()),
        serde_json::Value::Array(arr) => {
            let items: Vec<Value<'gc>> = arr.iter().map(|v| json_to_lr_value(mc, v)).collect();
            Value::list(mc, items)
        }
        serde_json::Value::Object(map) => {
            let entries: Vec<(Value<'gc>, Value<'gc>)> = map
                .iter()
                .map(|(k, v)| (Value::string(mc, k.to_string()), json_to_lr_value(mc, v)))
                .collect();
            Value::map(mc, entries)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lr_bytecode::Instruction;

    fn build_chunk(f: impl FnOnce(&mut Chunk)) -> Chunk {
        let mut chunk = Chunk::new();
        f(&mut chunk);
        chunk
    }

    #[test]
    fn test_vm_nop() {
        let chunk = build_chunk(|c| {
            c.emit(Instruction::new(Opcode::Nop, 0, 0, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "undefined");
    }

    #[test]
    fn test_vm_load_constant_number() {
        let chunk = build_chunk(|c| {
            let idx = c.add_constant(Constant::Number(42.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 1, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "42");
    }

    #[test]
    fn test_vm_load_constant_string() {
        let chunk = build_chunk(|c| {
            let idx = c.add_constant(Constant::String("hello".to_string())).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 1, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn test_vm_load_constant_boolean() {
        let chunk = build_chunk(|c| {
            let idx = c.add_constant(Constant::Boolean(true)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 1, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_vm_arithmetic_add() {
        let chunk = build_chunk(|c| {
            let idx5 = c.add_constant(Constant::Number(5.0)).unwrap();
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx5));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx3));
            c.emit(Instruction::new(Opcode::Add, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "8");
    }

    #[test]
    fn test_vm_arithmetic_sub() {
        let chunk = build_chunk(|c| {
            let idx10 = c.add_constant(Constant::Number(10.0)).unwrap();
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx10));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx3));
            c.emit(Instruction::new(Opcode::Sub, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "7");
    }

    #[test]
    fn test_vm_arithmetic_mul() {
        let chunk = build_chunk(|c| {
            let idx4 = c.add_constant(Constant::Number(4.0)).unwrap();
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx4));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx3));
            c.emit(Instruction::new(Opcode::Mul, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "12");
    }

    #[test]
    fn test_vm_arithmetic_div() {
        let chunk = build_chunk(|c| {
            let idx10 = c.add_constant(Constant::Number(10.0)).unwrap();
            let idx2 = c.add_constant(Constant::Number(2.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx10));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx2));
            c.emit(Instruction::new(Opcode::Div, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "5");
    }

    #[test]
    fn test_vm_comparison_eq() {
        let chunk = build_chunk(|c| {
            let idx5 = c.add_constant(Constant::Number(5.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx5));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx5));
            c.emit(Instruction::new(Opcode::Eq, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_vm_comparison_lt() {
        let chunk = build_chunk(|c| {
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            let idx5 = c.add_constant(Constant::Number(5.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx3));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx5));
            c.emit(Instruction::new(Opcode::Lt, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_vm_boolean_not() {
        let chunk = build_chunk(|c| {
            let idx = c.add_constant(Constant::Boolean(true)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx));
            c.emit(Instruction::new(Opcode::Not, 2, 1, 0));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 2, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn test_vm_string_concat() {
        let chunk = build_chunk(|c| {
            let idx1 = c.add_constant(Constant::String("hello".to_string())).unwrap();
            let idx2 = c
                .add_constant(Constant::String(" world".to_string()))
                .unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx1));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx2));
            c.emit(Instruction::new(Opcode::StringConcat, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello world");
    }

    #[test]
    fn test_vm_jump() {
        let chunk = build_chunk(|c| {
            let idx = c.add_constant(Constant::Number(42.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx));
            c.emit(Instruction::new(Opcode::Jump, 2, 0, 0)); // skip next instruction
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx)); // should be skipped
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 1, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "42");
    }

    #[test]
    fn test_vm_list_build_three_elements() {
        let chunk = build_chunk(|c| {
            let idx42 = c.add_constant(Constant::Number(42.0)).unwrap();
            let idx24 = c.add_constant(Constant::Number(24.0)).unwrap();
            let idx6 = c.add_constant(Constant::Number(6.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx42));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx24));
            c.emit(Instruction::new(Opcode::LoadConstant, 3, 0, idx6));
            c.emit(Instruction::new(Opcode::ListBuild, 4, 1, 3));
            c.emit(Instruction::new(Opcode::ListLen, 5, 4, 0));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 5, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "3");
    }

    #[test]
    fn test_vm_list_operations() {
        let chunk = build_chunk(|c| {
            let _idx42 = c.add_constant(Constant::Number(42.0)).unwrap();
            c.emit(Instruction::new(Opcode::ListNew, 1, 0, 0));
            c.emit(Instruction::new(Opcode::ListLen, 2, 1, 0));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 2, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "0");
    }

    #[test]
    fn test_vm_map_operations() {
        let chunk = build_chunk(|c| {
            c.emit(Instruction::new(Opcode::MapNew, 1, 0, 0));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 1, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("{"));
    }

    #[test]
    fn test_vm_division_by_zero() {
        let chunk = build_chunk(|c| {
            let idx10 = c.add_constant(Constant::Number(10.0)).unwrap();
            let idx0 = c.add_constant(Constant::Number(0.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx10));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx0));
            c.emit(Instruction::new(Opcode::Div, 3, 1, 2));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(matches!(result, Err(VMError::Runtime(_))));
    }

    #[test]
    fn test_vm_type_error() {
        let chunk = build_chunk(|c| {
            let idx_num = c.add_constant(Constant::Number(42.0)).unwrap();
            let idx_bool = c.add_constant(Constant::Boolean(true)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx_num));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_bool));
            c.emit(Instruction::new(Opcode::Sub, 3, 1, 2));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(matches!(result, Err(VMError::TypeError(_))));
    }

    #[test]
    fn test_vm_add_string_strings() {
        let chunk = build_chunk(|c| {
            let idx1 = c.add_constant(Constant::String("foo".to_string())).unwrap();
            let idx2 = c.add_constant(Constant::String("bar".to_string())).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx1));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx2));
            c.emit(Instruction::new(Opcode::Add, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "foobar");
    }

    #[test]
    fn test_vm_comparison_ne() {
        let chunk = build_chunk(|c| {
            let idx5 = c.add_constant(Constant::Number(5.0)).unwrap();
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx5));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx3));
            c.emit(Instruction::new(Opcode::Ne, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_vm_comparison_le() {
        let chunk = build_chunk(|c| {
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            let idx5 = c.add_constant(Constant::Number(5.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx3));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx5));
            c.emit(Instruction::new(Opcode::Le, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_vm_comparison_gt() {
        let chunk = build_chunk(|c| {
            let idx5 = c.add_constant(Constant::Number(5.0)).unwrap();
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx5));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx3));
            c.emit(Instruction::new(Opcode::Gt, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_vm_comparison_ge() {
        let chunk = build_chunk(|c| {
            let idx5 = c.add_constant(Constant::Number(5.0)).unwrap();
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx5));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx3));
            c.emit(Instruction::new(Opcode::Ge, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_vm_boolean_and() {
        let chunk = build_chunk(|c| {
            let idx1 = c.add_constant(Constant::Boolean(true)).unwrap();
            let idx2 = c.add_constant(Constant::Boolean(false)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx1));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx2));
            c.emit(Instruction::new(Opcode::And, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn test_vm_boolean_or() {
        let chunk = build_chunk(|c| {
            let idx1 = c.add_constant(Constant::Boolean(true)).unwrap();
            let idx2 = c.add_constant(Constant::Boolean(false)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx1));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx2));
            c.emit(Instruction::new(Opcode::Or, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_vm_boolean_to_boolean() {
        let chunk = build_chunk(|c| {
            let idx42 = c.add_constant(Constant::Number(42.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx42));
            c.emit(Instruction::new(Opcode::ToBoolean, 2, 1, 0));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 2, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_vm_mod() {
        let chunk = build_chunk(|c| {
            let idx10 = c.add_constant(Constant::Number(10.0)).unwrap();
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx10));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx3));
            c.emit(Instruction::new(Opcode::Mod, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1");
    }

    #[test]
    fn test_vm_mod_by_zero() {
        let chunk = build_chunk(|c| {
            let idx10 = c.add_constant(Constant::Number(10.0)).unwrap();
            let idx0 = c.add_constant(Constant::Number(0.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx10));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx0));
            c.emit(Instruction::new(Opcode::Mod, 3, 1, 2));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Division by zero"));
    }

    #[test]
    fn test_vm_call_map_function_simple() {
        let chunk = build_chunk(|c| {
            let idx_fn = c.add_constant(Constant::String("_<".to_string())).unwrap();
            let idx_arg = c.add_constant(Constant::String("_<".to_string())).unwrap();
            let idx_five = c.add_constant(Constant::Number(5.0)).unwrap();

            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx_fn));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_arg));
            c.emit(Instruction::new(Opcode::MapBuild, 3, 1, 1));
            c.emit(Instruction::new(Opcode::LoadConstant, 4, 0, idx_five));
            c.emit(Instruction::new(Opcode::Call, 5, 3, 4));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 5, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "5");
    }

    #[test]
    fn test_vm_unimplemented_opcodes() {
        let unimplemented_opcodes = vec![
            Opcode::MapPick,
            Opcode::MapOmit,
            Opcode::StringSlice,
            Opcode::StringToUpper,
            Opcode::StringToLower,
            Opcode::StringCapitalize,
            Opcode::LoopMap,
            Opcode::LoopFilter,
            Opcode::LoopFlatMap,
            Opcode::LoopUniqueBy,
            Opcode::LoopGroupBy,
            Opcode::LoopEvery,
            Opcode::LoopSome,
            Opcode::LoopFind,
            Opcode::LoopSort,
            Opcode::LoopCompact,
            Opcode::MakeAsync,
            Opcode::Await,
            Opcode::Push,
            Opcode::Pop,
            Opcode::Dup,
            Opcode::SilentExec,
        ];

        for opcode in unimplemented_opcodes {
            let chunk = build_chunk(|c| {
                c.emit(Instruction::new(opcode, 0, 0, 0));
                c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
            });

            let mut vm = VM::new();
            let result = vm.execute(&chunk);
            assert!(
                matches!(result, Err(VMError::UnimplementedOpcode(op)) if op == opcode),
                "Opcode {:?} should return UnimplementedOpcode",
                opcode
            );
        }
    }

    #[test]
    fn test_vm_map_merge() {
        let chunk = build_chunk(|c| {
            let idx_a = c.add_constant(Constant::String("a".to_string())).unwrap();
            let idx_1 = c.add_constant(Constant::Number(1.0)).unwrap();
            let idx_b = c.add_constant(Constant::String("b".to_string())).unwrap();
            let idx_2 = c.add_constant(Constant::Number(2.0)).unwrap();
            let idx_c = c.add_constant(Constant::String("c".to_string())).unwrap();
            let idx_3 = c.add_constant(Constant::Number(3.0)).unwrap();

            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx_a));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_1));
            c.emit(Instruction::new(Opcode::LoadConstant, 3, 0, idx_b));
            c.emit(Instruction::new(Opcode::LoadConstant, 4, 0, idx_2));
            c.emit(Instruction::new(Opcode::MapBuild, 5, 1, 2));

            c.emit(Instruction::new(Opcode::LoadConstant, 6, 0, idx_b));
            c.emit(Instruction::new(Opcode::LoadConstant, 7, 0, idx_3));
            c.emit(Instruction::new(Opcode::LoadConstant, 8, 0, idx_c));
            c.emit(Instruction::new(Opcode::LoadConstant, 9, 0, idx_3));
            c.emit(Instruction::new(Opcode::MapBuild, 10, 6, 2));

            c.emit(Instruction::new(Opcode::MapMerge, 11, 5, 10));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 11, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        let s = result.unwrap();
        assert!(s.contains("a"), "merged map should contain key 'a': {}", s);
        assert!(s.contains("b"), "merged map should contain key 'b': {}", s);
        assert!(s.contains("c"), "merged map should contain key 'c': {}", s);
    }

    #[test]
    fn test_vm_error_constructor() {
        let chunk = build_chunk(|c| {
            let idx_err = c.add_constant(Constant::String("Error".to_string())).unwrap();
            let idx_msg = c.add_constant(Constant::String("something went wrong".to_string())).unwrap();

            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx_err));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_msg));
            c.emit(Instruction::new(Opcode::ListBuild, 3, 2, 1));
            c.emit(Instruction::new(Opcode::Call, 4, 1, 3));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 4, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        let s = result.unwrap();
        assert!(s.contains("Error"), "Should contain 'Error': {}", s);
        assert!(s.contains("something went wrong"), "Should contain message: {}", s);
    }

    #[test]
    fn test_vm_arithmetic_float() {
        let chunk = build_chunk(|c| {
            let idx1 = c.add_constant(Constant::Number(3.0)).unwrap();
            let idx2 = c.add_constant(Constant::Number(2.14)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx1));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx2));
            c.emit(Instruction::new(Opcode::Add, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        let result_str = result.unwrap();
        assert!(result_str.starts_with("5.14"), "Expected result to start with '5.14', got '{}'", result_str);
    }

    #[test]
    fn test_vm_string_len() {
        let chunk = build_chunk(|c| {
            let idx = c.add_constant(Constant::String("hello".to_string())).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx));
            c.emit(Instruction::new(Opcode::StringLen, 2, 1, 0));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 2, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "5");
    }

    #[test]
    fn test_vm_number_display() {
        let chunk = build_chunk(|c| {
            let idx_int = c.add_constant(Constant::Number(42.0)).unwrap();
            let _idx_float = c.add_constant(Constant::Number(3.25)).unwrap();
            let _idx_neg_int = c.add_constant(Constant::Number(-7.0)).unwrap();
            let _idx_neg_float = c.add_constant(Constant::Number(-2.5)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 0, 0, idx_int));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert_eq!(result.unwrap(), "42");


        let chunk = build_chunk(|c| {
            let idx_float = c.add_constant(Constant::Number(3.25)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 0, 0, idx_float));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let result = vm.execute(&chunk);
        assert_eq!(result.unwrap(), "3.25");
    }

    #[test]
    fn test_vm_zero_is_falsy() {
        let chunk = build_chunk(|c| {
            let idx = c.add_constant(Constant::Number(0.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx));
            c.emit(Instruction::new(Opcode::ToBoolean, 2, 1, 0));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 2, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn test_vm_empty_string_is_falsy() {
        let chunk = build_chunk(|c| {
            let idx = c.add_constant(Constant::String("".to_string())).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx));
            c.emit(Instruction::new(Opcode::ToBoolean, 2, 1, 0));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 2, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn test_call_arithmetic() {
        let chunk = build_chunk(|c| {
            let idx5 = c.add_constant(Constant::Number(5.0)).unwrap();
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            let idx_plus = c.add_constant(Constant::String("+".to_string())).unwrap();

            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx5));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_plus));
            c.emit(Instruction::new(Opcode::Call, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadConstant, 4, 0, idx3));
            c.emit(Instruction::new(Opcode::Call, 5, 3, 4));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 5, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "8");
    }

    #[test]
    fn test_call_string_concat() {
        let chunk = build_chunk(|c| {
            let idx_hello = c.add_constant(Constant::String("hello".to_string())).unwrap();
            let idx_world = c.add_constant(Constant::String(" world".to_string())).unwrap();
            let idx_plus = c.add_constant(Constant::String("+".to_string())).unwrap();

            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx_hello));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_plus));
            c.emit(Instruction::new(Opcode::Call, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadConstant, 4, 0, idx_world));
            c.emit(Instruction::new(Opcode::Call, 5, 3, 4));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 5, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello world");
    }

    #[test]
    fn test_call_nested_arithmetic() {
        let chunk = build_chunk(|c| {
            let idx1 = c.add_constant(Constant::Number(1.0)).unwrap();
            let idx2 = c.add_constant(Constant::Number(2.0)).unwrap();
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            let idx_plus = c.add_constant(Constant::String("+".to_string())).unwrap();

            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx1));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_plus));
            c.emit(Instruction::new(Opcode::Call, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadConstant, 4, 0, idx2));
            c.emit(Instruction::new(Opcode::Call, 5, 3, 4));
            c.emit(Instruction::new(Opcode::LoadConstant, 6, 0, idx_plus));
            c.emit(Instruction::new(Opcode::Call, 7, 5, 6));
            c.emit(Instruction::new(Opcode::LoadConstant, 8, 0, idx3));
            c.emit(Instruction::new(Opcode::Call, 9, 7, 8));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 9, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "6");
    }

    #[test]
    fn test_operator_or_number_truthy_returns_left() {
        let chunk = build_chunk(|c| {
            let idx5 = c.add_constant(Constant::Number(5.0)).unwrap();
            let idx10 = c.add_constant(Constant::Number(10.0)).unwrap();
            let idx_or = c.add_constant(Constant::String("|".to_string())).unwrap();

            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx5));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_or));
            c.emit(Instruction::new(Opcode::Call, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadConstant, 4, 0, idx10));
            c.emit(Instruction::new(Opcode::Call, 5, 3, 4));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 5, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "5");
    }

    #[test]
    fn test_operator_or_number_falsy_returns_right() {
        let chunk = build_chunk(|c| {
            let idx0 = c.add_constant(Constant::Number(0.0)).unwrap();
            let idx10 = c.add_constant(Constant::Number(10.0)).unwrap();
            let idx_or = c.add_constant(Constant::String("|".to_string())).unwrap();

            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx0));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_or));
            c.emit(Instruction::new(Opcode::Call, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadConstant, 4, 0, idx10));
            c.emit(Instruction::new(Opcode::Call, 5, 3, 4));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 5, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "10");
    }

    #[test]
    fn test_operator_or_undefined_returns_right() {
        let chunk = build_chunk(|c| {
            let idx_undef = c.add_constant(Constant::Undefined).unwrap();
            let idx42 = c.add_constant(Constant::Number(42.0)).unwrap();
            let idx_or = c.add_constant(Constant::String("|".to_string())).unwrap();

            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx_undef));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_or));
            c.emit(Instruction::new(Opcode::Call, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadConstant, 4, 0, idx42));
            c.emit(Instruction::new(Opcode::Call, 5, 3, 4));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 5, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "42");
    }

    #[test]
    fn test_operator_or_boolean_true_returns_left() {
        let chunk = build_chunk(|c| {
            let idx_true = c.add_constant(Constant::Boolean(true)).unwrap();
            let idx_false = c.add_constant(Constant::Boolean(false)).unwrap();
            let idx_or = c.add_constant(Constant::String("|".to_string())).unwrap();

            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx_true));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_or));
            c.emit(Instruction::new(Opcode::Call, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadConstant, 4, 0, idx_false));
            c.emit(Instruction::new(Opcode::Call, 5, 3, 4));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 5, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_operator_or_boolean_false_returns_right() {
        let chunk = build_chunk(|c| {
            let idx_false = c.add_constant(Constant::Boolean(false)).unwrap();
            let idx_true = c.add_constant(Constant::Boolean(true)).unwrap();
            let idx_or = c.add_constant(Constant::String("|".to_string())).unwrap();

            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx_false));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_or));
            c.emit(Instruction::new(Opcode::Call, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadConstant, 4, 0, idx_true));
            c.emit(Instruction::new(Opcode::Call, 5, 3, 4));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 5, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_operator_or_string_truthy_returns_left() {
        let chunk = build_chunk(|c| {
            let idx_hello = c.add_constant(Constant::String("hello".to_string())).unwrap();
            let idx_world = c.add_constant(Constant::String("world".to_string())).unwrap();
            let idx_or = c.add_constant(Constant::String("|".to_string())).unwrap();

            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx_hello));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_or));
            c.emit(Instruction::new(Opcode::Call, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadConstant, 4, 0, idx_world));
            c.emit(Instruction::new(Opcode::Call, 5, 3, 4));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 5, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn test_operator_or_string_empty_returns_right() {
        let chunk = build_chunk(|c| {
            let idx_empty = c.add_constant(Constant::String("".to_string())).unwrap();
            let idx_default = c.add_constant(Constant::String("default".to_string())).unwrap();
            let idx_or = c.add_constant(Constant::String("|".to_string())).unwrap();

            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx_empty));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_or));
            c.emit(Instruction::new(Opcode::Call, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadConstant, 4, 0, idx_default));
            c.emit(Instruction::new(Opcode::Call, 5, 3, 4));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 5, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "default");
    }

    #[test]
    fn test_operator_or_list_truthy_returns_left() {
        let chunk = build_chunk(|c| {
            let idx1 = c.add_constant(Constant::Number(1.0)).unwrap();
            let idx2 = c.add_constant(Constant::Number(2.0)).unwrap();
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            let idx_or = c.add_constant(Constant::String("|".to_string())).unwrap();

            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx1));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx2));
            c.emit(Instruction::new(Opcode::ListBuild, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadConstant, 4, 0, idx_or));
            c.emit(Instruction::new(Opcode::Call, 5, 3, 4));
            c.emit(Instruction::new(Opcode::LoadConstant, 6, 0, idx3));
            c.emit(Instruction::new(Opcode::Call, 7, 5, 6));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 7, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[1, 2]");
    }

    #[test]
    fn test_json_parse_object() {
        let chunk = build_chunk(|c| {
            let idx_json_str = c.add_constant(Constant::String(r#"{"a":1,"b":"hello"}"#.to_string())).unwrap();
            let idx_json_op = c.add_constant(Constant::String("/json".to_string())).unwrap();

            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx_json_str));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_json_op));
            c.emit(Instruction::new(Opcode::Call, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "{a: 1, b: hello}");
    }

    #[test]
    fn test_json_parse_array() {
        let chunk = build_chunk(|c| {
            let idx_json_str = c.add_constant(Constant::String("[1,2,3]".to_string())).unwrap();
            let idx_json_op = c.add_constant(Constant::String("/json".to_string())).unwrap();

            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx_json_str));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_json_op));
            c.emit(Instruction::new(Opcode::Call, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[1, 2, 3]");
    }

    #[test]
    fn test_json_parse_nested() {
        let chunk = build_chunk(|c| {
            let idx_json_str = c.add_constant(Constant::String(r#"{"users":[{"name":"alice"},{"name":"bob"}]}"#.to_string())).unwrap();
            let idx_json_op = c.add_constant(Constant::String("/json".to_string())).unwrap();

            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx_json_str));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_json_op));
            c.emit(Instruction::new(Opcode::Call, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "{users: [{name: alice}, {name: bob}]}");
    }

    #[test]
    fn test_json_parse_error() {
        let chunk = build_chunk(|c| {
            let idx_json_str = c.add_constant(Constant::String("not json".to_string())).unwrap();
            let idx_json_op = c.add_constant(Constant::String("/json".to_string())).unwrap();

            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx_json_str));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_json_op));
            c.emit(Instruction::new(Opcode::Call, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_err());
    }
}