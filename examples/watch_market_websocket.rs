use dotenv::dotenv;
use std::{fmt::Debug, *};

fn for_each_planet<F>(f: F)
where
    F: Fn(&'static str) + 'static,
{
    f("Mercury");
    f("Venus");
    f("Earth");
    f("Mars");
    f("Jupiter");
    f("Saturn");
    f("Uranus");
    f("Neptune");
}

fn foobar<F>(mut f: F)
where
    F: FnMut(i32) -> i32,
{
    // println!("{}", f(f(2)));
    // println!("{}", f(2));
    let tmp = f(2);
    println!("{}", f(tmp));
}

fn foobar_once<F>(f: F)
where
    F: FnOnce() -> String,
{
    println!("{}", f());
}

fn foobar_two_args<F>(x: i32, y: i32, is_greater: F)
where
    F: Fn(i32, i32) -> bool,
{
    let (greater, smaller) = if is_greater(x, y) { (x, y) } else { (y, x) };
    println!("{} is greater than {}", greater, smaller);
}

fn countdown<F>(count: usize, tick: F)
where
    F: Fn(usize),
{
    for i in (1..count).rev() {
        tick(i);
    }
}

fn make_tester(answer: String) -> impl Fn(&str) -> bool {
    move |challenge| challenge == answer
}

fn make_tester1<'a>(answer: &'a str) -> impl Fn(&str) -> bool + 'a {
    move |challenge| challenge == answer
}

fn make_tester2(answer: &str) -> impl Fn(&str) -> bool + '_ {
    move |challenge| challenge == answer
}

#[tokio::main]
async fn main() {
    let test = make_tester("hunter2".into());
    println!("test {}", test("******"));
    println!("test {}", test("hunter2"));

    let test1 = make_tester1("hunter2");
    println!("test1 {}", test1("******"));
    println!("test1 {}", test1("hunter2"));
    /* for c in "Hello, world!"
        .chars()
        .filter(|c| c.is_lowercase())
        .flat_map(|c| c.to_uppercase())
    {
        println!("{}", c);
    } */

    /* for c in "rust".chars() {
        println!("Give me a {}", c);
    } */

    /* for i in &[1, 2, 3] {
        println!("{}", i);
    } */

    /* for i in vec![1, 2, 3, 4, 5] {
        println!("{}", i);
    }

    for i in &vec![1, 2, 3, 4, 5] {
        println!("{}", i);
    } */

    // countdown(1, |_| ());
    // countdown(10, |i| println!("tick {}", i));

    // foobar_two_args(32, 64, |x, y| x > y);
    // foobar_two_args(32, 64, |_, _| panic!("should not be called"));

    /* let s = String::from("alright");
    foobar_once(|| s.clone());
    foobar_once(|| s.clone()); */

    /* let mut acc = 2;
    foobar(|x| {
        // acc = acc * x;
        // acc
        acc += 1;
        x * acc
    }); */

    // for_each_planet(|planet| println!("Hello,{}", planet));
    /* let greeting = String::from("Good to see you");
    for_each_planet(move |planet| println!("{} {}", greeting, planet)); */
}

/* #[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn negate(p: Point) -> Point {
    Point { x: -p.x, y: -p.y }
}

#[tokio::main]
async fn main() {
    let p = Point { x: 1.0, y: 2.0 };
    let p_ref = &p;
    // println!("{:?},{:?}", p_ref.x, p_ref.y);
    negate(*p_ref);
} */

/* async fn main() -> Result<(), std::str::Utf8Error> {
    let s = std::str::from_utf8(&[240, 159, 141, 137])?;
    println!("{}", s);
    /* match std::str::from_utf8(&[240, 159, 141, 137]) {
        Ok(s) => println!("{}", s),
        Err(e) => return Err(e),
    } */
    Ok(())
    /* if let Ok(s) = std::str::from_utf8(&[240, 159, 141, 137]) {
        println!("{}", s);
    } */

    /* let s = std::str::from_utf8(&[240, 159, 141, 137]).unwrap();
    println!("{:?}", s);

    match std::str::from_utf8(&[240, 159, 141, 137]) {
        Ok(s) => println!("{:?}", s),
        Err(e) => panic!("{:?}", e),
    } */

    // let s = std::str::from_utf8(&[195, 40]).unwrap();
    // let s1 = std::str::from_utf8(&[195, 40]).expect("valid utf-8");

    // dotenv().ok();
    /* println!("Hello, world!");
    let x={
        let mut x=1000;
        x+=1;
        x
    };
    println!("{}",x);
    let a=(1,2,3);
    println!("{:?}",a);
    println!("{}",a.0);
    let nick=String::from("nick name");
    println!("{}",nick.len());
    let least=cmp::min(100,15);
    println!("{}",least); */
    /* let one = Number {
        value: 1,
        odd: true,
    };
    let two = Number {
        value: 2,
        odd: false,
    };
    print_number(one);
    print_number(two);
    let three = Number {
        value: 3,
        odd: true,
    };

    println!(
        "{}|{}|{}|{}",
        three.is_strictly_positive(),
        three.is_strictly_positive_fixed(),
        -three.value,
        if three.is_strictly_positive() {
            "positive"
        } else {
            "negative"
        }
    );
    let four = Number {
        value: 4,
        odd: false,
    };
    print_number_address(&four);
    print_number_address(&four); */

    /* let mut n = Number {
        value: 1,
        odd: true,
    };
    print_number_address(&n);
    invert(&mut n);
    print_number_address(&n); */

    /* let n = Number {
        value: 1,
        odd: true,
    };
    let mut m = n.clone();
    m.value += 100;
    print_number_address(&n);
    print_number_address(&m);
    let mn = m.clone();
    print_number_address(&mn);
    let mn1 = std::clone::Clone::clone(&mn);
    print_number_address(&mn1);
    let p = n;
    let p1 = n;
    print_number(p);
    print_number(p1);
    compare("left", "right"); */
    /* use std::any::type_name;
    println!("{}", type_name::<i32>());
    println!("{}", type_name::<(i64, char, str)>()); */
} */

fn print_number(n: Number) {
    if let Number { value, odd: true } = n {
        println!("{} ", value);
    } else if let Number { value, odd: false } = n {
        println!("{} ", value);
    }
}

fn print_number_address(n: &Number) {
    match n {
        Number { value, odd: true } => println!("{} ", value),
        Number { value, odd: false } => println!("{} ", value),
    }
}

fn invert(n: &mut Number) {
    n.value = -n.value;
}

#[derive(Debug, Clone, Copy)]
struct Number {
    odd: bool,
    value: i32,
}

impl Number {
    fn is_strictly_positive(&self) -> bool {
        self.value > 0
    }
}

impl Signed for Number {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn ne(&self, other: &Self) -> bool {
        self.odd == other.odd && self.value == other.value
    }

    fn is_strictly_positive_fixed(&self) -> bool {
        self.is_strictly_positive()
    }
}

trait Signed {
    fn is_strictly_positive_fixed(&self) -> bool;
    fn eq(&self, other: &Self) -> bool;
    fn ne(&self, other: &Self) -> bool;
}

impl std::ops::Neg for Number {
    type Output = Self;

    fn neg(self) -> Self {
        Number {
            odd: self.odd,
            value: -self.value,
        }
    }
}

/* impl std::clone::Clone for Number {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl std::marker::Copy for Number {} */

fn footbar_one<T: Debug>(arg: T) {
    println!("{:?}", arg);
}

fn footbar_two<L, R>(left: L, right: R)
where
    L: Debug,
    R: Debug,
{
    println!("{:?}", left);
    println!("{:?}", right);
}

fn footbar_two_one<L: Debug, R: Debug>(left: L, right: R) {
    println!("{:?}", left);
    println!("{:?}", right);
}

fn compare<T>(left: T, right: T) -> bool
where
    T: PartialEq + Debug,
{
    println!(
        "{:?} {} {:?}",
        left,
        if left == right { "==" } else { "!=" },
        right
    );
    return left == right;
}

struct Pair<T> {
    a: T,
    b: T,
}

fn print_type_name<T>(_val: &T) {
    println!("{}", std::any::type_name::<T>());
}

/* #[tokio::test]
async fn main_test1() {
    /* let p1 = Pair { a: 1, b: 2 };
    let p2 = Pair { a: true, b: false };
    print_type_name(&p1);
    print_type_name(&p2);
    let mut v1 = Vec::new();
    v1.push(1);
    let mut v2 = Vec::new();
    v2.push(false);
    print_type_name(&v1);
    print_type_name(&v2);
    let v11 = vec![1, 2, 3];
    let v22 = vec![false, true, false];
    print_type_name(&v11);
    print_type_name(&v22); */
    /* use std::io::{self, Read, Write};
    io::stdout().lock().write_all(b"hello world\n").unwrap();
    panic!("test"); */
    /* let o1: Option<i32> = Some(128);
    o1.unwrap();
    let o2: Option<i32> = None;
    o2.unwrap_or(0);
    o2.unwrap(); */

    /* let x_ref = {
        let x = 1;
        &x
    };
    println!("{}", x_ref); */
} */

/* #[tokio::test]
async fn main_test2() {
    let n = Number1 { value: 1 };
    let v = number_value(&n);
    println!("{}", v);
} */

fn print(x: &i32) {
    println!("{}", x);
}
fn print1<'a>(x: &'a i32) {
    println!("{}", x);
}
struct Number1 {
    value: i32,
}

fn number_value<'a>(n: &'a Number1) -> &'a i32 {
    &n.value
}

fn tail(s: &[u8]) -> &[u8] {
    &s[1..]
}

fn tail_a<'a>(s: &'a [u8]) -> &'a [u8] {
    &s[1..]
}

fn file_ext(name: &str) -> Option<&str> {
    name.split(".").last()
}

#[tokio::test]
async fn main_test3() {
    /* let name1 = "Read me. Or don't.txt";
    let ext = {
        let name = String::from("hello.txt");
        file_ext(&name).unwrap()
    };
    println!("{}", ext); */

    /*let name = "Read me. Or don't.txt";
     if let Some(ext) = file_ext(name) {
        println!("file extension:{}", ext);
    } else {
        println!("no extension");
    } */

    /* let v = vec![1, 2, 3, 4, 5];
    let v2 = &v[2..4];
    println!("{:?}", v2);
    let v3 = &v[0..1];
    println!("{:?}", v3);
    println!("{:?}", (0..));
    println!("{:?}", (1000..1500).contains(&1500)); */
    /* let x = &[1, 2, 3, 4, 5];
    let y = tail(x);
    println!("{:?}", y); */
    /* let y = {
        let x = vec![1, 2, 3, 4, 5];
        tail_a(&x)
    };
    println!("{:?}", y); */
}
#[tokio::test]
async fn main_test33() {
    let v = vec![1, 2, 3, 4, 5];
    let v2 = &v[2];
    println!("{:?}", v2);
}
