pub fn run() {
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("arr1 {:?} arr2 {:?}", arr1, arr2);

    let vec1: Vec<i32> = vec![4, 5, 6];
    let vec2 = &vec1;

    println!("vec1 {:?} vec2 {:?}", &vec1, vec2);
}
