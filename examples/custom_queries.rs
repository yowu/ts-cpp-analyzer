use std::sync::Arc;
use symbol_analyzer::{QueryManagerBuilder, QueryPattern, SymbolAnalyzer};

fn main() {
    // Create a C++ query manager with custom queries
    let cpp_queries = Arc::new(
        QueryManagerBuilder::new("cpp")
            .with_language(tree_sitter_cpp::LANGUAGE.into())
            .with_queries(get_custom_cpp_patterns())
            .build()
            .expect("Failed to build C++ language config")
    );

    // Sample C++ code with templates to analyze
    let code = r#"
        template<typename T>
        class Container {
            T value;
        public:
            Container(T val) : value(val) {}
            T getValue() { return value; }
        };

        template<typename T>
        void process(Container<T>& container) {
            T value = container.getValue();
            // Process value
        }

        int main() {
            Container<int> intContainer(42);
            process(intContainer);
            return 0;
        }
    "#;

    // Create analyzer with the code
    let analyzer = SymbolAnalyzer::new(cpp_queries, code)
        .expect("Failed to create analyzer");

    // Print the entire syntax tree for debugging
    println!("Syntax Tree:\n");
    analyzer.print_tree();

    // Find and analyze template class
    if let Some(position) = analyzer.get_position("template<typename T>", 0) {
        if let Some(symbol) = analyzer.query_symbol(position) {
            println!("\nFound template: {:?}", symbol.kind);
            println!("Template text: {}", symbol.text);
        }
    }

    // Find and analyze template function
    if let Some(position) = analyzer.get_position("process(Container<T>", 0) {
        if let Some(symbol) = analyzer.query_symbol(position) {
            println!("\nFound template function: {:?}", symbol.kind);
            println!("Function text: {}", symbol.text);
        }
    }
}

// Helper function to get custom C++ patterns including templates
fn get_custom_cpp_patterns() -> Vec<QueryPattern> {
    vec![
        // Template-specific patterns
        QueryPattern::Type(
            r#"(template_declaration) @template"#,
        ),
        QueryPattern::Type(
            r#"(template_type
              name: (type_identifier) @template_type)"#,
        ),
        QueryPattern::FunctionCall(
            r#"(call_expression
              function: (identifier) @template_function_call)"#,
        ),
    ]
}