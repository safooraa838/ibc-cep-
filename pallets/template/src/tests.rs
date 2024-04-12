use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn add_task_successfully() {
	new_test_ext().execute_with(|| {
		let task: Vec<u8> = "Testing".into();
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::add_task(RuntimeOrigin::signed(1), task));
	})
}
#[test]
fn add_duplicate_task_fail() {
	new_test_ext().execute_with(|| {
		let task: Vec<u8> = "Testing".into();
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::add_task(RuntimeOrigin::signed(1), task.clone()));

		assert_noop!(
			TemplateModule::add_task(RuntimeOrigin::signed(1), task),
			Error::<Test>::TaskAlreadyAdded
		);
	})
}

#[test]
fn mark_as_complete_successfully() {
	new_test_ext().execute_with(|| {
		let task: Vec<u8> = "Testing".into();
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::add_task(RuntimeOrigin::signed(1), task.clone()));

		assert_ok!(TemplateModule::mark_as_complete(RuntimeOrigin::signed(1), task));
	})
}

#[test]
fn mark_missing_task_as_complete_fail() {
	new_test_ext().execute_with(|| {
		let task1: Vec<u8> = "Testing".into();
		let task2: Vec<u8> = "Coding".into();
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::add_task(RuntimeOrigin::signed(1), task1));

		assert_noop!(
			TemplateModule::mark_as_complete(RuntimeOrigin::signed(1), task2),
			Error::<Test>::TaskNotFound
		);
	})
}

#[test]
fn assign_task_to_other_successfully() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;

		let task: Vec<u8> = "Testing".into();
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::assign_task_to_other(
			RuntimeOrigin::signed(2),
			TEST_ACCOUNT,
			task.clone()
		));
	})
}

#[test]
fn assign_same_task_to_other_fail() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;

		let task: Vec<u8> = "Testing".into();
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::assign_task_to_other(
			RuntimeOrigin::signed(2),
			TEST_ACCOUNT,
			task.clone()
		));

		assert_noop!(
			TemplateModule::assign_task_to_other(RuntimeOrigin::signed(2), TEST_ACCOUNT, task),
			Error::<Test>::TaskAlreadyAssigned
		);
	})
}

#[test]
fn mark_assigned_task_as_complete_successfully() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;

		let task: Vec<u8> = "Testing".into();
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::assign_task_to_other(
			RuntimeOrigin::signed(1),
			TEST_ACCOUNT,
			task.clone()
		));

		assert_ok!(TemplateModule::mark_assigned_task_as_complete(
			RuntimeOrigin::signed(TEST_ACCOUNT),
			task
		));
	})
}

#[test]
fn mark_missing_assigned_task_as_complete_fail() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;

		let task: Vec<u8> = "Testing".into();
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::assign_task_to_other(
			RuntimeOrigin::signed(1),
			TEST_ACCOUNT,
			task.clone()
		));

		assert_ok!(TemplateModule::mark_assigned_task_as_complete(
			RuntimeOrigin::signed(TEST_ACCOUNT),
			task.clone()
		));

		assert_noop!(
			TemplateModule::mark_assigned_task_as_complete(
				RuntimeOrigin::signed(TEST_ACCOUNT),
				task
			),
			Error::<Test>::TaskNotAssigned
		);
	})
}

#[test]
fn edit_task_successfully() {
	new_test_ext().execute_with(|| {
		let task1: Vec<u8> = "Testing".into();
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::add_task(RuntimeOrigin::signed(1), task1.clone()));

		let task2: Vec<u8> = "Coding".into();

		assert_ok!(TemplateModule::edit_task(RuntimeOrigin::signed(1), task1, task2));
	})
}

#[test]
fn edit_missing_task_fail() {
	new_test_ext().execute_with(|| {
		let task1: Vec<u8> = "Testing".into();
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::add_task(RuntimeOrigin::signed(1), task1.clone()));

		let task2: Vec<u8> = "Coding".into();
		let task3: Vec<u8> = "Code".into();

		assert_noop!(
			TemplateModule::edit_task(RuntimeOrigin::signed(1), task2, task3),
			Error::<Test>::TaskNotFound
		);
	})
}