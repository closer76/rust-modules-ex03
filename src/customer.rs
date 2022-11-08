// 因為 crate root 已經有 hosting 了（見第 30 行），因此可以直接 use
// super (i.e. crate root) 的 hosting
use super::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
