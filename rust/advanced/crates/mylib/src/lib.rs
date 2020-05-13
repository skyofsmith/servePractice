//! My Crate
//! 
//! 'my_crate' is a collection of utilites to make performing certain calcalations more convenient
//! 
/// Add two to the number given
///
/// #Example
/// ```
/// let five = 5;
/// 
/// assert_eq!(7, mylib::add_two(5));
/// ```
pub fn add_two(x: i32) -> i32 {
    x + 2
}

/// 给一个数加1
pub fn add_one(x: i32) -> i32 {
    x + 1
}
