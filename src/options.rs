use crate::types::Analyze;

/// Ignores blank lines and removes
/// comments from options file.
fn sanitize_options(options: &str) -> Vec<String> {
    options
        .lines()
        .map(|op| {
            let end = op.to_string().find('#').unwrap_or(op.len());
            op[0..end].trim().to_string()
        })
        .filter(|op| !op.is_empty())
        .collect()
}

/// Parses options and removes duplicates.
pub fn parse_options(options: &str) -> Vec<Analyze> {
    // Sanitizes options
    let options = sanitize_options(options);

    // Removes duplicates
    let mut seen = std::collections::HashSet::new();
    let options: Vec<Analyze> = options
        .into_iter()
        .filter(|s| seen.insert(s.clone()))
        .filter_map(|s| Analyze::try_from(s.as_str()).ok())
        .collect();

    options
}
