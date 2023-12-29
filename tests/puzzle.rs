#[cfg(test)]
mod puzzle_tests {
    use rustdle::puzzle::Puzzle;
    #[test]
    fn grid_stores_letters() {
        let grid = Puzzle::new("ABCDEFGHI").expect("Invalid letter selection");
        assert!(grid.letters() == "ABCDEFGHI");
        // check we don't kill the grid!
        assert!(grid.letters() == "ABCDEFGHI")
    }

    #[test]
    fn accept_square_grids() {
        let good_grid = Puzzle::new("VALIDGRID").expect("Invalid letter selection");

        // TODO test this via the constructor and make square private again
        assert!(good_grid.letters() == "VALIDGRID");
    }

    #[test]
    fn reject_non_square_grid() -> Result<(), String> {
        match Puzzle::new("NOT_SQUARE") {
            None => Ok(()),
            _ => Err(String::from(
                "shouldn't create grid from non-square letter list",
            )),
        }
    }

    #[test]
    fn retrieve_unique_letters_from_grid() {
        let good_grid = Puzzle::new("VALIDGRID").expect("Invalid letter selection");
        assert!(good_grid.unique_letters() == "ADGILRV");
        assert!(good_grid.unique_letters() == "ADGILRV");
    }

    #[test]
    fn only_accept_valid_letters() -> Result<(), String> {
        match Puzzle::new("A_3456789") {
            None => Ok(()),
            _ => Err(String::from("shouldn't create grid with invalid letters")),
        }
    }

    #[test]
    fn reject_too_small_puzzles() -> Result<(), String> {
        // size 1 is Square, but too small.
        match Puzzle::new("A") {
            None => Ok(()),
            _ => Err(String::from(
                "shouldn't create grid with less than four letters",
            )),
        }
    }

    #[test]
    fn random_access() {
        let good_grid = Puzzle::new("VALIDGRID").expect("Invalid letter selection");

        assert_eq!(good_grid.letter_at(1, 1), &'D');
        assert_eq!(good_grid.letter_at(2, 1), &'I');
    }

    #[test]
    fn printing_puzzle_displays_grid_representation() {
        let good_grid = Puzzle::new("VALIDGRID").expect("Invalid letter selection");
        assert_eq!(format!("{}", good_grid), "VAL\nIDG\nRID\n");
    }
}
