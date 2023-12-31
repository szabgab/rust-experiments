pub fn run() {
    let mut name = "Foo".to_string();
    name.push_str(" and");
    println!("{name}");
    //immutable_take_ownership(name);
    //mutable_take_ownership(name);
    //immutable_borrow(&name);
    mutable_borrow(&mut name);
    //let name = give_ownership();
    println!("after: {name}");
}

// fn give_ownership() -> String {
//     "Foo".to_string()
// }

// fn immutable_take_ownership(name: String) {
//     println!("{name}");
//     //name.push_str(" and bar");
// }

// fn mutable_take_ownership(mut name: String) {
//     name.push_str(" bar");
//     println!("{name}");

// }

// fn immutable_borrow(name: &String) {
//     //name.push_str(" and bar");
//     println!("{name}");
// }

// Cannot borrow variable (for the push_str) that is behind a & - reference
fn mutable_borrow(name: &mut String) {
    name.push_str(" bar");
    println!("{name}");
}


// cases
// we have a variable and we pass it to a function
// 1. we don't want to change the variable in the function
// 2. we want to change it and the change should not impact the variable before the function call
// 3. we want to change it and we want the change to impact the external variable - this generally not a good idea, but what if we have a list of structs and we would like to update them?




// pub fn run() {
//     let fname = "Foo".to_string();
//     let lname = "Bar".to_string();
//     println!("{fname}");
//     println!("{lname}");
//     say_hello(&fname);
//     // println!("This is {name}");
//     let res = combine(&fname, &lname);
//     println!("Combined name {res}");
//     println!("{fname}");
//     println!("{lname}");
// }
// fn say_hello(name: &str) {
//     println!("Hello {name}");
//     println!("Hello {name}");
// }

// fn combine(fname: &str, lname: &str) -> String {
//     //format!("{fname} {lname}")
//     fname.to_string() + " " + lname
// }


// let mut name = "Foo".to_string();
// println!("{name}");
// let lname = "Bar".to_string();
// name.push_str(&lname);
// let other = &name;  // it works here, but we cannot do this before calling push_str!
// println!("{other}");
// println!("{name}");
