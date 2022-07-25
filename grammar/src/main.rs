use grammar::Summary; // train必须引入到作用域才能使用
use grammar::Tweet;
use grammar::NewsArticle;
mod declaration_cycle; // 通过这样方式，rust会从src根目录下，加载同名文件作为modul引进来
pub use crate::declaration_cycle::cycle; // 绝对应用模块中的模块
const MAX_POINTS: u32 = 100_100;

fn main() {
    println!("Hel&&&lo, world!");

    //let one = "1";
    let one = 2; //& shadown

    println!("常量{}", MAX_POINTS);
    println!("shadown {}", one);

    // let inter: u8 = 256;

    // let t = true;

    let c: char = '1';

    println!("emjoy {}", c);

    // tup类型
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("{},{},{}", tup.0, tup.1, tup.2);

    // 数组
    // vector
    let months = [
        "12",
        "32",
        "232",
    ];
    println!("数组{}",months[0]);

    let student:[u32; 2] = [1,2];

    println!("数组访问 {}", student[1]);

    // 函数
    my_first_demon(1,2);

    // 块,块里面最后表达式就是块返回的值
    let y = {
        let x = 1;
        x + 3
    };
    
    let five = five();
    println!("five: {}", five);

    string_demon();

    cache_rule();

    // 函数与所有权规则
    let s = String::from("Hello World!"); // 可变变量

    take_ownership(s); // s从main的作用域移动到了函数里，此时s在main作用域是不可用了,如果在下面使用会报错

    let w = String::from("Hello World other !");
    let other_s = take_return_ownership(w); // 返回值就会移出了函数作用域，other_s和w不是同样的值了，如果要相同，就是用引用方式&
    // rust的引用是&，此时，不会获取到对应值的所有权，所以也就rust不会因为move了w而废弃w，导致w不可用。
    println!("{}",other_s);

    // 可变数组 slice
    let mut s = String::from("Hello World");

    let word_index = search_str(&mut s);
    println!("{}", word_index);

    slice_type();

    // 结构体的使用
    let rect = Rectangle {
        width: 30,
        length: 50,
    };

    println!("{}", area(&rect));
    println!("{:#?}", rect);

    println!("{}", rect.area());
    
    let square = Rectangle::square(12);
    println!("{:#?}", square);

    let c = Coin::Quarter(UsStates::Alaska);
    println!("{}", value_in_cents(c));


    // 泛型
    let p1 = Point {x:5, y:4.0};
    let p2 = Point{x:"hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // train
    let tweet = Tweet{
        username:String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle{
        headline: String::from("horse_ebooks"),
        location: String::from("广州-广电云平"),
        author: String::from("Jack"),
        content: String::from("this is test"),
    };

    let tweetv2 = Tweet{
        username:String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    grammar::notify(tweet);
    grammar::notifyv2(article);

    println!("{:?}", grammar::notifyv5(tweetv2));


    cycle::demon();

}

//rust的函数命名规则是snake case，小写和下划线链接

fn my_first_demon(x: i32, y:i32) {
    println!("the value of x is: {} y is: {}", x, y);
}
// 函数返回值
// 在->符号后边声明函数返回值类型，但是不可以为返回值命名
// 在rust里，返回值就是函数体最后一个表达式值
// 若想提前返回，需使用return关键词，并指定一个值，大多数使用最后一个表达式作为返回值

fn five() -> i32 {
    5
}
// 控制流
// if else
// match

fn if_else_match() {
    let condition = true;
    let number = if condition {2} else {1};
    println!("分支结果{}", number);
}

// loop循环

// while

// for遍历集合
fn for_set() {
    let a = [10, 20, 30];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // {1..2}就是range
    for number in (1..2).rev() {
        println!("{}!", number);
    }
}

// 所有权系统
// 解决问题：跟踪代码的那些部分正在使用heap的那些数据,rust没有gc
// 最小化heap上的重复数据量
// 清理heap上未使用的数据以避免空间不足
// 所有权规则

// 创建String类型和字面值类型区别，String类型可变，字面值类型不可变
fn string_demon() {
    let immutable_string = "immutable string";
    let mut variable_string = String::from("variable string");
    variable_string.push_str(" world");
    println!("不可变字符串字面值{}", immutable_string);
    println!("输出一个可变类型字符串{}", variable_string);
}


// rust 对于离开作用域的变量会调用drop函数交回内存给操作系统
// rust对于基础数据类型，内存是存放在stack上，存在stack上数据可以实现复制，复制后数据，原来数据还是可用
// rust对可变类型，存放在heap上，stack上只是存执行heap的指针、长度和容量，heap存放具体内容，rust对于移动后的可变类型，原来的类型就会被废弃，从而防止二次回收heap导致问题。
// clone()、copy()、drop() move()涉及到内存的几个重点函数

fn cache_rule() {
    let x = 1;
    let y = x; // 基础类型，这样就实现了copy在stack上数据，x还有值
    println!("x: {} y: {}",x,y);

    let immutable_string = String::from("variable string");
    let new_immutable_string = immutable_string; // 可变类型，这样就是move，rust会废弃immutable_string的stack的数据，这样就不会出现二次回收问题

    println!("new_immutable_string: {}", new_immutable_string);
}

// 所有权与函数

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn take_return_ownership(some_string: String) -> String {
    some_string
}

fn take_ownership_other(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_number:i32) {
    println!("{}", some_number);
}
// 把不可变的引用作为函数参数这个行为叫做借用
// 不可用修改借用的东西
// 可变引用，在特定作用域内，只能存在一个可变引用，好处是，在编译时候，防止数据竞争
fn search_str(s: &mut str) -> &str { // 字符串切片类型
    //s.push_str("string"); // 不可修改不可变的引用值。除非是可变mute变量
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
// 切片类型定义 &str

fn slice_type() {
    let a = [1,2,3]; // 数组
    let slice_a = &a[1..]; // 切片
    let b = &[1,2,3];
    let c = &b[..];
    let (x,y) = (b[0],b[1]);
    println!("i32切片第一个元素{:?}", slice_a[0]);
    println!("{:?}",c);
    println!("{}:{}",x,y);
}


// struct结构体

#[derive(Debug)] // 派生结构体实现debug的train
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn define_struct() -> User {
    // 可变的struct实例，实例中所有字段都是可变的
    let mut user_instance = User {
        email: String::from("someone@example.com"),
        name: String::from("someusername123"),
        active:true,
        sign_in_count: 1,
    };
    user_instance.name = String::from("someoneothername");
    return user_instance;
}
#[derive(Debug)]
struct Rectangle {
    width: u16,
    length: u16,
}

fn area(rect: &Rectangle) -> u16 {
    rect.width * rect.length
}


// 为结构体定义方法
impl Rectangle {
    fn area(&self) -> u16 {
        self.width * self.length
    }

    // 关联函数，关联函数是通过::使用
    fn square(size: u16) -> Rectangle {
        Rectangle { width: size, length: size }
    }
}

enum IpAddrKind {
    V4(u8,u8,u8,u8), // 括号内是指定类型，也可以是结构体类型
    V6(String),
}

// 枚举与模式匹配
fn emume_value() {

    let four = IpAddrKind::V4(127,0,0,1);
    let six = IpAddrKind::V6(String::from("::1"));
    route(four);
    route(six);
}

fn route(ip_kind: IpAddrKind) {
    
}

// rust 提供与null概念的枚举 Option<T>

// match 类型go中switch，用于简化多分支流程控制
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // 匹配到penny值就执行 =>后逻辑
        Coin::Penny => {
            println!("{}", "penny");
            1
        },
        Coin::Nickel => {
            println!("{}", "Nickel");
            2
        },
        Coin::Dime => {
            println!("{}", "Dime");
            3
        },
        Coin::Quarter(state) => {
            println!("{:#?}", state);
            4
        }
    }
}
#[derive(Debug)]
enum UsStates {
    Alabama,
    Alaska,
}

// 范型
// 作用：提高代码复用能力
// 泛型函数

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// struct定义中使用泛型
struct Point<T, U> {
    x:T,
    y:U,
}

// Enum定义中的泛型
enum Option<T> {
    Some(T),
    None,
}
// 方法定义中定义泛型
// 实现泛型结构体方法，其中方法函数的泛型可用与结构体泛型不一样
// 单态化
impl <T, U>Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point { x:self.x, y: other.y }
    }
}

//