fn main(){
    let pair1: [usize; 2] = [270, 192];
    println!("GCD For [270, 192] = {}", find_gcd(pair1[0], pair1[1]));
    let pair2: [usize; 2] = [1680, 640];
    println!("GCD For [1680, 640] = {}", find_gcd(pair2[0], pair2[1]));
    let pair3: [usize; 2] = [640, 1680];
    println!("GCD For [640, 168] =  {}", find_gcd(pair3[0], pair3[1]));

}

fn find_gcd(a: usize, b: usize)-> usize{
    let quotient;
    let remainder;
    if a > b {
        quotient = b;
        remainder = a % b;
    } else if b > a {
        quotient = a;
        remainder = b % a;
    } else {
        quotient = a;
        remainder = a % b;
    }
    // check if either a or b is zero
    if quotient == 0 || remainder == 0 {
        if quotient == 0 {
            remainder
        } else {
            quotient
        }
    } else {
        find_gcd(quotient, remainder)
    }
}
