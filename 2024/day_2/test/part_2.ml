let test_count_safe_reports () =
  let filename = "test/test_data.txt" in
  let reports = Files.read_and_parse filename in
  let result = Lib_2.count_safe_reports reports in
  assert (result = 4);
  print_endline "Resolve test passed!"

let () =
  print_endline "Running tests for part 2...";
  test_count_safe_reports ();
  print_endline "All tests for part 2 passed."
