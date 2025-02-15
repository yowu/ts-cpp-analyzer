use symbol_analyzer::SymbolType;

use crate::define_test_suite;

define_test_suite! {
    struct_name: CppTestSuite,
    name: "cpp",
    default_code: r#"
      class Foo {
          int value;
      public:
          Foo(int value) {
              this->value = value;
          }
          void double() {
              this.value *= 2;
          }
      };

      void test_function(Bar& param) {
          param.value = 42;
      }

      void test_function_pointer(Foo* foo) {
          foo->double();
      }

      void tset_function_call(int value) {
          Foo foo(value);
          test_function_pointer(&foo);
      }
    "#,
    cases: [
      {
        pattern: "class Foo",
        offset: 6,
        type: SymbolType::Class,
        description: "Class definition"
      },
      {
        pattern: "int value",
        offset: 4,
        type: SymbolType::Field,
        description: "Field definition"
      },
      {
        pattern: "Foo(int value)",
        offset: 0,
        type: SymbolType::Function,
        description: "Constructor definition"
      },
      {
        pattern: "test_function",
        offset: 0,
        type: SymbolType::Function,
        description: "Function definition"
      },
      {
        pattern: "Bar& param",
        offset: 0,
        type: SymbolType::Type,
        description: "Parmater type"
      },
      {
        pattern: "Bar& param",
        offset: 6,
        type: SymbolType::Parameter,
        description: "Reference parameter"
      },
      {
        pattern: "Foo* foo",
        offset: 5,
        type: SymbolType::Parameter,
        description: "Pointer parameter"
      },
      {
        pattern: "foo->double()",
        offset: 5,
        type: SymbolType::MethodCall,
        description: "Method call"
      },
      {
        pattern: "test_function_pointer(&foo);",
        offset: 0,
        type: SymbolType::FunctionCall,
        description: "Function call"
      },
      {
        pattern: "int value",
        offset: 5,
        type: SymbolType::Parameter,
        description: "Value parameter",
        code: r#"
            void set_x(int value) {
                x = value;
            }
        "#
      }
    ]
}
