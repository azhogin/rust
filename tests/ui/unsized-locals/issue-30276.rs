struct Test([i32]);

fn main() {
    let _x: fn(_) -> Test = Test;
    //~^ ERROR the size for values of type `[i32]` cannot be known at compilation time
}
