use reordering::{reorder, ReorderIndices};

fn main() {
    let mut seq = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];
    let inds: ReorderIndices = ReorderIndices {
        start: 1,
        end: 4,
        dest: 8,
    };
    println!("Started with {:?}", seq);
    println!(
        "Moving items in range [{}-{}) to position {}",
        inds.start, inds.end, inds.dest
    );
    reorder(&mut seq, inds);
    println!("Ended up with {:?}", seq);
}
