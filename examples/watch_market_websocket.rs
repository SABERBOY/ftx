use dotenv::dotenv;
use std::{fmt::Debug, *};

#[tokio::main]
async fn main() {
    dotenv().ok();
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
    use std::any::type_name;
    println!("{}", type_name::<i32>());
    println!("{}", type_name::<(i64, char, str)>());
}

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
