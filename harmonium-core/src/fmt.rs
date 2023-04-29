use crate::{
    configs::{FMT_MAX_COLS, FMT_MAX_LEN, FMT_MAX_ROWS, FMT_TABLE_FORMATTING},
    structs::{HComplexArray, HComplexMatrix, HFloatArray, HFloatAudio, HFloatMatrix},
};
use arrow2::{array::PrimitiveArray, types::NativeType};
use comfy_table::{
    presets::{
        ASCII_BORDERS_ONLY, ASCII_BORDERS_ONLY_CONDENSED, ASCII_FULL, ASCII_FULL_CONDENSED,
        ASCII_HORIZONTAL_ONLY, ASCII_MARKDOWN, ASCII_NO_BORDERS, NOTHING, UTF8_BORDERS_ONLY,
        UTF8_FULL, UTF8_FULL_CONDENSED, UTF8_HORIZONTAL_ONLY, UTF8_NO_BORDERS,
    },
    ContentArrangement, Table,
};
use num_traits::Float;
use std::fmt;

const DEFAULT_FMT_MAX_LEN: usize = 10;
const DEFAULT_FMT_MAX_COLS: usize = 10;
const DEFAULT_FMT_MAX_ROWS: usize = 10;
const DEFAULT_FMT_TABLE_FORMATTING: &str = "DEFAULT";

impl<T> fmt::Display for HFloatArray<T>
where
    T: NativeType + Float,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_len = std::env::var(FMT_MAX_LEN)
            .as_deref()
            .unwrap_or("")
            .parse::<usize>()
            .map(|x| std::cmp::max(x, 5))
            .unwrap_or(DEFAULT_FMT_MAX_LEN);
        let preset = match std::env::var(FMT_TABLE_FORMATTING)
            .as_deref()
            .unwrap_or(DEFAULT_FMT_TABLE_FORMATTING)
        {
            "ASCII_FULL" => ASCII_FULL,
            "ASCII_FULL_CONDENSED" => ASCII_FULL_CONDENSED,
            "ASCII_NO_BORDERS" => ASCII_NO_BORDERS,
            "ASCII_BORDERS_ONLY" => ASCII_BORDERS_ONLY,
            "ASCII_BORDERS_ONLY_CONDENSED" => ASCII_BORDERS_ONLY_CONDENSED,
            "ASCII_HORIZONTAL_ONLY" => ASCII_HORIZONTAL_ONLY,
            "ASCII_MARKDOWN" => ASCII_MARKDOWN,
            "UTF8_FULL" => UTF8_FULL,
            "UTF8_FULL_CONDENSED" => UTF8_FULL_CONDENSED,
            "UTF8_NO_BORDERS" => UTF8_NO_BORDERS,
            "UTF8_BORDERS_ONLY" => UTF8_BORDERS_ONLY,
            "UTF8_HORIZONTAL_ONLY" => UTF8_HORIZONTAL_ONLY,
            "NOTHING" => NOTHING,
            "DEFAULT" => UTF8_FULL_CONDENSED,
            _ => UTF8_FULL_CONDENSED,
        };

        let mut table = Table::new();
        table
            .load_preset(preset)
            .set_content_arrangement(ContentArrangement::Dynamic);

        let length = self.len();

        writeln!(f, "len: {}", length)?;

        if length == 0 {
            writeln!(f, "{}", table)?;
            return Ok(());
        }

        let reduced = max_len < length;
        let values = self.inner().values();

        if reduced {
            let table_len = max_len;
            let n_initial = if table_len % 2 == 0 {
                table_len / 2
            } else {
                table_len / 2 + 1
            };
            let n_last = table_len - n_initial;

            for i in values.iter().take(n_initial) {
                table.add_row(vec![i.to_string()]);
            }

            table.add_row(vec!["...".to_string()]);

            for i in values.iter().skip(length - n_last) {
                table.add_row(vec![i.to_string()]);
            }
        } else {
            let table_len = length;
            for i in values.iter().take(table_len) {
                table.add_row(vec![i.to_string()]);
            }
        }

        write!(f, "{}", table)?;

        Ok(())
    }
}

impl<T> fmt::Display for HComplexArray<T>
where
    T: NativeType + Float,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_len = std::env::var(FMT_MAX_LEN)
            .as_deref()
            .unwrap_or("")
            .parse::<usize>()
            .map(|x| std::cmp::max(x, 5))
            .unwrap_or(DEFAULT_FMT_MAX_LEN);
        let preset = match std::env::var(FMT_TABLE_FORMATTING)
            .as_deref()
            .unwrap_or(DEFAULT_FMT_TABLE_FORMATTING)
        {
            "ASCII_FULL" => ASCII_FULL,
            "ASCII_FULL_CONDENSED" => ASCII_FULL_CONDENSED,
            "ASCII_NO_BORDERS" => ASCII_NO_BORDERS,
            "ASCII_BORDERS_ONLY" => ASCII_BORDERS_ONLY,
            "ASCII_BORDERS_ONLY_CONDENSED" => ASCII_BORDERS_ONLY_CONDENSED,
            "ASCII_HORIZONTAL_ONLY" => ASCII_HORIZONTAL_ONLY,
            "ASCII_MARKDOWN" => ASCII_MARKDOWN,
            "UTF8_FULL" => UTF8_FULL,
            "UTF8_FULL_CONDENSED" => UTF8_FULL_CONDENSED,
            "UTF8_NO_BORDERS" => UTF8_NO_BORDERS,
            "UTF8_BORDERS_ONLY" => UTF8_BORDERS_ONLY,
            "UTF8_HORIZONTAL_ONLY" => UTF8_HORIZONTAL_ONLY,
            "NOTHING" => NOTHING,
            "DEFAULT" => UTF8_FULL_CONDENSED,
            _ => UTF8_FULL_CONDENSED,
        };

        let mut table = Table::new();
        table
            .load_preset(preset)
            .set_content_arrangement(ContentArrangement::Dynamic);

        let length = self.len();

        writeln!(f, "len: {}", length)?;

        if length == 0 {
            writeln!(f, "{}", table)?;
            return Ok(());
        }

        let reduced = max_len < length;
        let values = self.inner().values();

        if reduced {
            let table_len = max_len;
            let n_initial = if table_len % 2 == 0 {
                table_len / 2
            } else {
                table_len / 2 + 1
            };
            let n_last = table_len - n_initial;

            for i in values.chunks_exact(2).take(n_initial) {
                let s = if i[1] >= T::zero() || i[1].is_nan() {
                    format!("{}+{}i", i[0], i[1])
                } else {
                    format!("{}{}i", i[0], i[1])
                };
                table.add_row(vec![s]);
            }

            table.add_row(vec!["...".to_string()]);

            for i in values.chunks_exact(2).skip(length - n_last) {
                let s = if i[1] >= T::zero() || i[1].is_nan() {
                    format!("{}+{}i", i[0], i[1])
                } else {
                    format!("{}{}i", i[0], i[1])
                };
                table.add_row(vec![s]);
            }
        } else {
            let table_len = length;
            for i in values.chunks_exact(2).take(table_len) {
                let s = if i[1] >= T::zero() || i[1].is_nan() {
                    format!("{}+{}i", i[0], i[1])
                } else {
                    format!("{}{}i", i[0], i[1])
                };
                table.add_row(vec![s]);
            }
        }

        write!(f, "{}", table)?;

        Ok(())
    }
}

impl<T> fmt::Display for HFloatMatrix<T>
where
    T: NativeType + Float,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_cols = std::env::var(FMT_MAX_COLS)
            .as_deref()
            .unwrap_or("")
            .parse::<usize>()
            .map(|x| std::cmp::max(x, 5))
            .unwrap_or(DEFAULT_FMT_MAX_COLS);
        let max_rows = std::env::var(FMT_MAX_ROWS)
            .as_deref()
            .unwrap_or("")
            .parse::<usize>()
            .map(|x| std::cmp::max(x, 5))
            .unwrap_or(DEFAULT_FMT_MAX_ROWS);
        let preset = match std::env::var(FMT_TABLE_FORMATTING)
            .as_deref()
            .unwrap_or(DEFAULT_FMT_TABLE_FORMATTING)
        {
            "ASCII_FULL" => ASCII_FULL,
            "ASCII_FULL_CONDENSED" => ASCII_FULL_CONDENSED,
            "ASCII_NO_BORDERS" => ASCII_NO_BORDERS,
            "ASCII_BORDERS_ONLY" => ASCII_BORDERS_ONLY,
            "ASCII_BORDERS_ONLY_CONDENSED" => ASCII_BORDERS_ONLY_CONDENSED,
            "ASCII_HORIZONTAL_ONLY" => ASCII_HORIZONTAL_ONLY,
            "ASCII_MARKDOWN" => ASCII_MARKDOWN,
            "UTF8_FULL" => UTF8_FULL,
            "UTF8_FULL_CONDENSED" => UTF8_FULL_CONDENSED,
            "UTF8_NO_BORDERS" => UTF8_NO_BORDERS,
            "UTF8_BORDERS_ONLY" => UTF8_BORDERS_ONLY,
            "UTF8_HORIZONTAL_ONLY" => UTF8_HORIZONTAL_ONLY,
            "NOTHING" => NOTHING,
            "DEFAULT" => UTF8_FULL_CONDENSED,
            _ => UTF8_FULL_CONDENSED,
        };

        let mut table = Table::new();
        table
            .load_preset(preset)
            .set_content_arrangement(ContentArrangement::Dynamic);

        let length = self.len();
        let ncols = self.ncols();
        let nrows = self.nrows();

        writeln!(f, "(rows, cols): ({}, {})", nrows, ncols)?;

        if length == 0 {
            writeln!(f, "{}", table)?;
            return Ok(());
        }

        let reduced_cols = max_cols < ncols;
        let reduced_rows = max_rows < nrows;
        let values = self
            .inner()
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<T>>()
            .unwrap()
            .values();

        match (reduced_cols, reduced_rows) {
            (true, true) => {
                let table_rows = max_rows;
                let table_cols = max_cols;
                let n_initial_rows = if table_rows % 2 == 0 {
                    table_rows / 2
                } else {
                    table_rows / 2 + 1
                };
                let n_last_rows = table_rows - n_initial_rows;
                let n_initial_cols = if table_cols % 2 == 0 {
                    table_cols / 2
                } else {
                    table_cols / 2 + 1
                };
                let n_last_cols = table_cols - n_initial_cols;

                let mut v = Vec::with_capacity(n_initial_cols + n_last_cols + 1);
                for row in (0..(n_initial_rows + 1)).chain((nrows - n_last_rows)..nrows) {
                    for col in (0..n_initial_cols + 1).chain((ncols - n_last_cols)..ncols) {
                        if row == n_initial_rows {
                            v.push("...".to_string());
                            continue;
                        }

                        if col == n_initial_cols {
                            v.push("...".to_string());
                            continue;
                        }

                        v.push(values[row + col * nrows].to_string());
                    }

                    table.add_row(&v);
                    v.clear();
                }
            }
            (true, false) => {
                let table_cols = max_cols;
                let n_initial_cols = if table_cols % 2 == 0 {
                    table_cols / 2
                } else {
                    table_cols / 2 + 1
                };
                let n_last_cols = table_cols - n_initial_cols;

                let mut v = Vec::with_capacity(n_initial_cols + n_last_cols + 1);
                for row in 0..nrows {
                    for col in (0..n_initial_cols + 1).chain((ncols - n_last_cols)..ncols) {
                        if col == n_initial_cols {
                            v.push("...".to_string());
                            continue;
                        }

                        v.push(values[row + col * nrows].to_string());
                    }

                    table.add_row(&v);
                    v.clear();
                }
            }
            (false, true) => {
                let table_rows = max_rows;
                let n_initial_rows = if table_rows % 2 == 0 {
                    table_rows / 2
                } else {
                    table_rows / 2 + 1
                };
                let n_last_rows = table_rows - n_initial_rows;

                let mut v = Vec::with_capacity(ncols);
                for row in (0..(n_initial_rows + 1)).chain((nrows - n_last_rows)..nrows) {
                    for col in 0..ncols {
                        if row == n_initial_rows {
                            v.push("...".to_string());
                            continue;
                        }

                        v.push(values[row + col * nrows].to_string());
                    }

                    table.add_row(&v);
                    v.clear();
                }
            }
            (false, false) => {
                let mut v = Vec::with_capacity(ncols);
                for row in 0..nrows {
                    for col in 0..ncols {
                        v.push(values[row + col * nrows].to_string());
                    }

                    table.add_row(&v);
                    v.clear();
                }
            }
        }

        write!(f, "{}", table)?;

        Ok(())
    }
}

impl<T> fmt::Display for HComplexMatrix<T>
where
    T: NativeType + Float,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max_cols = std::env::var(FMT_MAX_COLS)
            .as_deref()
            .unwrap_or("")
            .parse::<usize>()
            .map(|x| std::cmp::max(x, 5))
            .unwrap_or(DEFAULT_FMT_MAX_COLS);
        let max_rows = std::env::var(FMT_MAX_ROWS)
            .as_deref()
            .unwrap_or("")
            .parse::<usize>()
            .map(|x| std::cmp::max(x, 5))
            .unwrap_or(DEFAULT_FMT_MAX_ROWS);
        let preset = match std::env::var(FMT_TABLE_FORMATTING)
            .as_deref()
            .unwrap_or(DEFAULT_FMT_TABLE_FORMATTING)
        {
            "ASCII_FULL" => ASCII_FULL,
            "ASCII_FULL_CONDENSED" => ASCII_FULL_CONDENSED,
            "ASCII_NO_BORDERS" => ASCII_NO_BORDERS,
            "ASCII_BORDERS_ONLY" => ASCII_BORDERS_ONLY,
            "ASCII_BORDERS_ONLY_CONDENSED" => ASCII_BORDERS_ONLY_CONDENSED,
            "ASCII_HORIZONTAL_ONLY" => ASCII_HORIZONTAL_ONLY,
            "ASCII_MARKDOWN" => ASCII_MARKDOWN,
            "UTF8_FULL" => UTF8_FULL,
            "UTF8_FULL_CONDENSED" => UTF8_FULL_CONDENSED,
            "UTF8_NO_BORDERS" => UTF8_NO_BORDERS,
            "UTF8_BORDERS_ONLY" => UTF8_BORDERS_ONLY,
            "UTF8_HORIZONTAL_ONLY" => UTF8_HORIZONTAL_ONLY,
            "NOTHING" => NOTHING,
            "DEFAULT" => UTF8_FULL_CONDENSED,
            _ => UTF8_FULL_CONDENSED,
        };

        let mut table = Table::new();
        table
            .load_preset(preset)
            .set_content_arrangement(ContentArrangement::Dynamic);

        let length = self.len();
        let ncols = self.ncols();
        let nrows = self.nrows();

        writeln!(f, "(rows, cols): ({}, {})", nrows, ncols)?;

        if length == 0 {
            writeln!(f, "{}", table)?;
            return Ok(());
        }

        let reduced_cols = max_cols < ncols;
        let reduced_rows = max_rows < nrows;
        let values = self
            .inner()
            .values()
            .as_any()
            .downcast_ref::<PrimitiveArray<T>>()
            .unwrap()
            .values();

        match (reduced_cols, reduced_rows) {
            (true, true) => {
                let table_rows = max_rows;
                let table_cols = max_cols;
                let n_initial_rows = if table_rows % 2 == 0 {
                    table_rows / 2
                } else {
                    table_rows / 2 + 1
                };
                let n_last_rows = table_rows - n_initial_rows;

                let n_initial_cols = if table_cols % 2 == 0 {
                    table_cols / 2
                } else {
                    table_cols / 2 + 1
                };
                let n_last_cols = table_cols - n_initial_cols;

                let mut v = Vec::with_capacity(n_initial_cols + n_last_cols + 1);
                for row in (0..(n_initial_rows + 1)).chain((nrows - n_last_rows)..nrows) {
                    for col in (0..n_initial_cols + 1).chain((ncols - n_last_cols)..ncols) {
                        if row == n_initial_rows {
                            v.push("...".to_string());
                            continue;
                        }

                        if col == n_initial_cols {
                            v.push("...".to_string());
                            continue;
                        }

                        let v_re = values[(row + col * nrows) * 2];
                        let v_im = values[(row + col * nrows) * 2 + 1];
                        let s = if v_im >= T::zero() || v_im.is_nan() {
                            format!("{}+{}i", v_re, v_im)
                        } else {
                            format!("{}{}i", v_re, v_im)
                        };

                        v.push(s);
                    }

                    table.add_row(&v);
                    v.clear();
                }
            }
            (true, false) => {
                let table_cols = max_cols;
                let n_initial_cols = if table_cols % 2 == 0 {
                    table_cols / 2
                } else {
                    table_cols / 2 + 1
                };
                let n_last_cols = table_cols - n_initial_cols;

                let mut v = Vec::with_capacity(ncols);
                for row in 0..nrows {
                    for col in (0..n_initial_cols + 1).chain((ncols - n_last_cols)..ncols) {
                        if col == n_initial_cols {
                            v.push("...".to_string());
                            continue;
                        }

                        let v_re = values[(row + col * nrows) * 2];
                        let v_im = values[(row + col * nrows) * 2 + 1];
                        let s = if v_im >= T::zero() || v_im.is_nan() {
                            format!("{}+{}i", v_re, v_im)
                        } else {
                            format!("{}{}i", v_re, v_im)
                        };

                        v.push(s);
                    }

                    table.add_row(&v);
                    v.clear();
                }
            }
            (false, true) => {
                let table_rows = max_rows;
                let n_initial_rows = if table_rows % 2 == 0 {
                    table_rows / 2
                } else {
                    table_rows / 2 + 1
                };
                let n_last_rows = table_rows - n_initial_rows;

                let mut v = Vec::with_capacity(ncols);
                for row in (0..(n_initial_rows + 1)).chain((nrows - n_last_rows)..nrows) {
                    for col in 0..ncols {
                        if row == n_initial_rows {
                            v.push("...".to_string());
                            continue;
                        }

                        let v_re = values[(row + col * nrows) * 2];
                        let v_im = values[(row + col * nrows) * 2 + 1];
                        let s = if v_im >= T::zero() || v_im.is_nan() {
                            format!("{}+{}i", v_re, v_im)
                        } else {
                            format!("{}{}i", v_re, v_im)
                        };

                        v.push(s);
                    }

                    table.add_row(&v);
                    v.clear();
                }
            }
            (false, false) => {
                let mut v = Vec::with_capacity(ncols);
                for row in 0..nrows {
                    for col in 0..ncols {
                        let v_re = values[(row + col * nrows) * 2];
                        let v_im = values[(row + col * nrows) * 2 + 1];
                        let s = if v_im >= T::zero() || v_im.is_nan() {
                            format!("{}+{}i", v_re, v_im)
                        } else {
                            format!("{}{}i", v_re, v_im)
                        };

                        v.push(s);
                    }

                    table.add_row(&v);
                    v.clear();
                }
            }
        }

        write!(f, "{}", table)?;

        Ok(())
    }
}

impl<T> fmt::Display for HFloatAudio<T>
where
    T: NativeType + Float,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "sr: {}", self.sr())?;
        write!(f, "{}", self.inner())?;
        Ok(())
    }
}
