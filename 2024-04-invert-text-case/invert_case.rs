/// Convert upper case to lower case, and vice-versa.
fn invert_case(text: &str) -> String {
    let mut opposites = std::collections::HashMap::<char, char>::new();
    for (l, u) in (text.to_lowercase().chars()).zip(text.to_uppercase().chars()) {
        opposites.insert(u, l);
        opposites.insert(l, u);
    }

    text.chars().map(|c| opposites[&c]).collect()
}
