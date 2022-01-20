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