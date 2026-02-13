// mod front_of_house {
//    pub mod hosting {  //added pub to make it private
//       pub  fn add_to_waitlist(){}
//     }
// }


// pb fn eat_at_restaurant(){
//     crate::front_of_house::hosting::add_to_waitlist();

    //relative path


// }

// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order(){
//         cook_order();
//         super::serve_order() //instead of adding pub, just use super to access parents fn

//     }
// }

mod back_of_house(){
    struct Breakfast {
        toast: String,
        seasonal_fruit: String,
    }

    //mod of house >> impl Breakfast >> summer function that resturns the breakfast values
    impl Breakfast {
   pub fn summer(toast: &str) -> Breakfast { //associated functiom with pub to make it public
            Breakfast{
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
            }
           
        }
    }
}


pub fn eat_at_restaurant(){
    let mut meal = back_of_house::Breakfast::summer()
}