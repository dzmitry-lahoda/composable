mod common_test_functions; // TODO(benluelo): Remove
mod dual_asset_constant_product_tests;
mod dual_asset_constant_product_tests_new;
mod pablo_tests;

tests! { mod pablo<crate::mock::Test> }
