use std::time::Instant;

fn main() {
    let fast_op_result = measure(|| 3 + 2);
    dbg!(fast_op_result);

    let long_op_result = measure(|| {
        let mut i = 0;
        while i < 100_000_000 {
            i += 1;
        }
        i
    });
    dbg!(long_op_result);

}

fn measure<F, R>(f: F) -> R 
where
    F: FnOnce() -> R
{
    let before = Instant::now();
    let result = f();
    let after = Instant::now();
    let duration = after - before;
    println!("Duration: {}", duration.as_secs_f32());
    result
}

pub struct InnerValue {}

impl InnerValue {
    pub fn a(&self) {}
    pub fn lot(&self) {}
    pub fn of(&self) {}
    pub fn methods(&self) {}
}