extern crate util;

use util::cli::input::get_i32s;

fn main() {
    let mut sum = 0;

    let multitab_count = get_i32s()[0];
    let mut i = 0;
    while i < multitab_count {
        sum = sum + get_i32s()[0];
        if i < multitab_count - 1 {
            sum = sum - 1;
        }
        i = i + 1;
    }

    println!("{}", sum);
}
