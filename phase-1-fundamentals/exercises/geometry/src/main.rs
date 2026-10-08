fn main(){
    // Exercise: Geometry, given coordinates and normalizing them to get the outputs by finding the sum pf their squares and taking the square roots of them 
    let mut vector = [11.0,12.0,13.0];
    println!("The given vector is {vector:?} and their magnitude before normalizing is {}",magnitude(&vector));
    normalize(&mut vector);
    println!("{:?}",&vector);
    println!("Their magnitude after normalizing is {}",magnitude(&vector));
}
fn magnitude(vector:&[f64;3])->f64{
    let mut mag = 0.0;
    for item in vector{
        mag += item.powf(2.0);
    }
    mag.sqrt()
}
fn normalize(vector: &mut[f64;3]){
    let mag = magnitude(vector);
    for item in vector{
        *item /= mag;
    }
}