use rustdle::puzzle::Grid;

#[test]
fn grid_stores_letters() {
    let grid = Grid::new("ABCDEFGHI");
    assert!(grid.letters() == "ABCDEFGHI")
}
