mod analyzer;
mod query;
mod languages;

#[cfg(test)]
mod tests;

pub use analyzer::{SymbolAnalyzer, SymbolInfo};
pub use query::{QueryManager, QueryManagerBuilder, QueryPattern, SymbolType};
pub use languages::cpp::{get_cpp_patterns, CPP_QUERIES};
