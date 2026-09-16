pub mod game_hud;
pub mod laboratory;
pub mod main_menu;
pub mod options;
pub mod side_panel;
pub mod theme;

/// Uses the notation adopted in the course handout: `+` denotes union.
pub fn display_regex(expression: &str) -> String {
    expression.replace('|', "+")
}

#[cfg(test)]
mod tests {
    use super::display_regex;

    #[test]
    fn displays_union_with_a_plus_sign() {
        assert_eq!(display_regex("(1|0|2)*a"), "(1+0+2)*a");
    }
}
