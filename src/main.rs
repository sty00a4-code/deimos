use std::{cell::RefCell, env, fs, process::exit, rc::Rc};

use luna_rs::{
    compile_str,
    lang::{code::Closure, value::{Function, Value}},
    luna_impl::{interpreter::{Interpreter, RunTimeError}, position::Located},
};
use translation::insert_module;

pub mod translation;

fn main() {
    let mut args = env::args().skip(1);
    if let Some(path) = args.next() {
        let text = fs::read_to_string(&path)
            .map_err(|err| {
                eprintln!("ERROR: {err}");
                exit(1);
            })
            .unwrap();
        let closure = compile_str(&text)
            .map_err(|Located { value: err, pos }| {
                eprintln!(
                    "ERROR {path}:{}:{}: {err}",
                    pos.ln.start + 1,
                    pos.col.start + 1
                );
                exit(1);
            })
            .unwrap();
        run(closure).map_err(|Located { value: err, pos }| {
            eprintln!(
                "ERROR {path}:{}:{}: {err}",
                pos.ln.start + 1,
                pos.col.start + 1
            );
            exit(1);
        })
        .unwrap();
    } else {
        eprintln!("{}", USAGE);
        exit(1);
    }
}
pub const USAGE: &str = r#"USAGE:
    deimos <input.luna> - runs the luna file
"#;

pub fn run(closure: Rc<RefCell<Closure>>) -> Result<Option<Value>, Located<RunTimeError>> {
    let mut interpreter = Interpreter::default();
    insert_module(&mut interpreter.globals.borrow_mut());
    interpreter.call(&Rc::new(Function {
        closure,
        upvalues: vec![]
    }), vec![], None);
    interpreter.run()
}

