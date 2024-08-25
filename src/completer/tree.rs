use indoc::indoc;

pub const INCOMPLETE_TREE_PARTS: &str = indoc! {"
    The parts of a tree are:
    - Trunk
    - "};
pub const COMPLETE_TREE_PARTS: &str = indoc! {"Branches
    - Leaves
    - Roots"
};

pub const INCOMPLETE_TREE_CHARACTERISTICS: &str = indoc! {"
    Adjectives describing an average tree are:
    - Tall
    - Green
    - "};
pub const COMPLETE_TREE_CHARACTERISTICS: &str = indoc! {"Leafy
    - Sturdy
    - Broad"
};

pub fn parts() -> (&'static str, &'static str) {
    (INCOMPLETE_TREE_PARTS, COMPLETE_TREE_PARTS)
}

pub fn characteristics() -> (&'static str, &'static str) {
    (
        INCOMPLETE_TREE_CHARACTERISTICS,
        COMPLETE_TREE_CHARACTERISTICS,
    )
}

