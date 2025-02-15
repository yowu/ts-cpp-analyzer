pub mod macros;

use std::sync::Arc;

use symbol_analyzer::{QueryManager, SymbolAnalyzer, SymbolType};

#[derive(Debug, Clone)]
pub struct TestCase {
    code: Option<String>, // None means use default_code
    pattern: String,
    offset: usize,
    expected_type: SymbolType,
    description: Option<String>, // Optional test case description
}

impl TestCase {
    pub fn new(pattern: impl Into<String>, offset: usize, expected_type: SymbolType) -> Self {
        Self {
            code: None,
            pattern: pattern.into(),
            offset,
            expected_type,
            description: None,
        }
    }

    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

pub struct TestResult {
    pub case: TestCase,
    pub success: bool,
    pub found_type: Option<SymbolType>,
    pub error: Option<String>,
}

pub trait TestSuite {
    fn name(&self) -> &'static str;
    fn default_code(&self) -> String;
    fn test_cases(&self) -> Vec<TestCase>;
}

pub struct TestRunner {
    queries: Arc<QueryManager>,
}

impl TestRunner {
    pub fn new(queries: Arc<QueryManager>) -> Self {
        Self { queries }
    }

    pub fn run_suite(&self, suite: &dyn TestSuite) {
        println!("\n=== Test suite: {} ===", suite.name());
        let mut results = Vec::new();
        for case in suite.test_cases() {
            let result = self.run_test_case(&case, Some(suite.default_code()));
            TestRunner::print_result(&result);
            results.push(result);
        }
        TestRunner::print_summary(&results);
    }

    fn run_test_case(&self, case: &TestCase, default_code: Option<String>) -> TestResult {
        let code = case
            .code
            .as_ref()
            .or(default_code.as_ref())
            .expect("No code provided for test case");

        let analyzer = match SymbolAnalyzer::new(self.queries.clone(), code) {
            Ok(analyzer) => analyzer,
            Err(e) => {
                return TestResult {
                    case: case.clone(),
                    success: false,
                    found_type: None,
                    error: Some(format!("Failed to create analyzer: {:?}", e)),
                }
            }
        };

        let position = match analyzer.get_position(&case.pattern, case.offset) {
            Some(pos) => pos,
            None => {
                return TestResult {
                    case: case.clone(),
                    success: false,
                    found_type: None,
                    error: Some(format!("Pattern '{}' not found", case.pattern)),
                }
            }
        };

        let symbol = match analyzer.query_symbol(position) {
            Some(symbol) => symbol,
            None => {
                return TestResult {
                    case: case.clone(),
                    success: false,
                    found_type: None,
                    error: Some("No symbol found at position".to_string()),
                }
            }
        };

        TestResult {
            case: case.clone(),
            success: symbol.kind == case.expected_type,
            found_type: Some(symbol.kind),
            error: None,
        }
    }

    fn print_result(result: &TestResult) {
        println!("\n--- Test Case ---");
        if let Some(desc) = &result.case.description {
            println!("Description: {}", desc);
        }
        println!("Pattern: {}", result.case.pattern);
        println!("Expected type: {:?}", result.case.expected_type);

        if result.success {
            println!("✅ Test passed");
        } else {
            println!("❌ Test failed");
            if let Some(found_type) = &result.found_type {
                println!("Found type: {:?}", found_type);
            }
            if let Some(error) = &result.error {
                println!("Error: {}", error);
            }
        }
    }

    pub fn print_summary(results: &[TestResult]) {
        println!("\n=== Test Summary ===");
        let total = results.len();
        let passed = results.iter().filter(|r| r.success).count();
        println!("Total tests: {}", total);
        println!("✅ Passed: {}", passed);
        let failed = total - passed;
        if failed > 0 {
            println!("❌ Failed: {}", failed);
        }
    }
}
