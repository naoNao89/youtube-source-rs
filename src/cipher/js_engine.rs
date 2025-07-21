use crate::Result;
use rquickjs::{Context, Function, Runtime, Value};
use std::time::Instant;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JavaScriptEngineError {
    #[error("JavaScript runtime error: {0}")]
    RuntimeError(String),
    #[error("Function not found: {0}")]
    FunctionNotFound(String),
    #[error("Invalid return type: expected string")]
    InvalidReturnType,
    #[error("Script compilation failed: {0}")]
    CompilationError(String),
    #[error("Execution timeout")]
    ExecutionTimeout,
}

/// JavaScript engine wrapper for executing YouTube cipher operations
pub struct JavaScriptEngine {
    runtime: Runtime,
}

impl std::fmt::Debug for JavaScriptEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JavaScriptEngine")
            .field("runtime", &"<QuickJS Runtime>")
            .finish()
    }
}

impl Clone for JavaScriptEngine {
    fn clone(&self) -> Self {
        // Create a new runtime for the cloned instance
        Self::new().expect("Failed to create cloned JavaScript engine")
    }
}

impl JavaScriptEngine {
    /// Create a new JavaScript engine instance
    pub fn new() -> Result<Self> {
        let runtime =
            Runtime::new().map_err(|e| JavaScriptEngineError::RuntimeError(e.to_string()))?;

        Ok(Self { runtime })
    }

    /// Execute a cipher function with the given signature
    pub fn execute_cipher_function(
        &self,
        script: &str,
        function_name: &str,
        signature: &str,
    ) -> Result<String> {
        let start_time = Instant::now();

        let context = Context::full(&self.runtime)
            .map_err(|e| JavaScriptEngineError::RuntimeError(e.to_string()))?;

        context.with(|ctx| {
            // Execute the script to define functions
            ctx.eval::<(), _>(script)
                .map_err(|e| JavaScriptEngineError::CompilationError(e.to_string()))?;

            // Get the cipher function
            let function: Function = ctx
                .globals()
                .get(function_name)
                .map_err(|_| JavaScriptEngineError::FunctionNotFound(function_name.to_string()))?;

            // Call the function with the signature
            let result: Value = function
                .call((signature,))
                .map_err(|e| JavaScriptEngineError::RuntimeError(e.to_string()))?;

            // Convert result to string
            let deciphered_signature: String = result
                .as_string()
                .ok_or(JavaScriptEngineError::InvalidReturnType)?
                .to_string()
                .map_err(|e| JavaScriptEngineError::RuntimeError(e.to_string()))?;

            // Check for timeout (should be <50ms for performance)
            let execution_time = start_time.elapsed();
            if execution_time.as_millis() > 100 {
                log::warn!(
                    "Cipher execution took {}ms, target is <50ms",
                    execution_time.as_millis()
                );
            }

            Ok(deciphered_signature)
        })
    }

    /// Execute an N parameter transformation function
    pub fn execute_n_transform_function(
        &self,
        script: &str,
        function_name: &str,
        n_parameter: &str,
    ) -> Result<String> {
        let start_time = Instant::now();

        let context = Context::full(&self.runtime)
            .map_err(|e| JavaScriptEngineError::RuntimeError(e.to_string()))?;

        context.with(|ctx| {
            // Execute the script to define functions
            ctx.eval::<(), _>(script)
                .map_err(|e| JavaScriptEngineError::CompilationError(e.to_string()))?;

            // Get the N transform function
            let function: Function = ctx
                .globals()
                .get(function_name)
                .map_err(|_| JavaScriptEngineError::FunctionNotFound(function_name.to_string()))?;

            // Call the function with the N parameter
            let result: Value = function
                .call((n_parameter,))
                .map_err(|e| JavaScriptEngineError::RuntimeError(e.to_string()))?;

            // Convert result to string
            let transformed_n: String = result
                .as_string()
                .ok_or(JavaScriptEngineError::InvalidReturnType)?
                .to_string()
                .map_err(|e| JavaScriptEngineError::RuntimeError(e.to_string()))?;

            // Check for timeout
            let execution_time = start_time.elapsed();
            if execution_time.as_millis() > 100 {
                log::warn!(
                    "N transform execution took {}ms, target is <50ms",
                    execution_time.as_millis()
                );
            }

            Ok(transformed_n)
        })
    }

    /// Test the JavaScript engine with a simple operation
    pub fn test_engine(&self) -> Result<()> {
        let test_script = r#"
            function testFunction(input) {
                return input.split('').reverse().join('');
            }
        "#;

        let result = self.execute_cipher_function(test_script, "testFunction", "hello")?;

        if result == "olleh" {
            log::info!(
                "JavaScript engine test passed: '{}' -> '{}'",
                "hello",
                result
            );
            Ok(())
        } else {
            Err(JavaScriptEngineError::RuntimeError(format!(
                "Test failed: expected 'olleh', got '{result}'"
            ))
            .into())
        }
    }
}

impl Default for JavaScriptEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create JavaScript engine")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_javascript_engine_creation() {
        let engine = JavaScriptEngine::new();
        assert!(engine.is_ok());
    }

    #[test]
    fn test_simple_function_execution() {
        let engine = JavaScriptEngine::new().unwrap();

        let script = r#"
            function reverse(str) {
                return str.split('').reverse().join('');
            }
        "#;

        let result = engine.execute_cipher_function(script, "reverse", "test");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "tset");
    }

    #[test]
    fn test_cipher_like_operation() {
        let engine = JavaScriptEngine::new().unwrap();

        // Simulate a simple cipher operation similar to YouTube's
        let script = r#"
            function decipher(signature) {
                var a = signature.split('');
                // Reverse
                a.reverse();
                // Swap first and second characters
                var temp = a[0];
                a[0] = a[1];
                a[1] = temp;
                // Remove first 2 characters
                a.splice(0, 2);
                return a.join('');
            }
        "#;

        let result = engine.execute_cipher_function(script, "decipher", "abcdefgh");
        assert!(result.is_ok());
        // Original: "abcdefgh"
        // Reversed: "hgfedcba"
        // Swapped: "ghfedcba"
        // Spliced: "fedcba"
        assert_eq!(result.unwrap(), "fedcba");
    }

    #[test]
    fn test_n_parameter_transformation() {
        let engine = JavaScriptEngine::new().unwrap();

        let script = r#"
            function transformN(n) {
                // Simple N parameter transformation
                return 'yt_' + n.split('').reverse().join('');
            }
        "#;

        let result = engine.execute_n_transform_function(script, "transformN", "abc123");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "yt_321cba");
    }

    #[test]
    fn test_engine_test_function() {
        let engine = JavaScriptEngine::new().unwrap();
        let result = engine.test_engine();
        assert!(result.is_ok());
    }
}
