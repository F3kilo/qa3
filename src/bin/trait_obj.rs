use std::fmt::Display;

fn main() {
    let val = 42;
    make_report_static(&val);
    make_report_dynamic(&val);
    make_report_dynamic_boxed(Box::new(val));

    let s = StringOrBool::String(String::from("hello enum"));
    make_report_static(&s);
    // make_report_dynamic(&s);
    // make_report_dynamic_boxed(Box::new(s));

    let s = StringOrBool::Bool(true);
    make_report_static(&s);
    // make_report_dynamic(&s);
    // make_report_dynamic_boxed(Box::new(s));

    let s = String::from("hello closure");
    let closure_report = || s.clone();
    make_report_static(&closure_report);
    make_report_dynamic(&closure_report);
    make_report_dynamic_boxed(Box::new(closure_report));

    let fn_report = get_report;
    make_report_static(&fn_report);
    make_report_dynamic(&fn_report);
    make_report_dynamic_boxed(Box::new(fn_report));
}

pub struct StringWrapper(String);

trait InfoProvider {
    type Output: ToString;

    fn provide(&self) -> Self::Output;

    // fn provide_static<T: ToString>(&self) -> T;
}

impl InfoProvider for i32 {
    type Output = String;

    fn provide(&self) -> Self::Output {
        self.to_string()
    }
}

fn make_report_static(info: &impl InfoProvider) {
    let provided = info.provide();
    let message = provided.to_string();
    println!("Report: state is: {message}")
}

fn make_report_dynamic(info: &dyn InfoProvider<Output = String>) {
    let message = info.provide();
    println!("Report: state is: {message}")
}

fn make_report_dynamic_boxed<'a>(info: Box<dyn InfoProvider<Output = String> + 'a>) {
    let message = info.provide();
    println!("Report: state is: {message}")
}

// ------------------

enum StringOrBool {
    String(String),
    Bool(bool),
}

impl InfoProvider for StringOrBool {
    type Output = Box<dyn Display>;

    fn provide(&self) -> Self::Output {
        match self {
            StringOrBool::String(s) => Box::new(s.clone()),
            StringOrBool::Bool(b) => Box::new(*b),
        }
    }
}

// ------------------

impl<T: Fn() -> String> InfoProvider for T {
    type Output = String;

    fn provide(&self) -> Self::Output {
        self()
    }
}

fn get_report() -> String {
    String::from("hello fn")
}
