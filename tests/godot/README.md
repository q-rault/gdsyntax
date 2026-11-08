# Godot's GDScript test scripts

These are the scripts in `modules/gdscript/tests/scripts` of the Godot Engine, copied unchanged from the Godot release pinned in [`godot.toml`](../../godot.toml), one folder per series (`4.7`). The `completion` tests, which test the editor's code completion, are left out. Each `.gd` file has an `.out` file whose first line says what Godot does with it: `GDTEST_OK`, `GDTEST_PARSER_ERROR`, `GDTEST_ANALYZER_ERROR` or `GDTEST_RUNTIME_ERROR`. For a parser error, the second line is Godot's message.

gdsyntax's `tests/godot_suite.rs` checks its tokenizer and parser against them.

They are Godot's work, under the MIT license in [LICENSE.txt](LICENSE.txt).

`just godot-sync check` checks that these files match the pinned release, and `just godot-sync update --tag <tag>` copies another one.
