use unicode_box_drawing::Width;

#[derive(Debug)]
pub struct CellConstraints {
    up: Option<Width>,
    down: Option<Width>,
    left: Option<Width>,
    right: Option<Width>,
}
