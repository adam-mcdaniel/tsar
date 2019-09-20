use tsar::assembler::{lower::Lower, parser::parse};

fn main() {
    println!(
        "{}",
        // "{:#?}",
        match 
        parse(
            r#"

@println("testing");




println("h" + "i" * 5);







println(5 * (5 + 4));


"#
        )
        {Ok(s) => s.lower(),
        Err(e) => e}
    )
}

// fn main() {
//     println!(
//         "{}",
//         (program()).parse(
//             &r#"

// mod core;
// mod core::func;

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// true = 1;
// false = 0;

// fn True(a) => b => a;
// fn False(a) => b => b;
// fn If(c) => a => b => (c(a)(b));

// fn and(a) => b => (a(b)(a));
// fn or(a) => b => (a(a)(b));

// fn inc(a) =>     a + 1;
// fn dec(a) =>     a - 1;
// fn square(n) =>  n * n;

// println(square(10));

// println(
//     If(and(True)(False))
//         (inc)
//         (dec)
//             (6)
// );

// println(((2) * 5) + 6 * (2));

// impl Option {
//     fn some(self, value) {
//         self.value = value;
//         self.is_some = true;
//         self
//     }

//     fn none(self) {
//         self.is_some = false;
//         self
//     }

//     fn and_then(self, function) {
//         self.value = function(self.value);
//         self
//     }

//     fn unwrap(self) => self.value;
// }

// println(
//     Option()
//         .none()
//         .and_then(a => a + 1)
// );

// println(5 is 5);

// "#
//             .trim()
//         ).and_then(|s| Ok(s.lower())).unwrap()
//     );
// }

// // class Value {
// //     fn new(self, value) {
// //         self.value = value;
// //         self
// //     }

// //     fn get_value(self) => self.value;

// //     fn plus_eq(self, value) => self.value += value;
// // }
