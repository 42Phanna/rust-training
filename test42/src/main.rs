fn adder(a: u32, b: u32) -> u32 {
    let carry = (a & b) << 1;
    let result = a ^ b;
    if carry == 0 {
       result
    }
    else {
        adder(carry, result)
    }
}

fn multiply_two_numbers(mut a: u32, mut b: u32) -> u32 {
    let mut result: u32 = 0;
    while b > 0 {
       if b & 1 > 0 {
          result = adder(result, a);
          }
       a = a << 1;
       b = b >> 1;
    }
    result
 }

fn main(){
    let a: u32 = 11;
    let b: u32 = 12;
    let c: u32 = adder(a, b);
    let d: u32 = multiply_two_numbers(a, b);
    println!("The sum of {a} and {b} using bitwise adding is {c}");
    println!("The sum of {a} and {b} using bitwise adding is {d}");
}