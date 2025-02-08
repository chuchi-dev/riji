use riji::Script;

#[test]
fn test_capture() {
	let mut script = Script::new("./tests/test_capture.rhai").unwrap();
	let output = script.execute_capture("run_test", vec![]);

	assert!(output.error.is_none());
	assert_eq!(output.stdout, "hello\n");
	assert_eq!(output.stderr, "unkown @ 3:2 > \"error\"\n");
}
