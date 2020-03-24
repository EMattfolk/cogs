/// statement := <comment> | <expression>
/// expression := <functioncall> | <string>
/// name := [a-zA-Z_0-9-]+?
/// comment := //.*
/// functioncall := <name>(<expression>)
/// string := ".*?" | '.*?'
///

#[macro_use]
extern crate lazy_static;

use regex::Regex;

trait Object {

    fn to_string(&self) -> &str;
}

struct CogString {
    data: String
}

impl Object for CogString {

    fn to_string(&self) -> &str {
        return &self.data;
    }
}

struct CogNone;

impl Object for CogNone {

    fn to_string(&self) -> &str {
        return "None";
    }
}

fn interpret_statement(line: &str) {
    lazy_static! {
        static ref COMMENT_RE: Regex = Regex::new("//.*").unwrap();
    }

    if COMMENT_RE.is_match(line) {
        println!("Comment: {}", line);
    } else {
        interpret_expression(line);
    }
}

fn interpret_expression(expr: &str) -> Box<dyn Object> {
    lazy_static! {
        static ref FUNCTION_CALL_RE: Regex = Regex::new("([a-zA-Z_0-9-]+?)\\((.*?)\\)")
            .unwrap();
        static ref STRING_RE: Regex = Regex::new("\"(.*?)\"|'(.*?)'").unwrap();
    }

    if FUNCTION_CALL_RE.is_match(expr) {
        let cap = FUNCTION_CALL_RE.captures(expr).unwrap();
        return interpret_functioncall(&cap[1], &cap[2]);
    } else if STRING_RE.is_match(expr) {
        let cap = STRING_RE.captures(expr).unwrap();
        let contents = cap.get(1).map_or("", |m| m.as_str()).to_string() +
            cap.get(2).map_or("", |m| m.as_str());
        return Box::new(CogString{data: contents});
    } else if expr == "" {
        Box::new(CogNone{})
    } else {
        panic!("Error parsing expression: '{}'", expr)
    }
}

fn interpret_functioncall(function: &str, args: &str) -> Box<dyn Object> {
    match function {
        "print" => println!("{}", interpret_expression(args).to_string()),
        &_ => {}
    }

    Box::new(CogNone{})
}

fn main() {
    interpret_statement("// Comment");
    interpret_statement("print()");
    interpret_statement("print('')");
    interpret_statement("print(\"Hello world\")");
    interpret_statement("print('Hello world')");
}
