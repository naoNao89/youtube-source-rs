use super::JavaScriptEngineError;
use crate::Result;
use regex::Regex;
use std::sync::OnceLock;

/// Extracted cipher information from YouTube player script
#[derive(Debug, Clone)]
pub struct ExtractedCipher {
    pub timestamp: String,
    pub global_vars: String,
    pub sig_actions: String,
    pub sig_function: String,
    pub n_function: String,
    pub raw_script: String,
}

/// YouTube player script parser for extracting cipher functions
pub struct ScriptParser;

impl ScriptParser {
    /// Extract cipher information from YouTube player script
    pub fn extract_cipher_from_script(script: &str) -> Result<ExtractedCipher> {
        // Extract timestamp
        let timestamp = Self::extract_timestamp(script)?;

        // Extract global variables
        let global_vars = Self::extract_global_vars(script)?;

        // Extract signature actions
        let sig_actions = Self::extract_sig_actions(script)?;

        // Extract signature function
        let sig_function = Self::extract_sig_function(script)?;

        // Extract N parameter function
        let n_function = Self::extract_n_function(script)?;

        Ok(ExtractedCipher {
            timestamp,
            global_vars,
            sig_actions,
            sig_function,
            n_function: Self::clean_n_function(&n_function),
            raw_script: script.to_string(),
        })
    }

    /// Extract timestamp from script
    pub fn extract_timestamp(script: &str) -> Result<String> {
        static TIMESTAMP_REGEX: OnceLock<Regex> = OnceLock::new();
        let regex = TIMESTAMP_REGEX
            .get_or_init(|| Regex::new(r"(signatureTimestamp|sts):\s*(\d+)").unwrap());

        regex
            .captures(script)
            .and_then(|caps| caps.get(2))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| {
                JavaScriptEngineError::CompilationError("Timestamp not found in script".to_string())
                    .into()
            })
    }

    /// Extract global variables from script
    pub fn extract_global_vars(script: &str) -> Result<String> {
        static GLOBAL_VARS_REGEX: OnceLock<Regex> = OnceLock::new();
        let regex = GLOBAL_VARS_REGEX.get_or_init(|| {
            Regex::new(
                r#"(?x)
                ('use\s*strict';)?
                (?P<code>var\s*(?P<varname>[a-zA-Z0-9_$]+)\s*=\s*
                (?P<value>(?:"[^"\\]*(?:\\.[^"\\]*)*"|'[^'\\]*(?:\\.[^'\\]*)*')
                \.split\((?:"[^"\\]*(?:\\.[^"\\]*)*"|'[^'\\]*(?:\\.[^'\\]*)*')\)
                |\[(?:(?:"[^"\\]*(?:\\.[^"\\]*)*"|'[^'\\]*(?:\\.[^'\\]*)*')\s*,?\s*)*\]
                |"[^"]*"\.split\("[^"]*"\)))
            "#,
            )
            .unwrap()
        });

        regex
            .captures(script)
            .and_then(|caps| caps.name("code"))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| {
                JavaScriptEngineError::CompilationError(
                    "Global variables not found in script".to_string(),
                )
                .into()
            })
    }

    /// Extract signature actions from script
    fn extract_sig_actions(script: &str) -> Result<String> {
        static SIG_ACTIONS_REGEX: OnceLock<Regex> = OnceLock::new();
        let regex = SIG_ACTIONS_REGEX.get_or_init(|| {
            Regex::new(
                r#"(?x)
                var\s+([$A-Za-z0-9_]+)\s*=\s*\{\s*
                [$A-Za-z0-9_]+\s*:\s*function\s*\([^)]*\)\s*\{[^{}]*(?:\{[^{}]*}[^{}]*)*}\s*,\s*
                [$A-Za-z0-9_]+\s*:\s*function\s*\([^)]*\)\s*\{[^{}]*(?:\{[^{}]*}[^{}]*)*}\s*,\s*
                [$A-Za-z0-9_]+\s*:\s*function\s*\([^)]*\)\s*\{[^{}]*(?:\{[^{}]*}[^{}]*)*}\s*
                \};
            "#,
            )
            .unwrap()
        });

        regex
            .find(script)
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| {
                JavaScriptEngineError::CompilationError(
                    "Signature actions not found in script".to_string(),
                )
                .into()
            })
    }

    /// Extract signature function from script
    fn extract_sig_function(script: &str) -> Result<String> {
        static SIG_FUNCTION_REGEX: OnceLock<Regex> = OnceLock::new();
        let regex = SIG_FUNCTION_REGEX.get_or_init(|| {
            Regex::new(
                r#"(?x)
                function(?:\s+[a-zA-Z_\$][a-zA-Z_0-9\$]*)?
                \(([a-zA-Z_\$][a-zA-Z_0-9\$]*)\)
                \{[a-zA-Z_\$][a-zA-Z_0-9\$]*=[a-zA-Z_\$][a-zA-Z_0-9\$]*.*?\([a-zA-Z_\$][a-zA-Z_0-9\$]*,\d+\);
                return\s*[a-zA-Z_\$][a-zA-Z_0-9\$]*.*};
            "#,
            )
            .unwrap()
        });

        regex
            .find(script)
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| {
                JavaScriptEngineError::CompilationError(
                    "Signature function not found in script".to_string(),
                )
                .into()
            })
    }

    /// Extract N parameter function from script
    fn extract_n_function(script: &str) -> Result<String> {
        static N_FUNCTION_REGEX: OnceLock<Regex> = OnceLock::new();
        let regex = N_FUNCTION_REGEX.get_or_init(|| {
            Regex::new(r#"(?xs)
                function\(\s*([a-zA-Z_\$][a-zA-Z_0-9\$]*)\s*\)\s*\{
                var\s*([a-zA-Z_\$][a-zA-Z_0-9\$]*)=[a-zA-Z_\$][a-zA-Z_0-9\$]*\[[a-zA-Z_\$][a-zA-Z_0-9\$]*\[\d+\]\]\([a-zA-Z_\$][a-zA-Z_0-9\$]*\[\d+\]\)
                .*?catch\(\s*(\w+)\s*\)\s*\{
                \s*return.*?\+\s*[a-zA-Z_\$][a-zA-Z_0-9\$]*\s*}
                \s*return\s*[a-zA-Z_\$][a-zA-Z_0-9\$]*\[[a-zA-Z_\$][a-zA-Z_0-9\$]*\[\d+\]\]\([a-zA-Z_\$][a-zA-Z_0-9\$]*\[\d+\]\)};
            "#).unwrap()
        });

        // Try primary pattern first
        if let Some(m) = regex.find(script) {
            return Ok(m.as_str().to_string());
        }

        // Try fallback pattern for older scripts
        static N_FUNCTION_OLD_REGEX: OnceLock<Regex> = OnceLock::new();
        let old_regex = N_FUNCTION_OLD_REGEX.get_or_init(|| {
            Regex::new(r#"(?xs)
                function\(\s*(\w+)\s*\)\s*\{
                var\s*(\w+)=\w+\[[a-zA-Z_\$][a-zA-Z_0-9\$]*\[\d+\]\]\([a-zA-Z_\$][a-zA-Z_0-9\$]*\[\d+\]\)
                .*?catch\(\s*(\w+)\s*\)\s*\{
                \s*return.*?\+\s*\w+\s*}
                \s*return\s*\w+\[[a-zA-Z_\$][a-zA-Z_0-9\$]*\[\d+\]\]\([a-zA-Z_\$][a-zA-Z_0-9\$]*\[\d+\]\)};
            "#).unwrap()
        });

        old_regex
            .find(script)
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| {
                JavaScriptEngineError::CompilationError(
                    "N parameter function not found in script".to_string(),
                )
                .into()
            })
    }

    /// Clean N function by removing short-circuit patterns
    fn clean_n_function(n_function: &str) -> String {
        // Extract parameter name from function signature
        let param_name = Self::extract_parameter_name(n_function);

        // Remove short-circuit that prevents n challenge transformation
        let short_circuit_pattern = format!(
            r"if\s*\(\s*typeof\s+\w+\s*===?\s*[^)]+\)\s*return\s+{}\s*;?",
            regex::escape(&param_name)
        );

        if let Ok(regex) = Regex::new(&short_circuit_pattern) {
            regex.replace_all(n_function, "").to_string()
        } else {
            n_function.to_string()
        }
    }

    /// Extract parameter name from function signature
    fn extract_parameter_name(function: &str) -> String {
        static PARAM_REGEX: OnceLock<Regex> = OnceLock::new();
        let regex =
            PARAM_REGEX.get_or_init(|| Regex::new(r"function\s*\(\s*([^)]+)\s*\)").unwrap());

        regex
            .captures(function)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_else(|| "a".to_string()) // fallback parameter name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_timestamp() {
        let script = r#"
            var config = {
                signatureTimestamp: 19834,
                other: "value"
            };
        "#;

        let result = ScriptParser::extract_timestamp(script);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "19834");
    }

    #[test]
    fn test_extract_timestamp_sts() {
        let script = r#"
            var config = {
                sts: 19834,
                other: "value"
            };
        "#;

        let result = ScriptParser::extract_timestamp(script);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "19834");
    }

    #[test]
    fn test_extract_global_vars() {
        let script = r#"
            var a = "abcdefghijklmnopqrstuvwxyz".split("");
            var other = "something";
        "#;

        let result = ScriptParser::extract_global_vars(script);
        assert!(result.is_ok());
        let vars = result.unwrap();
        assert!(vars.contains("var a"));
        assert!(vars.contains("split"));
    }

    #[test]
    fn test_parameter_name_extraction() {
        let function = "function(a) { return a; }";
        let param = ScriptParser::extract_parameter_name(function);
        assert_eq!(param, "a");

        let function2 = "function( param_name ) { return param_name; }";
        let param2 = ScriptParser::extract_parameter_name(function2);
        assert_eq!(param2, "param_name");
    }

    #[test]
    fn test_clean_n_function() {
        let n_function = r#"
            function(a) {
                if (typeof b === "undefined") return a;
                var c = a[d[0]](d[1]);
                return c;
            }
        "#;

        let cleaned = ScriptParser::clean_n_function(n_function);
        assert!(!cleaned.contains("if (typeof"));
        assert!(cleaned.contains("var c"));
    }
}
