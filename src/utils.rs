pub fn without(list: &[String], element: &String) -> Vec<String> {
    list.iter()
        .cloned()
        .filter(|item| item != element)
        .collect()
}

pub fn concat(one: &[String], another: &[String]) -> Vec<String> {
    one.iter().cloned().chain(another.iter().cloned()).collect()
}
