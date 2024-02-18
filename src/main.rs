use reordering::{reorder, ReorderIndices};

fn main() {
    let mut seq = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];
    let inds: ReorderIndices = ReorderIndices {
        start: 1,
        end: 4,
        dest: 8,
    };
    reorder(&mut seq, inds);
    println!("{:?}", seq);
}
