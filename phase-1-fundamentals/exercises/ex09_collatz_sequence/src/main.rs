fn main() {
    println!("Exercise: Collatz Sequence");
    let myarr = [3,5,7];
    for i in myarr{
        println!("The Length of the Collatz Sequence of {i} is {}", collatz_sequence(i));
    }
}
fn collatz_sequence(mut n:i32)->i32{
    let mut len = 1;
    while n > 1 {
        n = if n%2 == 0 {n/2} else {3*n + 1};
        len += 1;
    }len
}