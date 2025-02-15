use languages::cpp::CPP_QUERIES;
use test::cases::cpp::CppTestSuite;
use test::runner::TestRunner;

mod languages;
mod test;
fn main() {
    let runner = TestRunner::new(CPP_QUERIES.clone());
    runner.run_suite(&CppTestSuite::default());
}
