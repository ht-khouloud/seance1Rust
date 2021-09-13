
mod sayhello ;

mod mod_from_main{
 
pub fn say_hello() {
    println!("Hello in main");

} 
}
fn main() {

    println!("------------------------------ Modules ---------------------------------------------------");
    seanceModuleLib::submodule::submodule::extern_sub_module::print_sqoiner(& seanceModuleLib::submodule::submodule::extern_sub_module::Sqoiner {
        number: 10,
    });
    sayhello::sayhelloimpl::say_hello() ;
    println!("------------------------------ Variable ---------------------------------------------------");
    

   
}
