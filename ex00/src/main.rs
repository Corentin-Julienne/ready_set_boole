/* adder is a function that uses a half adder */
fn adder(a:u32, b:u32)->u32 {
    let mut carry: u32;
    let mut a_cpy: u32 = a;
    let mut b_cpy: u32 = b;

    while b_cpy != 0 {
        carry = a_cpy & b_cpy;

        // Sum of bits of `a` and `b` where at least one of the bits is not set.
        a_cpy = a_cpy ^ b_cpy;

        // Carry is shifted by one so that adding it to `a` gives the required sum.
        b_cpy = carry << 1;
    }
    return a_cpy;
}

fn main() {
    let num1: u32 = 700;
    let num2: u32 = 1400;

    let sum: u32 = adder(num1, num2);
    println!("The sum of {} and {} is {}", num1, num2, sum);
}
