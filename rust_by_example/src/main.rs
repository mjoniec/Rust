fn main() {
    rbe_1_hello_world(); 
    rbe_2_primitives();
    rbe_3_custom_types();
}

mod rbe_1_hello_world;
use rbe_1_hello_world::say_hello;
use rbe_1_hello_world::comment;
use rbe_1_hello_world::formatted_print;
use rbe_1_hello_world::debug;

fn rbe_1_hello_world() {
    say_hello();
    comment();
    formatted_print();
    debug();
}

mod rbe_2_primitives;
use rbe_2_primitives::primitives;
use rbe_2_primitives::literals_and_operators;
use rbe_2_primitives::tuple;
use rbe_2_primitives::arrays_and_slices;

fn rbe_2_primitives(){
    primitives();
    literals_and_operators();
    tuple();
    arrays_and_slices();
}

mod rbe_3_custom_types;
use rbe_3_custom_types::structures;
use rbe_3_custom_types::enums;
//use rbe_3_custom_types::use_test;
use rbe_3_custom_types::c_like_enums;
// use rbe_3_custom_types::linked_list_via_enums;
use rbe_3_custom_types::constants;

fn rbe_3_custom_types(){
    structures();
    enums();
    //use_test();
    c_like_enums();
    constants();
}
