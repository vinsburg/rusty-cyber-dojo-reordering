use reordering::reorder;

fn main() {
    let mut vec = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];
    reorder(&mut vec, 1, 4, 8);
    println!("{:?}", vec);
}
