trait Primeter {
    type Item;
    fn primeter(&self) -> Self::Item;
}

trait Extend {
    type Item;
    fn extend(&self) -> Self::Item;
}

#[derive(Clone)]
struct Circle {
    radius: f64,
}

impl Primeter for Circle {
    type Item = f64;
    fn primeter(&self) -> Self::Item {
        2.0 * 3.14 * self.radius
    }
}

impl Extend for Circle {
    type Item = f64;
    fn extend(&self) -> Self::Item {
        3.14 * self.radius * self.radius
    }
}

#[derive(Clone)]
struct Square {
    side: u64,
}

impl Primeter for Square {
    type Item = u64;
    fn primeter(&self) -> Self::Item {
        4 * self.side
    }
}

impl Extend for Square {
    type Item = u64;
    fn extend(&self) -> Self::Item {
        self.side * self.side
    }
}

fn calculations_primeter<T: Primeter>(shap: &T) -> T::Item {
    shap.primeter()
}

fn calculations_extend<T: Extend>(shap: &T) -> T::Item {
    shap.extend()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let circle = Circle { radius: 2.5 };
        let square = Square { side: 4 };
        let circle_primter = calculations_primeter(&circle);
        let square_primter = calculations_primeter(&square);
        let circle_extend = calculations_extend(&circle);
        let square_extend = calculations_extend(&square);

        assert_eq!(format!("{:.1}", circle_primter), "15.7");
        assert_eq!(square_primter, 16);
        assert_eq!(format!("{:.1}", circle_extend), "19.6");
        assert_eq!(square_extend, 16);
    }
}
