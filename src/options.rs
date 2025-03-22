use crate::types::*;

/// Ignores blank lines and removes
/// comments from options file.
pub fn sanitize_options(options: String) -> Vec<String> {
    options
        .lines()
        .map(|op| {
            let end = op.to_string().find("#").unwrap_or(op.len());
            op[0..end].trim().to_string()
        })
        .filter(|op| !op.is_empty())
        .collect()
}

/// Parses options and removes duplicates.
pub fn parse_options(options: Vec<String>) -> (Vec<Analyze>, bool) {
    let mut options = options;

    // Parses flavor and removes that line
    let ob_flavor = parse_flavor(&mut options);

    // Removes duplicates
    let mut seen = std::collections::HashSet::new();
    let options: Vec<Analyze> = options
        .into_iter()
        .filter(|s| seen.insert(s.clone()))
        .filter_map(|s| Analyze::try_from(s.as_str()).ok())
        .collect();

    (options, ob_flavor)
}

/// Check if Obsidian flavored markdown is enabled,
/// which could only be true if "ObsidianFlavor(true)"
/// is found in `options`, and the corresponding line
/// would be removed.
pub fn parse_flavor(options: &mut Vec<String>) -> bool {
    let mut ob_flavor = false;
    let mut index = -1i32;

    for (i, options) in options.iter().enumerate() {
        if let Some(inner) = options.to_lowercase().strip_prefix("obsidianflavor(") {
            if let Some(inner) = inner.strip_suffix(")") {
                ob_flavor = inner == "true";
                index = i as i32;
                break;
            }
        }
    }

    if index >= 0 {
        options.remove(index as usize);
    }

    ob_flavor
}
