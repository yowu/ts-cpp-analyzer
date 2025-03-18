use symbol_analyzer::{SymbolAnalyzer, CPP_QUERIES};

fn main() {
    // Use the pre-defined CPP_QUERIES from the library
    let cpp_queries = CPP_QUERIES.clone();

    // Sample C++ code to analyze
    let code = r#"
        class MyClass {
            int value;
        public:
            MyClass(int val) : value(val) {}
            void setValue(int val) { value = val; }
            int getValue() { return value; }
        };

        void testFunction(MyClass& obj) {
            obj.setValue(42);
            int x = obj.getValue();
        }
    "#;

    // Create analyzer with the code
    let analyzer = SymbolAnalyzer::new(cpp_queries, code)
        .expect("Failed to create analyzer");

    // Find the position of a method call
    if let Some(position) = analyzer.get_position("obj.setValue", 4) {
        // Query the symbol at that position
        if let Some(symbol) = analyzer.query_symbol(position) {
            println!("Found symbol: {:?}", symbol.kind);
            println!("Symbol text: {}", symbol.text);
            println!("Symbol range: {} - {}", symbol.start_byte, symbol.end_byte);
        }
    }
}