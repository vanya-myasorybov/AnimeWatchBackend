use std::sync::LazyLock;

use regex::Regex;

pub static USERNAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z]+$").expect("Invalid regex pattern"));
