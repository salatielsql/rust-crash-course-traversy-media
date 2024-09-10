// reference pointers - point to a resource in memory

pub fn run() {
    // primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("array 1: {:?} - array 2: {:?}", arr1, arr2);

    // non-primitves
    let vec1 = vec![1, 2, 3];
    // need to point to a reference (&) or else vec1 will no longer hold the values
    let vec2 = &vec1;

    println!("vec 1: {:?} - vec 2: {:?}", vec1, vec2)

}
