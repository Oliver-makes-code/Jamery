pub mod block;
pub mod button;
pub mod card;
pub mod file;
pub mod header;
pub mod markdown;
pub mod text;
pub mod time;

fn to_kebab_case(value: &str) -> String {
    let mut result = String::new();

    for (index, ch) in value.chars().enumerate() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_uppercase() && index > 0 {
                result.push('-');
            }

            result.extend(ch.to_lowercase());
        } else if !result.is_empty() && !result.ends_with('-') {
            result.push('-');
        }
    }

    return result.trim_matches('-').to_string();
}
