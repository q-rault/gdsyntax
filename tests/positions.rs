//! `SyntaxTree::position` agrees with counting line breaks before the
//! offset, on every character boundary of every script in Godot's tests.

use gdsyntax::Position;

fn by_scanning(source: &str, byte: usize) -> Position {
    let before = &source[..byte];
    let line_start = before.rfind('\n').map_or(0, |i| i + 1);
    Position {
        line: before.matches('\n').count() + 1,
        column: before[line_start..].chars().count() + 1,
    }
}

#[test]
fn positions_match_a_scan_of_the_source() {
    let mut checked = 0;
    let mut stack = vec![std::path::PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/godot"
    ))];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(dir).unwrap().map(Result::unwrap) {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "gd") {
                let Ok(source) = std::fs::read_to_string(&path) else {
                    continue;
                };
                let tree = gdsyntax::parse(source.clone());
                let boundaries = source.char_indices().map(|(byte, _)| byte);
                for byte in boundaries.chain([source.len()]) {
                    assert_eq!(
                        tree.position(byte),
                        by_scanning(&source, byte),
                        "{path:?} at byte {byte}"
                    );
                }
                checked += 1;
            }
        }
    }
    assert!(checked > 500, "only {checked} scripts found");
}
