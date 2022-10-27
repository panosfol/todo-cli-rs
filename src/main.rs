mod db;

use db::establish_connection;
fn main() {
    let connection = &mut establish_connection();
    println!("Hello, world!");
}
