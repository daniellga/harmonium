// Formatting environment variables
// * `H_FMT_MAX_COLS` -> maximum number of columns shown when formatting HMatrix.
// * `H_FMT_MAX_ROWS` -> maximum number of rows shown when formatting HMatrix.
// * `H_FMT_MAX_LEN` -> maximum length shown when formatting HArray.
// * `H_FMT_TABLE_FORMATTING` -> define styling of tables using any of the following options (default = UTF8_FULL_CONDENSED):
//
//                                    ASCII_FULL
//                                    ASCII_FULL_CONDENSED
//                                    ASCII_NO_BORDERS
//                                    ASCII_BORDERS_ONLY
//                                    ASCII_BORDERS_ONLY_CONDENSED
//                                    ASCII_HORIZONTAL_ONLY
//                                    ASCII_MARKDOWN
//                                    UTF8_FULL
//                                    UTF8_FULL_CONDENSED
//                                    UTF8_NO_BORDERS
//                                    UTF8_BORDERS_ONLY
//                                    UTF8_HORIZONTAL_ONLY
//                                    NOTHING
//
//                                    These options are defined by comfy-table which provides examples for each at:
//                                    https://github.com/Nukesor/comfy-table/blob/main/src/style/presets.rs
pub(crate) const FMT_MAX_COLS: &str = "H_FMT_MAX_COLS";
pub(crate) const FMT_MAX_ROWS: &str = "H_FMT_MAX_ROWS";
pub(crate) const FMT_MAX_LEN: &str = "H_FMT_MAX_LEN";
pub(crate) const FMT_TABLE_FORMATTING: &str = "H_FMT_TABLE_FORMATTING";
