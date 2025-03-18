//! Helper functions for testing symbol analysis

use crate::{SymbolAnalyzer, SymbolType, CPP_QUERIES};


// Helper function to run a single test case
pub fn run_test_case(pattern: &str, offset: usize, expected_type: SymbolType, code: &str) -> bool {
    let queries = CPP_QUERIES.clone();
    
    let analyzer = match SymbolAnalyzer::new(queries, code) {
        Ok(analyzer) => analyzer,
        Err(_) => return false,
    };
    
    let position = match analyzer.get_position(pattern, offset) {
        Some(pos) => pos,
        None => return false,
    };
    
    let symbol = match analyzer.query_symbol(position) {
        Some(symbol) => symbol,
        None => return false,
    };
    
    symbol.kind == expected_type
}