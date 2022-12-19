use std::borrow::{Borrow, BorrowMut, Cow};

fn main() {
    let s = MyStringWrapper::new(MyString::new(String::from("hello")));

    let mut cow = Cow::Borrowed(s.borrow());

    dbg!(&cow);
    maybe_update(&mut cow, false);
    dbg!(&cow);
    maybe_update(&mut cow, true);
    dbg!(&cow);
    maybe_update(&mut cow, true);
    dbg!(&cow);

    dbg!(&s);
    dbg!(cow.into_owned());
}

#[derive(Debug)]
struct MyStringWrapper(MyString);

impl MyStringWrapper {
    pub fn new(inner: MyString) -> Self {
        Self(inner)
    }

    pub fn push(&mut self, ch: char) {
        self.0.push(ch)
    }
}

#[derive(Debug)]
struct MyString(String);

impl MyString {
    pub fn new(inner: String) -> Self {
        Self(inner)
    }

    pub fn push(&mut self, ch: char) {
        self.0.push(ch)
    }
}

impl Borrow<MyString> for MyStringWrapper {
    fn borrow(&self) -> &MyString {
        println!("borrow called");
        &self.0
    }
}

impl BorrowMut<MyString> for MyStringWrapper {
    fn borrow_mut(&mut self) -> &mut MyString {
        println!("borrow_mut called");
        &mut self.0
    }
}

impl ToOwned for MyString {
    type Owned = MyStringWrapper;

    fn to_owned(&self) -> Self::Owned {
        println!("to owned called");
        let new_my_string = Self::new(self.0.clone());
        MyStringWrapper::new(new_my_string)
    }
}

fn maybe_update(val: &mut Cow<MyString>, need_update: bool) {
    if need_update {
        val.to_mut().push('1');
    }
}
