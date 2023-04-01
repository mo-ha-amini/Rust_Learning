// Scope

// fn main() {
//     let x= 54;
//     {
//         let y = 99;
//         println!("{}, {}", x, y);
//     }
//     // println!("{}, {}", x, y); // Error Scope
// }

//shadowing 

fn main() {
    let x= 54;
    {
        let x = 99;
        println!("{}", x);
    }
    // println!("{}, {}", x, y); // Error Scope
}
