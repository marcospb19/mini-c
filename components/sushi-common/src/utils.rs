pub fn display_comma_separate_list(list: &[String]) -> String {
    assert!(!list.is_empty());

    list.iter().map(String::as_str).intersperse(", ").collect()
}
