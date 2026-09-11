#[allow(unused)]
fn main() {
   //8.1 println!("Hello, world!");
   let mut a:[i8;5]=[5,4,6,7,10];
   println!("Before changing some values of the array");

   for i in 0..a.len(){
    println!("This is number {}", a[i]);
   }

   //8.2 Tuples
   let t:(i8,bool)=(5,true);
   dbg!(t.0);
   dbg!(t.1);
   // fields of a tuple can be accessed by the period notation
   //Array Iteration
   let mut names:[&str;5]=["Seth","John","Daniel","Ama","Kofi"];
   for i in 0..names.len(){
    if names[i].len() < 5 {
        println!("This had length of {} and the actual name is {}",names[i].len(),names[i]);
    }
   }
   // modern iteration
   let mut colors:[&str;5]=["Red","Green","Blue","Yellow","Black"];
   for color in colors.iter(){
    println!("This is color {}",color);
   };
// 8.4 Pattern and Dstructuring
let tuple = (1,5,3);
println!(
    "(tuple {:?}): {}",
    tuple,
    if check_order(tuple) {"ordered"} else {"not ordered"}
);
let matrix = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]   
];
let transposed = transpose(matrix);

println!("Original matrix:");
for row in matrix.iter() {
    println!("  {:?}", row);
}

println!("\nTransposed matrix:");
for row in transposed.iter() {
    println!("  {:?}", row);
}
}
fn check_order(tuple:(i32,i32,i32))->bool{
    let(left,middle,right) = tuple;
    left < middle && middle < right
} 

// Exercise 8.1 Nested Arrays
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transposed = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            transposed[j][i] = matrix[i][j];
        }
    }
    transposed
}