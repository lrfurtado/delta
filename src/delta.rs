use crate::paint::{self, Painter};
        } else if (state == State::FileMeta || source == Source::DiffUnified)
            // FIXME: For unified diff input, removal ("-") of a line starting with "--" (e.g. a
            // Haskell or SQL comment) will be confused with the "---" file metadata marker.
            && (line.starts_with("--- ") || line.starts_with("rename from "))
            || line.starts_with("Binary files ")
        config.commit_color,
        config.true_color,
        &paint::paint_text_foreground(line, config.file_color, config.true_color),
        config.file_color,
        config.true_color,
            config.hunk_color,
            config.true_color,
    writeln!(
        painter.writer,
        "\n{}",
        paint::paint_text_foreground(line_number, config.hunk_color, config.true_color)
    )?;
        #[derive(PartialEq)]
            let is_true_color = true;
                style::get_minus_color_default(expected_mode == Mode::Light, is_true_color)
                style::get_minus_emph_color_default(expected_mode == Mode::Light, is_true_color)
                style::get_plus_color_default(expected_mode == Mode::Light, is_true_color)
                style::get_plus_emph_color_default(expected_mode == Mode::Light, is_true_color)
        paint::paint_text(&input, style, &mut output, config.true_color);
            commit_color: "Yellow".to_string(),
            file_color: "Blue".to_string(),
            hunk_color: "blue".to_string(),
            true_color: "always".to_string(),
            paging_mode: "auto".to_string(),
            list_theme_names: false,
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
