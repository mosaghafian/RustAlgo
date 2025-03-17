
mod Linear_Algebra;
use Linear_Algebra::linear_algebra;
use Linear_Algebra::linear_algebra_test;

fn main() {
    linear_algebra::create_matrix();
    linear_algebra_test::test_create_matrix();
}
