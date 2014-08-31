use group_core::Cycles;

#[test]
fn ready() {
    println!("it works!")
}



#[test]
fn new_cycles() {
    // {{3, 9, 1, 4}, {8}, {6, 0, 17}}
    let cycles = Cycles::new(vec![
        vec![3, 9, 1, 4],
        vec![8],
        vec![6, 0, 17],
    ]);
    // {{0, 17, 6}, {1, 4, 3, 9}}
    println!("{:?}", cycles);

}

#[test]
fn test_cycles() {
    let mut this = vec![6, 1, 2, 7, 0, 4, 8, 3, 5];
    let last = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let cycles = Cycles::find_permutation(&this, &last).unwrap();
    // {{0, 6, 8, 5, 4}, {3, 7}}
    println!("{:?}", cycles);
    let next = cycles.permute(&mut this);
    println!("{:?}", next);
}


