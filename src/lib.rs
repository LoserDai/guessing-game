mod front_of_house {
    pub mod hosting {
        //将add_to_waitlist设置为公有
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    //绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    //相对路径
    front_of_house::hosting::add_to_waitlist();
}