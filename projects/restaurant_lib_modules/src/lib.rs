mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // relative path
    front_of_house::hosting::add_to_waitlist();
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // use keyword ...bring to the scope
    use crate::front_of_house::hosting;
    hosting::add_to_waitlist();
    //
    use crate::front_of_house as test;
    let st1 = test::St { id: 1, cnt: 0 };
}
