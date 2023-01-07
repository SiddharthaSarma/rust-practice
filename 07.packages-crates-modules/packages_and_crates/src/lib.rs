/* mod front_of_house {
    mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {

        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
} */

fn check_how_many_users() {}
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            super::super::check_how_many_users();
        }
    }
}


mod customer {

    // use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        // Absolute path
        crates::front_of_house::hosting::add_to_waitlist();

        // Relative path
        front_of_house::hosting::add_to_waitlist();
    }
}

// re-exporting pub use
pub use crate::front_of_house::hosting;

// instead of calling restaurant::front_of_house::hosting::add_to_waitlist
// now we can call restaurant::hosting::add_to_waitlist
