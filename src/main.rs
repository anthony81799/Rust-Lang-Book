mod vectors;
mod strings;
mod hash_maps;

fn main() {
    // Collections store multiple values, but are built on the heap
    vectors::run();
    strings::run();
    hash_maps::run();
}
