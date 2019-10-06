pub mod cmd;
pub mod assembler;
pub mod builder;


pub const STD_LIBRARY: &'static str = r#"
std = dict();
std["io"] = dict();
std["io"]["print"] = print;
std["io"]["println"] = println;
std["io"]["format"] = format;

std["object"] = dict();
std["object"]["new"] = new;

std["math"] = dict();
std["math"]["add"] = add;
std["math"]["sub"] = sub;
std["math"]["div"] = div;
std["math"]["mul"] = mul;
std["math"]["rem"] = rem;
std["math"]["not"] = not;
std["math"]["eq"] = eq;
std["math"]["neq"] = neq;

std["list"] = dict();
std["list"]["len"] = len;
std["list"]["push"] = push;
std["list"]["pop"] = pop;
std["list"]["range"] = range;
std["list"]["reverse"] = reverse;
std["list"]["map"] = map;
std["list"]["filter"] = filter;
std["list"]["reduce"] = reduce;

"#;