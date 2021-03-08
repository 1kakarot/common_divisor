fn main() {
    let mut a = 15;
    let mut b = 400;
    while b != 0 {
        let temp = b;
        println!("a = {}",a);
        println!("b = {}",b);
        b = a % b;
        println!("a%b = {}",b);
        a = temp;
  }
    println!("Greatest common divisor of 15 and 40 is: {}", a);
}
