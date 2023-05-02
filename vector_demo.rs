#[derive(Debug)]
struct Test {
    score: i32,
}

impl Test {
    fn new(num: i32) -> Self {
        Self { score: num }
    }
}

fn print_scores(scores: &Vec<Test>) {
    for score in scores {
        println!("{:?}", score);
    }
}

fn main() {
    let scores = vec![
        Test::new(93),
        Test::new(85),
        Test::new(79),
        Test::new(59),
        Test::new(69),
    ];
    print_scores(&scores);
}
