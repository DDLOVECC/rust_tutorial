fn main() {
    let numbers = &[2, 3, 4, 5, 800];
    let result1 = sum(numbers);

    match result1 {
        Some(sum) => println!("Sum of {:?} is:{}",numbers, sum),
        None => println!("Overflow"),
    }

    let new_numbers = &[1,u32::MAX];
    let result2 = sum(new_numbers);
    match result2 {
        Some(sum) => println!("Sum of {:?} is:{}",numbers, sum),
        None => println!("Overflow"),
    }
}

fn sum(numbers: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for &num in numbers {
        if sum > u32::MAX {
            return None;
        }
        sum += num;
    }
    Some(sum)
}