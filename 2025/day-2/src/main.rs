use day_2::{RepetitionFinder};

fn main() {
    // Snipped input
    let input = "24-46,124420-259708,584447-720297";
    let repetition_finder = RepetitionFinder::new(input);
    let mut total: u64 = 0;
    let mut advanced_total: u64 = 0;
    for segment in repetition_finder {
        dbg!(&segment);
        total += segment.process_range();
        advanced_total += segment.process_range_advanced();
    }

    dbg!(total);
    dbg!(advanced_total);
}
