extern crate scheduler;
mod common;
use scheduler::{Input, Output};
use std::path::Path;
#[cfg(test)]
use pretty_assertions::assert_eq;

fn run_test(directory: &str) -> (String,String) {
    let i = format!("./js-tests/{}/input.json",directory);
    let o = format!("./js-tests/{}/output.json",directory);
    let input_path = Path::new(&i[..]);
	let output_path = Path::new(&o[..]);
	let input: Input = common::get_input_from_json(input_path).unwrap();
	let desired_output: String = common::get_output_string_from_json(output_path).unwrap();
	let output: Vec<Output> = scheduler::run_scheduler(input);

    (serde_json::to_string_pretty(&output).unwrap(),desired_output)
}

#[test]
fn basic_test_works() {
    let (actual_output, desired_output) = run_test("basic-test");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn repetition_daily_works() {
    let (actual_output, desired_output) = run_test("repetition-daily");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn repetition_daily_bounds_works() {
    let (actual_output, desired_output) = run_test("repetition-daily-bounds");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn non_repetitive_bounds_multipledays_works() {
    let (actual_output, desired_output) = run_test("non-rep-bounds-multipledays");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn single_day_many_goals_works() {
    let (actual_output, desired_output) = run_test("singleday-manygoals");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn every_wednesday_works() {
    let (actual_output, desired_output) = run_test("every-wednesday");
    assert_eq!(actual_output, desired_output);
}

#[test]
fn realistic_schedule_works() {
    let (actual_output, desired_output) = run_test("realistic-schedule");
    assert_eq!(actual_output, desired_output);
}
