mod rbe_1_hello_world;
use rbe_1_hello_world::say_hello;
use rbe_1_hello_world::comment;
use rbe_1_hello_world::formatted_print;
use rbe_1_hello_world::debug;

fn main() {
    rbe_1_hello_world(); 
}

fn rbe_1_hello_world() {
    say_hello();
    comment();
    formatted_print();
    debug();
}