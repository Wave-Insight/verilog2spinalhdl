mod app;
mod example;
mod convert;
use app::App;

fn main() {
    yew::start_app::<App>();
}

//fn main() {
//    println!("{}", convert::convert(&example::example()))
//}

#[test]
fn test() {
    use Werilog::prelude::*;
    //println!("{}", std::mem::size_of::<ModuleDeclaration>());
    println!("{:?}", std::mem::size_of_val(&single_module_parser()));
}
