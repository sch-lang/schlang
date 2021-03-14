enum Arithmetic {
    Addition(i32, i32),
    Subtraction(i32, i32),
    Modulo(i32, i32),
    Division(f32, f32),
    Multiplication(i32, i32),
}

enum Punctuation {
    Comma,
    Semicolon,
    Colon,
    Dot,
    Arrow // ->
}

/*
a == b
a is b

a != b
a is not b

a < b
a is less than b

a > b
a is more than b

a <= b
a is less than or is b

a >= b
a is more than or is b
*/

if (a is more that or is b) {
    print 'Huzzah';
}

enum Comparison {
    Equal, // is
    NotEqual, // is not
    LessThan, // <
    GreaterThan, // >
    LessOrEqual,
}

enum Logical {
    And, // and
    Or, // or
    Not // not
}

enum Declaration {
    Assignment(String, String, Expression)
}

enum Expression {
    Operator,
}

enum Statement {
    Declaration,
}

1 + 1 - 1
Subtract(Add(1, 1), 1)
