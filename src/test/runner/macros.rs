#[macro_export]
macro_rules! define_test_suite {
    (
        struct_name: $struct_name:ident,
        name: $name:expr,
        default_code: $code:expr,
        cases: [
            $(
                {
                    pattern: $pattern:expr,
                    offset: $offset:expr,
                    type: $type:expr,
                    description: $desc:expr
                    $(, code: $test_code:expr)?
                }
            ),* $(,)?
        ]
    ) => {
        pub struct $struct_name;

        impl crate::test::runner::TestSuite for $struct_name {
            fn name(&self) -> &'static str {
                $name
            }

            fn default_code(&self) -> String {
                $code.to_string()
            }

            fn test_cases(&self) -> Vec<crate::test::runner::TestCase> {
                vec![
                    $(
                        crate::test::runner::TestCase::new($pattern, $offset, $type)
                            .with_description($desc)
                            $(
                                .with_code($test_code)
                            )?
                    ),*
                ]
            }
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self {}
            }
        }
    };
}
