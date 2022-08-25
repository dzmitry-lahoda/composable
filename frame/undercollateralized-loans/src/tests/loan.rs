use super::{create_test_loan, create_test_market, parse_timestamp, prelude::*};
use crate::currency::BTC;
use composable_traits::undercollateralized_loans::LoanInput;

#[test]
fn can_create_loan() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let manager = *ALICE;
		let origin = Origin::signed(manager);
		let market_info = create_test_market();
		let market_account_id = market_info.config().account_id().clone();
		let mut payment_schedule = BTreeMap::new();
		payment_schedule.insert(parse_timestamp("24-08-1991"), 100);
		let loan_input = LoanInput {
			market_account_id,
			borrower_account_id: *BOB,
			principal: 1000,
			collateral: 5,
			payment_schedule,
			activation_date: parse_timestamp("24-08-1991"),
			delayed_payment_treatment: None,
		};
		assert_ok!(pallet_undercollateralized_loans::Pallet::<Runtime>::create_loan(
			origin, loan_input
		));
	})
}

#[test]
fn can_execute_loan_contract() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		let borrower = *BOB;
		// Borrwer should have some collateral to deposit.
		Tokens::mint_into(BTC::ID, &borrower, BTC::units(10)).unwrap();
		let origin = Origin::signed(borrower);
		let loan_config = create_test_loan();
		let loan_account_id = *loan_config.account_id();
		assert_ok!(pallet_undercollateralized_loans::Pallet::<Runtime>::borrow(
			origin,
			loan_account_id,
			true
		));
	});
}