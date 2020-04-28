use crate::paint::{self, Painter};
    let mut source = Source::Unknown;
    for raw_line in lines {
        let line = strip_ansi_codes(&raw_line).to_string();
            source = detect_source(&line);
            if config.commit_style != cli::SectionStyle::Plain {
        } else if (state == State::FileMeta || source == Source::DiffUnified)
            // FIXME: For unified diff input, removal ("-") of a line starting with "--" (e.g. a
            // Haskell or SQL comment) will be confused with the "---" file metadata marker.
            && (line.starts_with("--- ") || line.starts_with("rename from "))
            && config.file_style != cli::SectionStyle::Plain
            && config.file_style != cli::SectionStyle::Plain
        } else if line.starts_with("@@") {
            if config.hunk_style != cli::SectionStyle::Plain {
        } else if source == Source::DiffUnified && line.starts_with("Only in ")
            || line.starts_with("Submodule ")
            || line.starts_with("Binary files ")
        {
            // Additional FileMeta cases:
            //
            // 1. When comparing directories with diff -u, if filenames match between the
            //    directories, the files themselves will be compared. However, if an equivalent
            //    filename is not present, diff outputs a single line (Only in...) starting
            //    indicating that the file is present in only one of the directories.
            //
            // 2. Git diff emits lines describing submodule state such as "Submodule x/y/z contains
            //    untracked content"
            //
            // See https://github.com/dandavison/delta/issues/60#issuecomment-557485242 for a
            // proposal for more robust parsing logic.

            if config.file_style != cli::SectionStyle::Plain {
                handle_generic_file_meta_header_line(&mut painter, &raw_line, config)?;
        if state == State::FileMeta && config.file_style != cli::SectionStyle::Plain {
/// Try to detect what is producing the input for delta.
fn detect_source(line: &str) -> Source {
    if line.starts_with("commit ") || line.starts_with("diff --git ") {
        Source::GitDiff
    } else if line.starts_with("diff -u ")
        || line.starts_with("diff -U")
        || line.starts_with("--- ")
    {
        Source::DiffUnified
    } else {
        Source::Unknown
    }
    let draw_fn = match config.commit_style {
        config.commit_color,
        config.true_color,
/// Construct file change line from minus and plus file and write with FileMeta styling.
    let line = parse::get_file_change_description_from_file_paths(minus_file, plus_file, comparing);
    handle_generic_file_meta_header_line(painter, &line, config)
}

/// Write `line` with FileMeta styling.
fn handle_generic_file_meta_header_line(
    painter: &mut Painter,
    line: &str,
    config: &Config,
) -> std::io::Result<()> {
    let draw_fn = match config.file_style {
        &paint::paint_text_foreground(line, config.file_color, config.true_color),
        config.file_color,
        config.true_color,
    let draw_fn = match config.hunk_style {
    let code_fragment = prepare(raw_code_fragment, false, config);
            &mut painter.output_buffer,
            "",
            config.hunk_color,
            config.true_color,
    writeln!(
        painter.writer,
        "\n{}",
        paint::paint_text_foreground(line_number, config.hunk_color, config.true_color)
    )?;
            painter.minus_lines.push(prepare(&line, true, config));
            painter.plus_lines.push(prepare(&line, true, config));
            // First character at this point is typically a space, but could also be e.g. '\'
            // from '\ No newline at end of file'.
            let prefix = if line.is_empty() { "" } else { &line[..1] };
            let line = prepare(&line, true, config);
                &mut painter.output_buffer,
                prefix,
fn prepare(line: &str, append_newline: bool, config: &Config) -> String {
        // The first column contains a -/+/space character, added by git. We drop it now, so that
        // it is not present during syntax highlighting, and inject a replacement when emitting the
        // line.
        let output_line = if config.tab_width > 0 {
            let tab_replacement = " ".repeat(config.tab_width);
    #[test]
    #[ignore] // #128
    fn test_added_empty_file() {
        let options = get_command_line_options();
        let output = strip_ansi_codes(&run_delta(ADDED_EMPTY_FILE, &options)).to_string();
        assert!(output.contains("\nadded: file\n"));
    }

    #[test]
    fn test_added_file_directory_path_containing_space() {
        let options = get_command_line_options();
        let output = strip_ansi_codes(&run_delta(
            ADDED_FILES_DIRECTORY_PATH_CONTAINING_SPACE,
            &options,
        ))
        .to_string();
        assert!(output.contains("\nadded: with space/file1\n"));
        assert!(output.contains("\nadded: nospace/file2\n"));
    }

        #[derive(PartialEq)]
            let is_true_color = true;
                style::get_minus_color_default(expected_mode == Mode::Light, is_true_color)
                style::get_minus_emph_color_default(expected_mode == Mode::Light, is_true_color)
                style::get_plus_color_default(expected_mode == Mode::Light, is_true_color)
                style::get_plus_emph_color_default(expected_mode == Mode::Light, is_true_color)
        paint::paint_text(&input, style, &mut output, config.true_color);
            color_only: false,
            keep_plus_minus_markers: false,
            commit_color: "Yellow".to_string(),
            file_color: "Blue".to_string(),
            hunk_color: "blue".to_string(),
            true_color: "always".to_string(),
            paging_mode: "auto".to_string(),
            list_theme_names: false,
    #[ignore] // Ideally, delta would make this test pass. See #121.
    #[test]
    fn test_delta_paints_diff_when_there_is_unrecognized_initial_content() {
        for input in vec![
            DIFF_WITH_UNRECOGNIZED_PRECEDING_MATERIAL_1,
            DIFF_WITH_UNRECOGNIZED_PRECEDING_MATERIAL_2,
        ] {
            let mut options = get_command_line_options();
            options.color_only = true;
            let output = run_delta(input, &options);
            assert_eq!(
                strip_ansi_codes(&output).to_string(),
                input.to_owned() + "\n"
            );
            assert_ne!(output, input.to_owned() + "\n");
        }
    }

    #[test]
    fn test_diff_with_merge_conflict_is_not_truncated() {
        let options = get_command_line_options();
        let output = run_delta(DIFF_WITH_MERGE_CONFLICT, &options);
        // TODO: The + in the first column is being removed.
        assert!(strip_ansi_codes(&output).contains("+>>>>>>> Stashed changes"));
        assert_eq!(output.split('\n').count(), 47);
    }

    #[test]
    fn test_diff_with_merge_conflict_is_passed_on_unchanged_under_color_only() {
        let mut options = get_command_line_options();
        options.color_only = true;
        let output = run_delta(DIFF_WITH_MERGE_CONFLICT, &options);
        assert_eq!(
            strip_ansi_codes(&output).to_string(),
            DIFF_WITH_MERGE_CONFLICT.to_owned() + "\n"
        );
    }

    #[test]
    fn test_submodule_contains_untracked_content() {
        let options = get_command_line_options();
        let output = strip_ansi_codes(&run_delta(
            SUBMODULE_CONTAINS_UNTRACKED_CONTENT_INPUT,
            &options,
        ))
        .to_string();
        assert!(output.contains("\nSubmodule x/y/z contains untracked content\n"));
    }

    #[test]
    fn test_triple_dash_at_beginning_of_line_in_code() {
        let options = get_command_line_options();
        let output = strip_ansi_codes(&run_delta(
            TRIPLE_DASH_AT_BEGINNING_OF_LINE_IN_CODE,
            &options,
        ))
        .to_string();
        assert!(
            output.contains(" -- instance (Category p, Category q) => Category (p ∧ q) where\n")
        );
    }

    #[test]
    fn test_binary_files_differ() {
        let options = get_command_line_options();
        let output = strip_ansi_codes(&run_delta(BINARY_FILES_DIFFER, &options)).to_string();
        assert!(output.contains("Binary files /dev/null and b/foo differ\n"));
    }

    #[test]
    fn test_diff_in_diff() {
        let options = get_command_line_options();
        let output = strip_ansi_codes(&run_delta(DIFF_IN_DIFF, &options)).to_string();
        assert!(output.contains("\n ---\n"));
        assert!(output.contains("\n Subject: [PATCH] Init\n"));
    }

    const DIFF_IN_DIFF: &str = "\
diff --git a/0001-Init.patch b/0001-Init.patch
deleted file mode 100644
index 5e35a67..0000000
--- a/0001-Init.patch
+++ /dev/null
@@ -1,22 +0,0 @@
-From d3a8fe3e62be67484729c19e9d8db071f8b1d60c Mon Sep 17 00:00:00 2001
-From: Maximilian Bosch <maximilian@mbosch.me>
-Date: Sat, 28 Dec 2019 15:51:48 +0100
-Subject: [PATCH] Init
-
----
- README.md | 3 +++
- 1 file changed, 3 insertions(+)
- create mode 100644 README.md
-
-diff --git a/README.md b/README.md
-new file mode 100644
-index 0000000..2e6ca05
---- /dev/null
-+++ b/README.md
-@@ -0,0 +1,3 @@
-+# Test
-+
-+abc
---
-2.23.1
-
diff --git a/README.md b/README.md
index 2e6ca05..8ae0569 100644
--- a/README.md
+++ b/README.md
@@ -1,3 +1 @@
 # Test
-
-abc
";
