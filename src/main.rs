/// statement := <comment> | <expression>
/// expression := <functioncall> | <string>
/// name := [a-zA-Z_][a-zA-Z_0-9-]*
/// comment := //.*
/// functioncall := <name>(<expression>)
/// string := ".*?" | '.*?'
///
///
/// TODO:
///     Variables
///
///     Numbers
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

trait CogObject {

    fn to_string(&self) -> String;

    fn call(&self, _arg: Box<dyn CogObject>) -> Box<dyn CogObject> {
        panic!("Object is not callable!")
    }
}

struct CogString {
    data: String
}

impl CogObject for CogString {

    fn to_string(&self) -> String {
        self.data.clone()
    }
}

struct CogNone;

impl CogObject for CogNone {

    fn to_string(&self) -> String {
        "None".to_string()
    }
}

struct CogInt {
    data: i128
}

impl CogObject for CogInt {

    fn to_string(&self) -> String {
        self.data.to_string()
    }
}

struct CogFn {
    data: fn(Box<dyn CogObject>) -> Box<dyn CogObject>
}

impl CogObject for CogFn {

    fn to_string(&self) -> String {
        "Function".to_string()
    }

    fn call(&self, arg: Box<dyn CogObject>) -> Box<dyn CogObject> {
        (self.data)(arg)
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

    fn interpret_statement(&self, line: &str) {
        lazy_static! {
            static ref COMMENT_RE: Regex = Regex::new("//.*").unwrap();
        }

        if COMMENT_RE.is_match(line) {
            println!("Comment: {}", line);
        } else {
            self.interpret_expression(line);
        }
    }

    fn interpret_expression(&self, expr: &str) -> Box<dyn CogObject> {
        lazy_static! {
            static ref FUNCTION_CALL_RE: Regex =
                Regex::new("(^[a-zA-Z_][a-zA-Z_0-9-]*)\\((.*)\\)").unwrap();
            static ref STRING_RE: Regex = Regex::new("^\"(.*?)\"|^'(.*?)'").unwrap();
        }

        if FUNCTION_CALL_RE.is_match(expr) {
            let cap = FUNCTION_CALL_RE.captures(expr).unwrap();

            self.interpret_functioncall(&cap[1], &cap[2])

        } else if STRING_RE.is_match(expr) {
            let cap = STRING_RE.captures(expr).unwrap();
            let contents = cap.get(1).map_or("", |m| m.as_str()).to_string() +
                cap.get(2).map_or("", |m| m.as_str());

            Box::new(CogString{data: contents})

        } else if expr == "" {
            Box::new(CogNone)
        } else {
            panic!("Error parsing expression: '{}'", expr)
        }
    }

    fn interpret_functioncall(&self, function: &str, args: &str) -> Box<dyn CogObject> {
        match self.variables.get(function) {
            Some(f) => f.call(self.interpret_expression(args)),
            None => panic!("No varible named '{function}'")
        }
    }
}


fn cog_print(arg: Box<dyn CogObject>) -> Box<dyn CogObject> {
    println!("{}", arg.to_string());
    Box::new(CogNone)
}

fn main() {
    let interpreter = Interpreter::new();

    interpreter.interpret_statement("// Comment");
    interpreter.interpret_statement("print(print())");
    interpreter.interpret_statement("print('')");
    interpreter.interpret_statement("print(\"Hello world\")");
    interpreter.interpret_statement("print('Hello world')");
//    interpret_statement("9print('Hello world')");
}
