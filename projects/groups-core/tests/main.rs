
use group_core::{CycleElement, CycleNotation};

#[test]
fn ready() {
    println!("it works!")
}



#[test]
fn new_cycles() {
    // {{3, 9, 1, 4}, {8}, {6, 0, 17}}
    let cycles = CycleNotation::new(vec![
        CycleElement::from_iter(vec![3, 9, 1, 4]),
        CycleElement::from_iter(vec![8]),
        CycleElement::from_iter(vec![6, 0, 17]),
    ]);

    for (src, to) in &CycleElement::from_iter(vec![3, 9, 1, 4]) {
        println!("{} -> {}", src, to);
    }
    // {{0, 17, 6}, {1, 4, 3, 9}}
    println!("{:?}", cycles);

}

#[test]
fn test_cycles() {
    let mut this = vec![6, 1, 2, 7, 0, 4, 8, 3, 5];
    let last = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    let cycles = CycleNotation::find_permutation(&this, &last).unwrap();
    // {{0, 6, 8, 5, 4}, {3, 7}}
    println!("{:?}", cycles);
    let next = cycles.apply(&mut this);
    println!("{:?}", next);
}


