/// statement => <comment> | <tree>
///
/// variable => [a-zA-Z_][a-zA-Z_0-9-]*
/// varop => <variable> | <assignment>
///
/// functioncall => <expression>(<expression>)
/// comment => //.*
/// string => ".*?" | '.*?'
/// number => -?[0-9]+
///
/// tree => <ifstatement> | <arithmetic>
/// arithmetic => <subarithmetic> | <subarithmetic> [+-] <subarithmetic>
/// subarithmetic => <basearithmetic> | <basearithmetic> [/*] <basearithmetic>
/// basearithmetic => <expression> | <expression> [**] <expression>
/// expression => (<tree>) | <varop> | <number> | <string> | <functioncall>
///
///
/// TODO:
///     Arithmetic
///
///     if
///     Boolean logic
///
///     loops
///     iterators
///

#[macro_use]
extern crate lazy_static;

use regex::Regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

trait CogObject {

    fn to_string(&self) -> String { String::new() }

    fn to_int(&self) -> i128 { 0 }

    fn call(&mut self, _arg: Box<dyn CogObject>) -> Box<dyn CogObject> {
        panic!("Object is not callable!")
    }

    fn cloned(&self) -> Box<dyn CogObject>;

    fn plus(&self, _other: Box<dyn CogObject>) -> Box<dyn CogObject> {
        panic!("Object cannot be summmed!")
    }

    fn cog_type(&self) -> &str;
}

#[derive(Clone)]
struct CogString {
    data: String
}

impl CogObject for CogString {

    fn to_string(&self) -> String {
        self.data.clone()
    }

    fn cloned(&self) -> Box<dyn CogObject> {
        Box::new(self.clone())
    }

    fn cog_type(&self) -> &str {
        "string"
    }
}

#[derive(Clone)]
struct CogNone;

impl CogObject for CogNone {

    fn to_string(&self) -> String {
        "None".to_string()
    }

    fn cloned(&self) -> Box<dyn CogObject> {
        Box::new(self.clone())
    }

    fn cog_type(&self) -> &str {
        "none"
    }
}

#[derive(Clone)]
struct CogInt {
    data: i128
}

impl CogObject for CogInt {

    fn to_string(&self) -> String {
        self.data.to_string()
    }

    fn to_int(&self) -> i128 {
        self.data
    }

    fn cloned(&self) -> Box<dyn CogObject> {
        Box::new(self.clone())
    }

    fn plus(&self, other: Box<dyn CogObject>) -> Box<dyn CogObject> {
        if self.cog_type() != other.cog_type() {
            panic!(
                "Cannot sum type '{}' with type '{}'",
                self.cog_type(),
                other.cog_type()
            );
        }

        Box::new(CogInt{data: self.to_int() + other.to_int()})
    }

    fn cog_type(&self) -> &str {
        "int"
    }
}

#[derive(Clone)]
struct CogFn {
    data: fn(Box<dyn CogObject>) -> Box<dyn CogObject>
}

impl CogObject for CogFn {

    fn to_string(&self) -> String {
        "Function".to_string()
    }

    fn call(&mut self, arg: Box<dyn CogObject>) -> Box<dyn CogObject> {
        (self.data)(arg)
    }

    fn cloned(&self) -> Box<dyn CogObject> {
        Box::new(self.clone())
    }

    fn cog_type(&self) -> &str {
        "function"
    }
}

struct Interpreter {
    variables: HashMap<String, Box<dyn CogObject>>
}

impl Interpreter {

    fn new() -> Interpreter {
        let mut intr = Interpreter{variables: HashMap::new()};

        intr.variables.insert("print".to_string(), Box::new(CogFn{data: cog_print}));

        intr
    }

    fn interpret_statement(&mut self, line: &str) {
        lazy_static! {
            static ref COMMENT_RE: Regex = Regex::new("//.*").unwrap();
        }

        if COMMENT_RE.is_match(line) {
            println!("Comment: {}", line);
        } else {
            self.interpret_expression(line);
        }
    }

    fn interpret_expression(&mut self, expr: &str) -> Box<dyn CogObject> {
        lazy_static! {
            static ref FUNCTION_CALL_RE: Regex =
                Regex::new("(^[a-zA-Z_][a-zA-Z_0-9-]*)\\((.*)\\)").unwrap();
            static ref STRING_RE: Regex =
                Regex::new("^\"(.*?)\"|^'(.*?)'").unwrap();
            static ref NUMBER_RE: Regex =
                Regex::new("^(-?\\d+)").unwrap();
            static ref VARIABLE_RE: Regex =
                Regex::new("(^[a-zA-Z_][a-zA-Z_0-9-]*)").unwrap();
            static ref ASSIGNMENT_RE: Regex =
                Regex::new("(^[a-zA-Z_][a-zA-Z_0-9-]*)\\s*=\\s*([^\\s].*)").unwrap();
            static ref PLUS_RE: Regex =
                Regex::new(r"(.+?)\s*\+\s*(.+)").unwrap();
        }

        if ASSIGNMENT_RE.is_match(expr) {
            let cap = ASSIGNMENT_RE.captures(expr).unwrap();

            let value = self.interpret_expression(&cap[2]);
            self.variables.insert(cap[1].to_string(), value.cloned());

            return value;

        } else if PLUS_RE.is_match(expr) {
            let cap = PLUS_RE.captures(expr).unwrap();

            let left_p: i32 = cap[1].chars().map(
                |c| if c == '(' { 1 } else if c == ')' { -1 } else { 0 }
                ).sum();

            let right_p: i32 = cap[2].chars().map(
                |c| if c == '(' { 1 } else if c == ')' { -1 } else { 0 }
                ).sum();

            if left_p == 0 && right_p == 0 {
                return self.interpret_expression(&cap[1])
                    .plus(self.interpret_expression(&cap[2]));
            }
        }

        if FUNCTION_CALL_RE.is_match(expr) {
            let cap = FUNCTION_CALL_RE.captures(expr).unwrap();

            self.interpret_functioncall(&cap[1], &cap[2])

        } else if STRING_RE.is_match(expr) {
            let cap = STRING_RE.captures(expr).unwrap();
            let contents = cap.get(1).map_or("", |m| m.as_str()).to_string() +
                cap.get(2).map_or("", |m| m.as_str());

            Box::new(CogString{data: contents})

        } else if NUMBER_RE.is_match(expr) {
            let cap = NUMBER_RE.captures(expr).unwrap();

            Box::new(CogInt{data: cap[1].parse().unwrap()})

        } else if VARIABLE_RE.is_match(expr) {
            let cap = VARIABLE_RE.captures(expr).unwrap();

            match self.variables.get(&cap[1]) {
                Some(v) => (*v).cloned(),
                None => panic!("No such variable: '{}'", &cap[1])
            }

        } else if expr == "" {

            Box::new(CogNone)

        } else {
            panic!("Error parsing expression: '{}'", expr)
        }
    }

    fn interpret_functioncall(&mut self, function: &str, args: &str) -> Box<dyn CogObject> {
        let argv = self.interpret_expression(args);
        match self.variables.get_mut(function) {
            Some(f) => f.call(argv),
            None => panic!("No varible named '{function}'")
        }
    }

    fn execute_script(&mut self, file_name: &str) {
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines().filter_map(|l| l.ok()) {
            self.interpret_statement(&line);
        }
    }
}


fn cog_print(arg: Box<dyn CogObject>) -> Box<dyn CogObject> {
    println!("{}", arg.to_string());
    Box::new(CogNone)
}

fn main() {
    let mut interpreter = Interpreter::new();
    interpreter.execute_script("prototype.co");
}
