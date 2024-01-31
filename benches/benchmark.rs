use algorithms::search::linear_search;

fn main() {
    divan::main();
}

mod search {
    use algorithms::search::{binary_search, linear_search};

    #[divan::bench]
    fn linear() {
        linear_search(
            divan::black_box(
                include_str!("../assets/search.txt")
                    .split_whitespace()
                    .collect::<Vec<&str>>(),
            ),
            "1000000",
        );
    }

    #[divan::bench]
    fn binary() {
        binary_search(
            divan::black_box(
                include_str!("../assets/search.txt")
                    .split_whitespace()
                    .collect::<Vec<&str>>(),
            ),
            "1000000",
        );
    }
}
