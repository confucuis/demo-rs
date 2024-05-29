use std::fmt::Debug;
use std::ops::{Add, Mul, Sub, Div};

pub enum Arithmetic {
    Add,
    Sub,
    Mul,
    Div,
}

pub fn arithmetic<T>(num: T, ops: Arithmetic) -> T
where
    T: Copy + Add<Output=T> + Mul<Output=T> + Sub<Output=T> + Div<Output=T> + Debug,
{
    let add = |item: T| -> T { num + item };
    let sub = |item: T| -> T { num - item };
    let mul = |item: T| -> T { num * item };
    let div = |item: T| -> T { num / item };

    match ops {
        Arithmetic::Add => return add(num),
        Arithmetic::Sub => return sub(num),
        Arithmetic::Mul => return mul(num),
        Arithmetic::Div => return div(num),
    }

    // let result = add(num);
    // println!("Add result: {:?}", result);
    // let result = sub(num);
    // println!("Sub result: {:?}", result);
    // let result = mul(num);
    // println!("Mul result: {:?}", result);
    // let result = div(num);
    // println!("Div result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic() {
       assert_eq!(arithmetic(5u32, Arithmetic::Add), 10);
       assert_eq!(arithmetic(5u32, Arithmetic::Sub), 0);
       assert_eq!(arithmetic(5u32, Arithmetic::Mul), 25);
       assert_eq!(arithmetic(5u32, Arithmetic::Div), 1);
    }
}
