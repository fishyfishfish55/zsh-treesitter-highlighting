use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

fn main() {
    let mut highlighter = Highlighter::new();

    let highlight_names = [
        "attribute",
        "constant",
        "function.builtin",
        "function",
        "keyword",
        "operator",
        "property",
        "punctuation",
        "punctuation.bracket",
        "punctuation.delimiter",
        "string",
        "string.special",
        "tag",
        "type",
        "type.builtin",
        "variable",
        "variable.builtin",
        "variable.parameter",
    ];

    let source = b"export TEST=\"hello$(date)\"";

    let bash_language = tree_sitter_bash::language();

    let mut bash_config = HighlightConfiguration::new(
        bash_language,
        "bash",
        tree_sitter_bash::HIGHLIGHT_QUERY,
        "",
        "",
    )
    .unwrap();

    bash_config.configure(&highlight_names);

    let highlights = highlighter
        .highlight(&bash_config, source, None, |_| None)
        .unwrap();

    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source { start, end } => {
                eprintln!("source: {}-{}", start, end);
            }
            HighlightEvent::HighlightStart(s) => {
                eprintln!("highlight style started: {:?}", s);
            }
            HighlightEvent::HighlightEnd => {
                eprintln!("highlight style ended");
            }
        }
    }
}
