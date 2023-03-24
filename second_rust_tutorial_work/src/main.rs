// fn main() {
//     let red =RedLight{};
//     let yellow = YellowLight{};
//     let green = GreenLight{};
//     println!("Red Light Time :{}",red.time());
//     println!("Yellow Light Time :{}",yellow.time());
//     println!("Green Light Time :{}",green.time());
// }


trait TrafficLight{
    fn time(&self)-> u32;
}



struct RedLight{}
struct YellowLight{}
struct GreenLight{}

impl TrafficLight for RedLight{
    fn time(&self)->u32{
        20
    }
}

impl TrafficLight for YellowLight{
    fn time(&self)->u32{
        30
    }
}

impl TrafficLight for GreenLight{
    fn time(&self)->u32{
        40
    }
}

fn main(){
    let numbers =[2,3,4,5,8];
    match  sum(&numbers){
        Some(result) => println!("Sum of is:{}",result),
        None => println!("Overflow"),
    }
}

fn sum(numbers:&[u32])->Option<u32>{
    let mut result = 0u32;
    for &num in numbers {
        match sum.checked_add(num) {
            Some(result) => sum = result,
            None => return None,
        }
    }
    Some(result)
}
