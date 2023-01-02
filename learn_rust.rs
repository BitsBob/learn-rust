// * This code views best on VS Code with the 'Better Comments' extension.

/* 
!! This code dose NOT compile properly,
!! and is not made to. It is made as almost
!! a reference table
*/

// * PART ONE: OPERATORS
/*
Operators in Rust are fairy typical, with a few
exeptions. I will not discuss referencing (&) and
dereferencing (*) operators here, however the
dereferencing operator is also the multiplication
operator so will be mentioned in a different
context.
*/ 

// * Arithmetic operators
// These are fairly self explanitory, so I will 
// not write much about them.

// Multiplication
10 * 15

// Addition
10 + 5

// Subtraction
5 - 2

// Division
10 / 5

// Division remainder (Modulous)
10 % 5

// * Assingment operators
// Asingment operator
// We use an assignment operator to assign a 
// value to a variable. For example,
let x = 5;
/*
Here, = is an assignment operator that assigns
the value on the right 5 to the variable x on 
the left. The assignment operator is the most 
common operator in Rust.
*/

// Compound assignment operators
/*
We can also use an assignment operator and an
arithmetic operator, known as a compound assignment
operator. For example,
*/
let mut x = 1; // X is one

x += 10; // X is now eleven

/*
You can also use a compound assignment operator with
any of the aforementioned operators, to return a similar
result. For example, you could use a compound assignment
operatior like -= or *=.
*/
// * PART TWO: VARIABLES AND MUTALITY
/*
In the first statement, you'll notice that x is
assigned with 'let mut x = 1' not 'let x = 1'. This
is because variables are immutable in Rust by deafult.
This means that they cannot be changed, so when we 
need a variable that can change, we use the 'mut'
keyword.
You receive the error message 'cannot assign twice 
to immutable variable `x`' if you try to change an
immutable variable.
It’s important that we get compile-time errors when 
we attempt to change a value that’s designated as 
immutable because this very situation can lead to bugs. 
If one part of our code operates on the assumption 
that a value will never change and another part of 
our code changes that value, it’s possible that the 
first part of the code won’t do what it was designed 
to do. The cause of this kind of bug can be difficult 
to track down after the fact, especially when the 
second piece of code changes the value only sometimes. 
The Rust compiler guarantees that when you state that a 
value won’t change, it really won’t change, so you 
don’t have to keep track of it yourself. Your code 
is thus easier to reason through.
*/

// Constants
/*
Constants are variables that cannot change, not just by
deafult, but can never change, ever. Here is an example.
Don't worry about the ':u32' bit, that's a type annotation,
which we'll cover later.
*/

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

/*
Constants are valid for the entire time a program runs, 
within the scope in which they were declared. This property 
makes constants useful for values in your application 
domain that multiple parts of the program might need to 
know about, such as the maximum number of points any 
player of a game is allowed to earn, or the speed of light.
*/

// * A quick note on naming conventions
/*
When declaring a constant, we use all capitals, like above.
A function or a program is named with snake case, such as
'int_to_double'.
*/ 

// * PART THREE: SHADOWING
/*
In Rust, we can 'shadow' variables, which means that variables 
can have the same name, and the value will just be replaced.
This is best explained with an example.
*/

let x = 5;

let x = x + 1;

{ // Here, we are creating a new scope. A scope is just a section of code, like a function.
    let x = x * 2;
    println!("The value of x in the inner scope is: {}", x);
} // Here the scope ends

println!("The value of x is: {}", x);

/*
This program first binds x to a value of 5.
Then it creates a new variable x by repeating
let x =, taking the original value and adding
1 so the value of x is then 6. Then, within
an inner scope created with the curly brackets,
the third let statement also shadows x and
creates a new variable, multiplying the previous
value by 2 to give x a value of 12. When that
scope is over, the inner shadowing ends and x 
returns to being 6. When we run this program, 
it will output the following:
*/

/*
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/variables`
The value of x in the inner scope is: 12
The value of x is: 6
*/

/*
Here, we see that the value of x changes in the diferent
scopes and we are still using the same variale name, x.
This removes the need for creating exponentially more 
differently named variables, such as num_1, then num_2, 
then num_3 and on and on.
*/

// * PART 4: INSTALLING AND USING THE RUST COMPILER

// * PART 5: DATA TYPES
/*
Every value in Rust has a type, called a data type, which
which tells Rust exactly how to work with this data. If
you have previously programmed in a language suuch as 
Python, you will be compleatly new to this concept, as
Python iinfers data types for you. Rust can also infer 
data types, however in cartian situations it is necessary
to annotate the data types by yourself, so it is important
to learn this. 
*/

// Scalar types
/*
A scalar type represents a single value. Rust has four
primary scalar types: integers, floating-point numbers, 
Booleans, and characters. You may recognize these from 
other programming languages.
Keep in mind that Rust is a statically typed language, 
which means that it must know the types of all variables 
at compile time. The compiler can usually infer what type 
we want to use based on the value and how we use it. 
In cases when many types are possible, such as when we 
converte a String to a numeric type using parse, we must
add a type annotation as so:
*/

let guess: u32 = "42".parse().expect("Not a number!");

/*
If we don’t add the : u32 type annotation shown in the 
preceding code, Rust will display an error message that indicates
that we should add a type annotation. Here is a list of
the integer data types that are available to us by deafult
in Rust:

Length     Signed       Unsigned     
    8          i8           u8        
    16         i16          u16        
    32         i32          u32        
    64         i64          u64       
    128        i128         u128    
    arch       isize        usize  

Each variant can be either signed or unsigned and has an
explicit size. Signed and unsigned refer to whether it’s 
possible for the number to be negative—in other words, 
whether the number needs to have a sign with it (signed) 
or whether it will only ever be positive and can therefore 
be represented without a sign (unsigned). It’s like writing 
numbers on paper: when the sign matters, a number is shown 
with a plus sign or a minus sign; however, when it’s safe 
to assume the number is positive, it’s shown with no sign.

Additionally, the isize and usize types depend on the architecture
of the computer your program is running on, which is 
denoted in the table as “arch”: 64 bits if you’re on a 
64-bit architecture and 32 bits if you’re on a 32-bit architecture.

So how do you know what type of number to use? Well generally,
if you don't know what to put, then you should probably put u32,
as it is the most common type.
*/

// Floating point types
/*
Rust has two floating point types, f32 and f64, which are 
32 bits and 64 bits in size, respectively. The default 
type is f64 because on modern CPUs, it’s roughly the same 
speed as f32 but is capable of more precision. All floating-point 
types are signed. f32 are single precision and f64 are double 
precision. Here’s an example that shows floating-point 
numbers in action:
*/

let x = 2.0; // f64

let y: f32 = 3.0; // f32

// Boolean values
/*
As in most other languages, Rust supports boolean values. The 
only values that a boolean type can take is true or false. The
main use for a boolean type is with control flow, which we will
look at in the next section. An example of a boolean type in action:
*/

let t = true;

let f: bool = false; // with explicit type annotation

// The char type
/*
Rust’s char type is the language’s most primitive alphabetic type.
Here are some examples of declaring char values:
*/

let z: char = 'z';

let natural_symbol: char = 'ℤ';

let heart_eyed_cat: char = '😻';

/*
!! Note that we specify char literals with single quotes, as opposed 
!! to string literals, which use double quotes. Rust’s char type is 
!! four bytes in size and represents a Unicode Scalar Value, which 
!! means it can represent a lot more than just ASCII. Accented letters; 
!! Chinese, Japanese, and Korean characters; emoji; and zero-width 
!! spaces are all valid char values in Rust. Unicode Scalar Values 
!! range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. 
!! However, a “character” isn’t really a concept in Unicode, so your 
!! human intuition for what a “character” is may not match up with what 
!! a char is in Rust.
*/

// Compound types

/*
Compound types can group multiple values into one type. Rust has 
two primitive compound types: tuples and arrays.
*/

// Tuple type
/*
A tuple is a way of grouping together multiple values with different
data types. They have a fixed length, which cannot be added to or 
removed from. We create a tuple by writing a comma-separated list 
of values inside parentheses. This example has type annotations.
*/

let my_tup: (u32, f32, bool) = (27, 2.7, true);

/*
The value my_tup is now bound to all three values, and is considered a
single compound element. 
We can acces the items in a tuple in many ways, but one way is using the .
operator, as follows:
*/

let x: (i32, f64, u8) = (500, 6.4, 1);

let five_hundred = x.0;

let six_point_four = x.1;

let one = x.2;

/*
This program creates the tuple x and then takes the index of each element
and assigns it to another varaible. As with most languages, indexign 
starts with 0. The tuple without any values has a special name, unit.
This value and its corresponding type are both written () and represent 
an empty value or an empty return type. Expressions implicitly return the 
unit value if they don’t return any other value. This may be a bit confusing
to you at the moment, but is unimportant until later.
*/

// The array type

// * PART 6: CONTROLL FLOW AND THE MATCH STATEMENT

// * PART 7: FUNCTIONS

// * PART 8: UNDERSTANDING OWNERSHIP
/*
Ownership is Rust’s most unique feature and has deep 
implications for the rest of the language. It enables 
Rust to make memory safety guarantees without needing a 
garbage collector, so it’s important to understand 
how ownership works.
*/