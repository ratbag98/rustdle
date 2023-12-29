#[cfg(test)]
mod puzzle_tests {
    use rustdle::puzzle::Puzzle;
    #[test]
    fn happy_creation_path() {
        let good_grid = Puzzle::new("VALIDGRID").expect("Invalid letter selection");

        // TODO test this via the constructor and make square private again
        assert!(good_grid.letters() == "VALIDGRID");

        // ownership check whilst I'm a beginner!
        assert!(good_grid.letters() == "VALIDGRID");
    }

    #[test]
    fn reject_non_square_grid() {
        assert!(matches!(Puzzle::new("NOT_SQUARE"), Err(_)));
    }

    #[test]
    fn reject_too_small_puzzles() {
        assert!(matches!(Puzzle::new("A"), Err(_)));
    }

    #[test]
    fn random_access() {
        let good_grid = Puzzle::new("VALIDGRID").expect("Invalid letter selection");

        assert_eq!(good_grid.letter_at(1, 1), &'D');
        assert_eq!(good_grid.letter_at(2, 1), &'I');
    }

    #[test]
    fn neighbours_calculated() {
        let good_grid = Puzzle::new("VALIDGRID").expect("Invalid letter selection");

        assert!(good_grid.neighbours_of(0, 0) == vec![1, 3, 4]);
        assert!(good_grid.neighbours_of(1, 1) == vec![0, 1, 2, 3, 5, 6, 7, 8]);
        assert!(good_grid.neighbours_of(2, 2) == vec![4, 5, 7]);
    }

    #[test]
    fn printing_puzzle_displays_grid_representation() {
        let good_grid = Puzzle::new("VALIDGRID").expect("Invalid letter selection");
        assert_eq!(format!("{}", good_grid), "VAL\nIDG\nRID\n");
    }
}
