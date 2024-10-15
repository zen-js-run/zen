use std::fs;
use std::io::Error;
use regex::Regex;

pub struct Formatter {
    indent: String,
    line_length: usize,
    quote_style: QuoteStyle,
    trailing_commas: bool,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum QuoteStyle {
    Single,
    Double,
}

impl Formatter {
    pub fn new(indent_size: usize, line_length: usize, quote_style: QuoteStyle, trailing_commas: bool) -> Self {
        Self {
            indent: " ".repeat(indent_size),
            line_length,
            quote_style,
            trailing_commas,
        }
    }

    pub fn format_js(&self, file_path: &str) -> Result<(), Error> {
        let content = fs::read_to_string(file_path)?;
        let formatted_content = self.format_code(&content);
        fs::write(file_path, formatted_content)?;
        Ok(())
    }

    fn format_code(&self, code: &str) -> String {
        let mut result = String::new();
        let mut indent_level = 0;
        let mut inside_comment = false;

        for line in code.lines() {
            let trimmed_line = line.trim();

            if trimmed_line.is_empty() {
                result.push('\n');
                continue;
            }

            if trimmed_line.starts_with("/*") {
                inside_comment = true;
                result.push_str(&format!("{}{}\n", self.indent.repeat(indent_level), trimmed_line));
                continue;
            }

            if inside_comment {
                result.push_str(&format!("{}{}\n", self.indent.repeat(indent_level), trimmed_line));
                if trimmed_line.ends_with("*/") {
                    inside_comment = false;
                }
                continue;
            }

            if trimmed_line.starts_with("//") {
                result.push_str(&format!("{}{}\n", self.indent.repeat(indent_level), trimmed_line));
                continue;
            }

            if trimmed_line.ends_with('{') {
                result.push_str(&format!("{}{}\n", self.indent.repeat(indent_level), trimmed_line));
                indent_level += 1;
                continue;
            }

            if trimmed_line == "}" {
                indent_level -= 1;
                result.push_str(&format!("{}{}\n", self.indent.repeat(indent_level), trimmed_line));
                continue;
            }

            let formatted_line = self.format_line(trimmed_line);
            result.push_str(&format!("{}{}\n", self.indent.repeat(indent_level), formatted_line));
        }

        self.break_lines(result)
    }

    fn format_line(&self, line: &str) -> String {
        let mut formatted_line = line.to_string();
        formatted_line = self.replace_quotes(formatted_line);
        formatted_line = Regex::new(r"\s+").unwrap().replace_all(&formatted_line, " ").to_string();
        formatted_line = self.format_arrow_functions(formatted_line);
        formatted_line = self.ensure_semicolon(formatted_line);
        if self.trailing_commas {
            formatted_line = self.add_trailing_commas(formatted_line);
        }
        formatted_line.trim().to_string()
    }

    fn replace_quotes(&self, line: String) -> String {
        let quote_char = match self.quote_style {
            QuoteStyle::Single => "'",
            QuoteStyle::Double => "\"",
        };

        Regex::new(r#""([^"]*)""#)
            .unwrap()
            .replace_all(&line, quote_char)
            .to_string()
    }

    fn format_arrow_functions(&self, line: String) -> String {
        Regex::new(r"(\w+)\s*=>")
            .unwrap()
            .replace_all(&line, |caps: &regex::Captures| format!("{} =>", &caps[1]))
            .to_string()
    }

    fn ensure_semicolon(&self, line: String) -> String {
        if line.ends_with(';') || line.ends_with('}') || line.ends_with('{') {
            line
        } else {
            format!("{};", line)
        }
    }

    fn add_trailing_commas(&self, line: String) -> String {
        Regex::new(r#"(,\s*\})|(\s*})"#)
            .unwrap()
            .replace_all(&line, ", }")
            .to_string()
    }

    fn break_lines(&self, content: String) -> String {
        let mut result = String::new();
        for line in content.lines() {
            let mut current_line = String::new();
            for word in line.split_whitespace() {
                if current_line.len() + word.len() + 1 > self.line_length {
                    result.push_str(current_line.trim());
                    result.push('\n');
                    current_line.clear();
                }
                current_line.push_str(word);
                current_line.push(' ');
            }
            result.push_str(current_line.trim());
            result.push('\n');
        }
        result
    }
}