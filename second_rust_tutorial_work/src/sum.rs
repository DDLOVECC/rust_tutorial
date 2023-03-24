fn main(){
   let numbers =[2,3,4,5,8];
    match  sum(&numbers){
        Some(result) => println!("Sum of is:{}",result),
        None(result) => println!("Overflow"),
    }
}

fn sum(numbers:&[u32])->Option<u32>{
    let mut result = 0u32;
    for &num in numbers {
       match sum.checked_add(num) {
           Some(result) =>sum = result,
           None => return None,
       }
    }
    some(result)
}