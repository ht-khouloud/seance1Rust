
 use crate::mod_from_main;
pub fn say_hello() {
    println!("Hello From extern Mod");
    mod_from_main::say_hello()

}
