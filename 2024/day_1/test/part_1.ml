let test_resolve_lists () =
  let filename = "test/test_data.txt" in
  let a, b = Files.read_and_parse filename in
  let result = Lib_1.resolve_lists a b in
  assert (result = 11);
  print_endline "Resolve test passed!"

let () =
  print_endline "Running tests for part 1...";
  test_resolve_lists ();
  print_endline "All tests for part 1 passed."
