fn main(){
    println!("************* Pixel Matrix Grid *************");
    let myarr:[[i32;5];5] = [[1,1,1,0,0],[0,0,1,1,0],[1,1,1,0,0],[0,0,1,1,0],[1,1,0,0,1]];
    for i in 0..myarr.len() {
        println!("This is array[{}] = {:?}",i+1,myarr[i]);
    }
    println!("The total number of ones in it is {}",count_lit_pixels(&myarr));
}
fn count_lit_pixels(myarr: &[[i32;5];5])->i32{
    let mut total_ones = 0;
    for i in 0..5{
        for j in 0..5{
            if myarr[i][j] == 1 {total_ones += 1}
        }
    }total_ones
}