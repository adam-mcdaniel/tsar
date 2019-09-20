use super::lower::Lower;


#[derive(Debug, Clone, PartialEq)]
pub struct ScopedName(pub Vec<String>);

impl Lower for ScopedName {
    fn lower(&self) -> String {
        let ScopedName(names) = self;
        let mut result = names[0].to_string();
        for name in &names[1..] {
            result += &format!("[\"{}\"]", name);
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Module(pub ScopedName);

impl Lower for Module {
    fn lower(&self) -> String {
        let Module(name) = self;
        name.lower() + " = dict()"
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub module: ScopedName,
    pub imports: Vec<String>,
}

impl Lower for Import {
    fn lower(&self) -> String {
        let Import { module, imports } = self;
        let mut result = "".to_string();
        for item in imports {
            result += &format!(
                "{item} = {module}[\"{item}\"]\n",
                module = module.lower(),
                item = item
            );
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier(pub String);

impl Lower for Identifier {
    fn lower(&self) -> String {
        let Identifier(name) = self;
        name.to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DotName(pub String);

impl Lower for DotName {
    fn lower(&self) -> String {
        let DotName(name) = self;
        format!(".{}", name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexedName(pub String);

impl Lower for IndexedName {
    fn lower(&self) -> String {
        let IndexedName(name) = self;
        format!("[{}]", name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call(pub Vec<Value>);

impl Lower for Call {
    fn lower(&self) -> String {
        let Call(values) = self;
        format!(
            "({})",
            values
                .iter()
                .map(|v| v.lower())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Number(pub String);

impl Lower for Number {
    fn lower(&self) -> String {
        let Number(n) = self;
        format!("{}", n)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct List(pub Vec<Value>);

impl Lower for List {
    fn lower(&self) -> String {
        String::from("list()")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function(pub Vec<Identifier>, pub Vec<Expr>);

impl Lower for Function {
    fn lower(&self) -> String {
        let Function(args, exprs) = self;
        format!(
            "fn({args}) {{ {body} }}",
            args = args
                .iter()
                .map(|v| v.lower())
                .collect::<Vec<String>>()
                .join(","),
            body = exprs
                .iter()
                .map(|v| v.lower())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDef(pub FirstValue, pub Function);

impl Lower for FunctionDef {
    fn lower(&self) -> String {
        let FunctionDef(name, func) = self;
        let Function(args, exprs) = func;
        format!(
            "fn {}({args}) {{ {body} }}",
            name.lower(),
            args = args
                .iter()
                .map(|v| v.lower())
                .collect::<Vec<String>>()
                .join(","),
            body = exprs
                .iter()
                .map(|v| v.lower())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDef(pub FirstValue, pub Vec<FunctionDef>);

impl Lower for ClassDef {
    fn lower(&self) -> String {
        let ClassDef(name, methods) = self;

        format!(
            "class {} {{ {} }}",
            name.lower(),
            methods
                .iter()
                .map(|v| v.lower())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    List(List),
    Number(Number),
    Function(Function),
    ForeignFunction(Identifier),
}

impl Lower for Literal {
    fn lower(&self) -> String {
        match self {
            Self::String(s) => format!("\"{}\"", s),
            Self::List(l) => l.lower(),
            Self::Number(n) => n.lower(),
            Self::Function(f) => f.lower(),
            Self::ForeignFunction(f) => format!("@{}", f.lower()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FirstValue {
    Group(Box<Value>),
    ScopedName(ScopedName),
    Identifier(Identifier),
    Literal(Literal),
    Math(Math),
}

impl Lower for FirstValue {
    fn lower(&self) -> String {
        format!(
            "{}",
            match self {
                Self::Group(g) => format!("({})", (*g).clone().lower()),
                Self::ScopedName(s) => format!("{}", s.lower()),
                Self::Identifier(i) => format!("{}", i.lower()),
                Self::Literal(l) => format!("{}", l.lower()),
                Self::Math(m) => format!("{}", m.lower()),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Math {
    Add(Box<Value>, Box<Value>),
    Subtract(Box<Value>, Box<Value>),
    Multiply(Box<Value>, Box<Value>),
    Divide(Box<Value>, Box<Value>),
    Remainder(Box<Value>, Box<Value>),
    Is(Box<Value>, Box<Value>),
    Isnt(Box<Value>, Box<Value>),
}

impl Lower for Math {
    fn lower(&self) -> String {
        format!(
            "{}",
            match self {
                Self::Add(a, b) => format!("add({}, {})", a.lower(), b.lower()),
                Self::Subtract(a, b) => format!("sub({}, {})", a.lower(), b.lower()),
                Self::Multiply(a, b) => format!("mul({}, {})", a.lower(), b.lower()),
                Self::Divide(a, b) => format!("div({}, {})", a.lower(), b.lower()),
                Self::Remainder(a, b) => format!("rem({}, {})", a.lower(), b.lower()),
                Self::Is(a, b) => format!("eq({}, {})", a.lower(), b.lower()),
                Self::Isnt(a, b) => format!("not(eq({}, {}))", a.lower(), b.lower()),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SecondValue {
    Call(Call),
    DotName(DotName),
    IndexedName(IndexedName),
}

impl Lower for SecondValue {
    fn lower(&self) -> String {
        format!(
            "{}",
            match self {
                Self::Call(c) => c.lower(),
                Self::DotName(d) => d.lower(),
                Self::IndexedName(i) => i.lower(),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value(pub FirstValue, pub Vec<SecondValue>);

impl Lower for Value {
    fn lower(&self) -> String {
        let Value(head, tail) = self;
        let mut result = format!("{}", head.lower());
        // for val in tail {
        //     result = format!("({}){}", result, val.lower());
        // }
        if tail.len() > 0 {
            result = format!("({})", result);
            result += &tail[0].lower();
            for (i, value) in tail[1..].iter().enumerate() {
                match (&tail[i], value) {
                    (SecondValue::DotName(_), SecondValue::Call(_)) => {
                        result = format!("{}{}", result, value.lower())
                    }
                    (SecondValue::IndexedName(_), SecondValue::Call(_)) => {
                        result = format!("{}{}", result, value.lower())
                    }
                    (SecondValue::DotName(_), SecondValue::DotName(_)) => {
                        result = format!("{}{}", result, value.lower())
                    }
                    (SecondValue::IndexedName(_), SecondValue::IndexedName(_)) => {
                        result = format!("{}{}", result, value.lower())
                    }
                    _ => result = format!("({}){}", result, value.lower()),
                }
            }
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment(pub Value, pub Value);

impl Lower for Assignment {
    fn lower(&self) -> String {
        let Assignment(left, right) = self;
        format!(
            "{} = {}",
            left.lower().replace("(", "").replace(")", ""),
            right.lower()
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Value(Value),
    ClassDef(ClassDef),
    Assignment(Assignment),
    FunctionDef(FunctionDef),
}

impl Lower for Expr {
    fn lower(&self) -> String {
        match self {
            Self::Value(v) => v.lower() + ";",
            Self::ClassDef(c) => c.lower(),
            Self::Assignment(a) => a.lower() + ";",
            Self::FunctionDef(f) => f.lower(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program(pub Vec<Import>, pub Vec<Module>, pub Vec<Expr>);

impl Lower for Program {
    fn lower(&self) -> String {
        let Program(imports, modules, body) = self;
        format!(
            "{}\n\n\n{}\n\n\n{}",
            imports
                .iter()
                .map(|v| v.lower())
                .collect::<Vec<String>>()
                .join("\n"),
            modules
                .iter()
                .map(|v| v.lower())
                .collect::<Vec<String>>()
                .join("\n"),
            body.iter()
                .map(|v| v.lower())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
