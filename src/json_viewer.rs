use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashSet;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

pub struct JsonViewer {
    pub content: Option<String>,
    pub parsed_json: Option<Value>,
    pub current_file: Option<PathBuf>,
}

impl JsonViewer {
    pub fn new() -> Self {
        Self {
            content: None,
            parsed_json: None,
            current_file: None,
        }
    }

    pub fn load_file(&mut self, path: &PathBuf) -> Result<()> {
        let content = fs::read_to_string(path)
            .context("Failed to read file")?;

        // Try to parse as JSON
        let parsed = serde_json::from_str(&content).ok();

        self.content = Some(content);
        self.parsed_json = parsed;
        self.current_file = Some(path.clone());

        Ok(())
    }

    pub fn get_plain_text(&self) -> Option<&str> {
        self.content.as_deref()
    }

/*     pub fn get_hierarchical_view(&self) -> Option<String> {
        self.parsed_json
            .as_ref()
            .map(|json| self.format_json_hierarchical(json, 0))
    } */

    pub fn get_hierarchical_view_with_depth(&self, max_depth: Option<usize>) -> Option<String> {
        self.parsed_json
            .as_ref()
            .map(|json| self.format_json_hierarchical_with_depth(json, 0, max_depth))
    }

    pub fn get_hierarchical_view_with_collapse(
        &self,
        collapsed_paths: &HashSet<String>,
    ) -> Option<(String, Vec<String>)> {
        self.parsed_json.as_ref().map(|json| {
            let mut line_paths = Vec::new();
            let content = self.format_json_with_collapse(json, "", 0, collapsed_paths, &mut line_paths);
            (content, line_paths)
        })
    }

    fn format_json_with_collapse(
        &self,
        value: &Value,
        path: &str,
        indent: usize,
        collapsed_paths: &HashSet<String>,
        line_paths: &mut Vec<String>,
    ) -> String {
        let indent_str = "  ".repeat(indent);

        match value {
            Value::Object(map) => {
                let mut result = String::new();
                for (key, val) in map {
                    let key_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{path}.{key}")
                    };

                    let _ = write!(result, "{indent_str}{key}: ");
                    line_paths.push(key_path.clone());

                    match val {
                        Value::Object(_) | Value::Array(_) => {
                            if collapsed_paths.contains(&key_path) {
                                // Show collapsed indicator
                                match val {
                                    Value::Object(o) => {
                                        let _ = writeln!(result, "{{...}}  // {} keys", o.len());
                                    }
                                    Value::Array(a) => {
                                        let _ = writeln!(result, "[...]  // {} items", a.len());
                                    }
                                    _ => {}
                                }
                            } else {
                                result.push('\n');
                                result.push_str(&self.format_json_with_collapse(
                                    val,
                                    &key_path,
                                    indent + 1,
                                    collapsed_paths,
                                    line_paths,
                                ));
                            }
                        }
                        _ => {
                            let _ = writeln!(result, "{}", self.format_simple_value(val));
                        }
                    }
                }
                result
            }
            Value::Array(arr) => {
                let mut result = String::new();
                for (idx, val) in arr.iter().enumerate() {
                    let idx_path = if path.is_empty() {
                        format!("[{idx}]")
                    } else {
                        format!("{path}[{idx}]")
                    };

                    let _ = write!(result, "{indent_str}[{idx}]: ");
                    line_paths.push(idx_path.clone());

                    match val {
                        Value::Object(_) | Value::Array(_) => {
                            if collapsed_paths.contains(&idx_path) {
                                // Show collapsed indicator
                                match val {
                                    Value::Object(o) => {
                                        let _ = writeln!(result, "{{...}}  // {} keys", o.len());
                                    }
                                    Value::Array(a) => {
                                        let _ = writeln!(result, "[...]  // {} items", a.len());
                                    }
                                    _ => {}
                                }
                            } else {
                                result.push('\n');
                                result.push_str(&self.format_json_with_collapse(
                                    val,
                                    &idx_path,
                                    indent + 1,
                                    collapsed_paths,
                                    line_paths,
                                ));
                            }
                        }
                        _ => {
                            let _ = writeln!(result, "{}", self.format_simple_value(val));
                        }
                    }
                }
                result
            }
            _ => {
                line_paths.push(path.to_string());
                format!("{indent_str}{}\n", self.format_simple_value(value))
            }
        }
    }

    fn format_json_hierarchical_with_depth(
        &self,
        value: &Value,
        indent: usize,
        max_depth: Option<usize>,
    ) -> String {
        let indent_str = "  ".repeat(indent);
        if let Some(max) = max_depth {
            if indent >= max {
                return match value {
                    Value::Object(map) => format!("{indent_str}{{...}}  // {} keys\n", map.len()),
                    Value::Array(arr) => format!("{indent_str}[...]  // {} items\n", arr.len()),
                    _ => format!("{indent_str}{}\n", self.format_simple_value(value)),
                };
            }
        }

        match value {
            Value::Object(map) => {
                let mut result = String::new();
                for (key, val) in map {
                    let _ = write!(result, "{indent_str}{key}: ");
                    match val {
                        Value::Object(_) | Value::Array(_) => {
                            result.push('\n');
                            result.push_str(&self.format_json_hierarchical_with_depth(
                                val,
                                indent + 1,
                                max_depth,
                            ));
                        }
                        _ => {
                            let _ = writeln!(result, "{}", self.format_simple_value(val));
                        }
                    }
                }
                result
            }
            Value::Array(arr) => {
                let mut result = String::new();
                for (idx, val) in arr.iter().enumerate() {
                    let _ = write!(result, "{indent_str}[{idx}]: ");
                    match val {
                        Value::Object(_) | Value::Array(_) => {
                            result.push('\n');
                            result.push_str(&self.format_json_hierarchical_with_depth(
                                val,
                                indent + 1,
                                max_depth,
                            ));
                        }
                        _ => {
                            let _ = writeln!(result, "{}", self.format_simple_value(val));
                        }
                    }
                }
                result
            }
            _ => format!("{indent_str}{}\n", self.format_simple_value(value)),
        }
    }

    fn format_simple_value(&self, value: &Value) -> String {
        match value {
            Value::String(s) => format!("\"{s}\""),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            _ => value.to_string(),
        }
    }

    pub fn query(&self, query: &str) -> Result<String> {
        if let Some(json) = &self.parsed_json {
            let normalized = Self::normalize_query(query);
            let result = Self::execute_simple_query(json, &normalized);
            Ok(serde_json::to_string_pretty(&result)?)
        } else {
            Ok("No JSON data loaded or file is not valid JSON".to_string())
        }
    }

    fn normalize_query(query: &str) -> String {
        let mut q = query.trim().to_string();
        // Strip surrounding quotes if present
        if (q.starts_with('"') && q.ends_with('"')) || (q.starts_with('\'') && q.ends_with('\'')) {
            q = q[1..q.len().saturating_sub(1)].to_string();
        }
        // Strip leading dot
        if q.starts_with('.') { q.remove(0); }
        q
    }

    fn execute_simple_query(value: &Value, query: &str) -> Value {
        // Supports basic dot/bracket notation:
        // examples: name, user.address.city, users[0].name, users[].name
        #[derive(Debug, Clone)]
        enum Token {
            Key(String),
            Index(usize),
            Wildcard,
        }

        fn parse_segment(seg: &str) -> Vec<Token> {
            let mut tokens = Vec::new();
            if seg.is_empty() { return tokens; }
            // Split key and bracket parts, e.g., users[0][1] or users[]
            let mut key = String::new();
            let mut rest = seg;
            // Extract leading identifier
            for (i, ch) in seg.char_indices() {
                if ch == '[' { rest = &seg[i..]; break; }
                key.push(ch);
                if i == seg.len() - 1 { rest = &seg[seg.len()..]; }
            }
            if !key.is_empty() { tokens.push(Token::Key(key)); }
            // Parse bracket expressions
            let mut r = rest;
            while let Some(pos) = r.find('[') {
                if let Some(end) = r[pos..].find(']') {
                    let inner = &r[pos+1 .. pos+end];
                    if inner.trim().is_empty() || inner.trim() == "*" { tokens.push(Token::Wildcard); }
                    else if let Ok(idx) = inner.trim().parse::<usize>() { tokens.push(Token::Index(idx)); }
                    r = &r[pos+end+1 ..];
                } else { break; }
            }
            tokens
        }

        fn parse_query(q: &str) -> Vec<Token> {
            let mut tokens = Vec::new();
            for seg in q.split('.') { tokens.extend(parse_segment(seg)); }
            tokens
        }

        fn walk(value: &Value, tokens: &[Token]) -> Vec<Value> {
            if tokens.is_empty() { return vec![value.clone()]; }
            match &tokens[0] {
                Token::Key(k) => {
                    if let Value::Object(map) = value {
                        if let Some(v) = map.get(k) { return walk(v, &tokens[1..]); }
                    }
                    vec![Value::Null]
                }
                Token::Index(i) => {
                    if let Value::Array(arr) = value {
                        if let Some(v) = arr.get(*i) { return walk(v, &tokens[1..]); }
                    }
                    vec![Value::Null]
                }
                Token::Wildcard => {
                    if let Value::Array(arr) = value {
                        let mut out = Vec::new();
                        for v in arr {
                            out.extend(walk(v, &tokens[1..]));
                        }
                        return out;
                    }
                    vec![Value::Null]
                }
            }
        }

        let tokens = parse_query(query);
        let results = walk(value, &tokens);
        if results.len() == 1 { results.into_iter().next().unwrap_or(Value::Null) }
        else { Value::Array(results) }
    }
}
