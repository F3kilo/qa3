use std::any::Any;

fn main() {
    let boxed = Box::new(42_i32);

    let boxed_any: Box<dyn Any> = boxed;

    let bad_downcast: Option<&String> = boxed_any.downcast_ref();
    assert!(bad_downcast.is_none());

    let good_downcast: &i32 = boxed_any.downcast_ref().unwrap();
    dbg!(good_downcast);

    // --------

    let array_of_anything: [Box<dyn Any>; 3] = [Box::new(42), Box::new("hello"), Box::new(true)];
    for item in array_of_anything {
        dbg!(item.downcast_ref::<&str>());
    }
}
