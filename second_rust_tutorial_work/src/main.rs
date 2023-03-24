fn main() {
    let red =RedLight{};
    let yellow = YellowLight{};
    let green = GreenLight{};
    println!("Red Light Time :{}",red.time());
    println!("Yellow Light Time :{}",yellow.time());
    println!("Green Light Time :{}",green.time());
}


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


