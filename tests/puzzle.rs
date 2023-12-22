#[cfg(test)]
mod puzzle_tests {
    use rustdle::puzzle::Grid;
    #[test]
    fn grid_stores_letters() {
        let grid = Grid::new("ABCDEFGHI").expect("Invalid letter selection");
        assert!(grid.letters() == "ABCDEFGHI")
    }

    #[test]
    fn accept_square_grids() {
        let good_grid = Grid::new("VALIDGRID").expect("Invalid letter selection");

        // TODO test this via the constructor and make square private again
        assert!(good_grid.letters() == "VALIDGRID");
    }

    #[test]
    fn reject_non_square_grid() -> Result<(), String> {
        match Grid::new("NOT_SQUARE") {
            None => Ok(()),
            _ => Err(String::from(
                "shouldn't create grid from non-square letter list",
            )),
        }
    }
}
