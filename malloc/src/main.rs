mod heapsize;
mod types;

extern crate graphannis_malloc_size_of as malloc_size_of;
#[macro_use]
extern crate graphannis_malloc_size_of_derive as malloc_size_of_derive;

fn main() {
    types::test_size();
}
