use symbol_analyzer::{QueryManager, QueryManagerBuilder, QueryPattern};

use lazy_static::lazy_static;
use std::sync::Arc;

pub fn get_cpp_patterns() -> Vec<QueryPattern> {
    vec![
        // Function patterns
        QueryPattern::Function(
            r#"(function_definition
              declarator: (function_declarator
                  declarator: (identifier) @function_name))"#,
        ),
        // Class patterns
        QueryPattern::Class(
            r#"(class_specifier
              name: (type_identifier) @class_name)"#,
        ),
        QueryPattern::Variable(
            r#"(declaration
              declarator: (init_declarator
                  declarator: (identifier) @variable_name))"#,
        ),
        QueryPattern::FunctionCall(
            r#"(call_expression
              function: (identifier) @function_call)"#,
        ),
        QueryPattern::MethodCall(
            r#"(call_expression
              function: (field_expression
                  field: (field_identifier) @function_call))"#,
        ),
        QueryPattern::Type(
            r#"[
              (type_identifier) @type
              (primitive_type) @type
          ]"#,
        ),
        QueryPattern::Parameter(
            r#"[
              (parameter_declaration
                  declarator: (identifier) @param)
              (parameter_declaration
                  declarator: (pointer_declarator
                      declarator: (identifier) @param))
          ]"#,
        ),
        QueryPattern::Parameter(
            r#"(parameter_declaration
              declarator: (reference_declarator (identifier) @param))"#,
        ),
        QueryPattern::Field(
            r#"(field_declaration
              declarator: (field_identifier) @field_name)"#,
        ),
        QueryPattern::FieldAccess(
            r#"[
              (field_expression
                  field: (field_identifier) @field)
              (field_identifier) @field
          ]"#,
        ),
    ]
}

lazy_static! {
    pub static ref CPP_QUERIES: Arc<QueryManager> = Arc::new(
        QueryManagerBuilder::new("cpp")
            .with_language(tree_sitter_cpp::LANGUAGE.into())
            .with_queries(get_cpp_patterns())
            .build()
            .expect("Failed to build C++ language config")
    );
}
