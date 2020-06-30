// // // MAKING STRUCT PUBLIC
// // pub mod front_house{
// //     #[derive(Debug)]
// //     pub struct Breakfast{
// //         pub toast:String,
// //         sf:String,
// //     }
// //     impl Breakfast{
// //         pub fn new (toast:String)-> Breakfast{
// //             Breakfast{
// //                 toast,
// //                 sf:String::from("PineApples")
// //             }
// //         }
// //     }
// // }
// // fn eat(){
// //     let mut meal=front_house::Breakfast::new(String::from("Wheat"));
// //     println!("{:?}",meal);
// //     meal.toast=String::from("Barlay");
// //     println!("{:?}",meal);

// // }

// // MAKING ENUMS PUBLIC
// // #![allow(dead_Code)]
// // #![allow(unused_variables)]
// // pub mod front{
// //     #[derive(Debug)]
// //     pub enum Appetite{
// //         Soup,
// //         Salad
// //     }
// // }
// //     fn eat(){
// //         let s1=front::Appetite::Soup;
// //         println!("{:?}",s1);
// //     }

// // Keyword "USE"
// // pub mod front_house{
// //     pub mod hosting{
// //         pub fn add(){

// //         }
// //     }
// // }
// // use crate :: front_house::hosting;
// // pub fn eat(){
// //     hosting::add();
// // }

// use rand::Rng;
// fn main(){
//     let sectet=rand::thread_rng().gen_range(1,101);
//     println!("{}",sectet);
// }