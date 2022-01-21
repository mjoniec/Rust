pub fn primitives(){
    //I dont like the examples from rbe so I ll edit
    
    // Variables can be type annotated.
    let a_float: f64 = 1.1;    // Regular annotation
    let b_float  = 1.1f64; // Suffix  annotation
    let default_float   = 1.1; // default
    
    println!("{a_float} {b_float} {default_float}");

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    println!("{mutable}");
    mutable = 21;
    println!("{mutable}");

    // A type can also be inferred from context 
    let mut inferred_type = 12; // Type i64 is inferred from another line, that occurs later
    println!("{inferred_type}");
    inferred_type = 4294967296i64;
    println!("{inferred_type}");
  //inferred_type = true; // Error! The type of a variable can't be inferred to this type, after using previous types.
    let inferred_type = true; // Variables can be overwritten with shadowing (with changing type).
    println!("{inferred_type}");
}

pub fn literals_and_operators() {
  // Integer addition
  println!("1 + 2 = {}", 1u32 + 2);//why 11i32 - suffix annotation after 1

  // Integer subtraction
  println!("1 - 2 = {}", 1i32 - 2);
  // TODO ^ Try changing `1i32` to `1u32` to see why the type is important
  //guess answer - counter overflow to 4 bilion
  //println!("1 - 2 = {}", 1u32 - 2);
  //after I put the line I even got overflow error in vs code - wont even let to compile

  // Short-circuiting boolean logic
  println!("true AND false is {}", true && false);
  println!("true OR false is {}", true || false);
  println!("NOT true is {}", !true);

  // Bitwise operations
  println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
  println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
  println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
  println!("1 << 5 is {}", 1u32 << 5);
  println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

  // Use underscores to improve readability!
  println!("One million is written as {}", 1_000_000u32);
}

// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
  // `let` can be used to bind the members of a tuple to variables
  let (integer, boolean) = pair;

  (boolean, integer)
}

// The following struct is for the activity.
#[derive(Debug)]
pub struct Matrix(f32, f32, f32, f32);

//2.2 activity
use std::fmt;
impl fmt::Display for Matrix {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
  }
}

pub fn tuple(){
  // A tuple with a bunch of different types
  let long_tuple = (1u8, 2u16, 3u32, 4u64,
    -1i8, -2i16, -3i32, -4i64,
    0.1f32, 0.2f64,
    'a', true);

  // Values can be extracted from the tuple using tuple indexing
  println!("long tuple first value: {}", long_tuple.0);
  println!("long tuple second value: {}", long_tuple.1);

  // Tuples can be tuple members
  let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

  // Tuples are printable
  println!("tuple of tuples: {:?}", tuple_of_tuples);

  // But long Tuples cannot be printed
   //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
  // println!("too long tuple: {:?}", too_long_tuple);
  // TODO ^ Uncomment the above 2 lines to see the compiler error

  let pair = (1, true);
  println!("pair is {:?}", pair);

  println!("the reversed pair is {:?}", reverse(pair));

  // To create one element tuples, the comma is required to tell them apart
  // from a literal surrounded by parentheses
  println!("one element tuple: {:?}", (5u32,));
  println!("just an integer: {:?}", (5u32));

  //tuples can be destructured to create bindings
  let tuple = (1, "hello", 4.5, true);

  let (a, b, c, d) = tuple;
  println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

  let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
  println!("{:?}", matrix);

  
  println!("activities");
  println!("{matrix}");
  println!("Transpose:\n{}", transpose(matrix));
}

pub fn transpose(matrix: Matrix) -> Matrix{
  let m = Matrix (matrix.0, matrix.2, matrix.1, matrix.3);
  m
}