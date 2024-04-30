### trait bound

* *结构体0x01*  
```
// 定义一个泛型结构体，其中的泛型参数必须同时实现Debug和Clone trait
#[derive(Debug, Clone)]
struct Pair<T: Debug + Clone> {
    first: T,
    second: T,
}
```

* *结构体0x02*
```
// 定义一个结构体，其中的泛型参数必须同时实现Debug和Clone trait
#[derive(Debug, Clone)]
struct Pair<T, U>
where
    T: std::fmt::Debug + Clone,
    U: std::fmt::Debug + Clone,
{
    first: T,
    second: U,
}
```

* *函数0x01*
```
// 定义一个函数show，该函数接受两个泛型参数，并要求它们同时实现Debug和Display trait
fn show<T: std::fmt::Debug + std::fmt::Display>(value1: T, value2: T) {
    println!("Value 1: {:?}", value1);
    println!("Value 2: {}", value2);
}
```

* *函数0x02*
```
// 定义一个函数show，该函数接受两个泛型参数，并要求它们同时实现Debug和Display trait
fn show<T, U>(value1: T, value2: U)
where
    T: std::fmt::Debug + std::fmt::Display,
    U: std::fmt::Debug + std::fmt::Display,
{
    println!("Value 1: {:?}", value1);
    println!("Value 2: {}", value2);
}
```

* *类型别名*
```
// 定义一个类型别名 FlyingObject，它表示实现了 Fly trait 的类型
type FlyingObject<T> where T: Fly = T;
```

* *闭包参数*
```
// 定义一个 trait，表示可以进行某种操作
trait Operation {
    fn operate(&self, x: i32, y: i32) -> i32;
}

// 实现 Add trait 的结构体
struct Add;

impl Operation for Add {
    fn operate(&self, x: i32, y: i32) -> i32 {
        x + y
    }
}

// 实现 Subtract trait 的结构体
struct Subtract;

impl Operation for Subtract {
    fn operate(&self, x: i32, y: i32) -> i32 {
        x - y
    }
}

// 定义一个函数，接受一个闭包作为参数，并调用闭包中的操作
fn execute_operation<F>(op: F, x: i32, y: i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    op(x, y)
}

fn main() {
    let result_add = execute_operation(|x, y| x + y, 10, 5);
    let result_subtract = execute_operation(|x, y| x - y, 10, 5);

    println!("Result of addition: {}", result_add);
    println!("Result of subtraction: {}", result_subtract);
}
```

* *生命周期*
```
// 定义一个结构体，包含一个字符串引用字段
struct Container<'a> {
    data: &'a str,
}

// 定义一个 trait，表示可以打印数据
trait PrintData {
    fn print(&self);
}

// 实现 PrintData trait 的结构体
impl<'a> PrintData for Container<'a> {
    fn print(&self) {
        println!("Data: {}", self.data);
    }
}

// 定义一个函数，接受实现了 PrintData trait 的参数，并调用其 print 方法
fn print_data<T: PrintData>(data: T) {
    data.print();
}

fn main() {
    let container = Container { data: "Hello, Rust!" };

    // 调用 print_data 函数，传入实现了 PrintData trait 的参数
    print_data(container);
}
```