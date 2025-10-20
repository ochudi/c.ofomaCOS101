fn main () {
    let p:f32 = 1000.00;
    let r:f32 = 1.00;
    let t:f32 = 2.00;

    // simple interest
    let a = p * ( 1.00 + (r / 100.00)) * t;
    println! ("Amount is ${}", a);

    let si = a - p;
    println! ("Simple Interest is ${}", si);
}