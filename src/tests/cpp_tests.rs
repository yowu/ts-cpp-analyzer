//! Unit tests for C++ symbol analysis

use crate::SymbolType;
use crate::tests::test_helpers::run_test_case;

// Default test code for this module
const DEFAULT_TEST_CODE: &str = r#"
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
"#;

#[test]
fn test_class_definition() {
    assert!(run_test_case("class Foo", 6, SymbolType::Class, DEFAULT_TEST_CODE));
}

#[test]
fn test_field_definition() {
    assert!(run_test_case("int value", 4, SymbolType::Field, DEFAULT_TEST_CODE));
}

#[test]
fn test_constructor_definition() {
    assert!(run_test_case("Foo(int value)", 0, SymbolType::Function, DEFAULT_TEST_CODE));
}

#[test]
fn test_function_definition() {
    assert!(run_test_case("test_function", 0, SymbolType::Function, DEFAULT_TEST_CODE));
}

#[test]
fn test_parameter_type() {
    assert!(run_test_case("Bar& param", 0, SymbolType::Type, DEFAULT_TEST_CODE));
}

#[test]
fn test_reference_parameter() {
    assert!(run_test_case("Bar& param", 6, SymbolType::Parameter, DEFAULT_TEST_CODE));
}

#[test]
fn test_pointer_parameter() {
    assert!(run_test_case("Foo* foo", 5, SymbolType::Parameter, DEFAULT_TEST_CODE));
}

#[test]
fn test_method_call() {
    assert!(run_test_case("foo->double()", 5, SymbolType::MethodCall, DEFAULT_TEST_CODE));
}

#[test]
fn test_function_call() {
    assert!(run_test_case("test_function_pointer(&foo);", 0, SymbolType::FunctionCall, DEFAULT_TEST_CODE));
}

#[test]
fn test_value_parameter() {
    let custom_code = r#"
        void set_x(int value) {
            x = value;
        }
    "#;
    assert!(run_test_case("int value", 5, SymbolType::Parameter, custom_code));
}