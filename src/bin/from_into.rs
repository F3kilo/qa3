use std::fmt::Display;

fn main() {
    let string = String::from("Hello, world!");
    let wrapped = string.into();
    print_wrapper(wrapped);

    let string = String::from("Hello, rust!");
    let string_ref = &string;
    let wrapped = string_ref.into();
    print_wrapper(wrapped);

    let value = 42;
    let wrapped = value.into();
    print_wrapper(wrapped);
}

fn print_wrapper(wrapped: StringWrapper) {
    println!("wrapped string debug output: {wrapped:?}");
    println!("wrapped string display output: {wrapped}");
}

#[derive(Debug)]
struct StringWrapper(String);

// From string into wrapper
impl From<String> for StringWrapper {
    fn from(inner: String) -> Self {
        Self(inner)
    }
}

// From string reference into wrapper
impl From<&String> for StringWrapper {
    fn from(inner: &String) -> Self {
        Self(inner.clone())
    }
}

// From value into wrapper
impl From<i32> for StringWrapper {
    fn from(value: i32) -> Self {
        Self(value.to_string())
    }
}

// From any type which implements Display into wrapper
// impl<T: ToString> From<T> for StringWrapper {
//     fn from(value: T) -> Self {
//         Self(value.to_string())
//     }
// }

// From wrapper into string
impl From<StringWrapper> for String {
    fn from(inner: StringWrapper) -> Self {
        inner.0
    }
}

impl Display for StringWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "displaying the inner string: {}", self.0)
    }
}
