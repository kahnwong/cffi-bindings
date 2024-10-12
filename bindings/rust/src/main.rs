extern "C" {
    fn multiply(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32;
}

fn main() {
    let a = 25;
    let b = 30;
    let c = 35;
    let d = 54;
    let e = 654;

   let result = unsafe { multiply(a,b,c,d,e) };
    println!("{:?}", result);
}
