// move_semantics4.rs
// Refactor this code so that instead of having `vec0` and creating the vector
// in `fn main`, we instead create it within `fn fill_vec` and transfer the
// freshly created vector from fill_vec to its caller.
// Execute `rustlings hint move_semantics4` for hints!

fn main() {
    let vec0 = Vec::new();
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    let mut vec1 = fill_vec(&vec0);
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` no longer take `vec: Vec<i32>` as argument
fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
