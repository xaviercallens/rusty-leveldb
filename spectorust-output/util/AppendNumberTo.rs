/// Appends the decimal representation of `num` to `s`.
pub fn append_number_to(s: &mut String, num: u64) {
    use std::fmt::Write;
    let _ = write!(s, "{num}");
}