// // import rand::Rng;


// // let secret_number = rand::thread_rng().gen_range(1..=100);
// // mod front_of_house {
// //    pub mod hosting {  //added pub to make it private
// //       pub  fn add_to_waitlist(){}
// //     }
// // }


// // pb fn eat_at_restaurant(){
// //     crate::front_of_house::hosting::add_to_waitlist();

//     //relative path


// // }

// // fn serve_order() {}

// // mod back_of_house {
// //     fn fix_incorrect_order(){
// //         cook_order();
// //         super::serve_order() //instead of adding pub, just use super to access parents fn

// //     }
// // }

// mod back_of_house(){
//   pub struct Breakfast {
//       pub  toast: String,
//         seasonal_fruit: String,
//     }

//     //mod of house >> impl Breakfast >> summer function that resturns the breakfast values
//     impl Breakfast {
//    pub fn summer(toast: &str) -> Breakfast { //associated functiom with pub to make it public
//             Breakfast{
//             toast: String::from(toast),
//             seasonal_fruit: String::from("peaches"),
//             }
           
//         }
//     }

//     //use of enums when marked as public, every of its variance becomes public

// pub enum Appertizer {
//     soup,
//     salad.
// }

// //The Use keyword
// pub mod hosting {
//     pub fn add_to_waitlist(){};
// }
// }


// pub fn eat_at_restaurant(){
//     let mut meal = back_of_house::Breakfast::summer('Rye');

//     meal.toast = String::from("Wheat");
// }




// pub fn eat_at_restaurant(){
//     let order1 = Back_of_house::Appertizer::soup;
//     let order2 = Back_of_house::Appertizer::salad;
// }


// pub fn eat_at_restaurant(){
//     //X don't do this
//     front_of_house::hosting::add_to_waitlist();
//      front_of_house::hosting::add_to_waitlist();
//       front_of_house::hosting::add_to_waitlist();
// }
// //rather use the use keyword to bring the hosting module into scope
// //add pub keyword to make it public and access to other modules
//  pub use crate::front_of_house::hosting; //for absolute paths 
// use self::front_of_house::hosting; //for relative paths 
// //Use keyword allows us to bring a path into scope and we add the pub keyword to make it public so that other modules can access it without having to specify the full path every time
// //so it becomes 
// pub fn eat_at_restaurant(){
//     //X don't do this
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }