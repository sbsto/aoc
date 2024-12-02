let test_count_occurrences () =
  let filename = "test/test_data.txt" in
  let a, b = Files.read_and_parse filename in
  let result = Lib_2.count_occurrences 3 a in
  assert (result = 3);
  let result = Lib_2.count_occurrences 4 b in
  assert (result = 1);
  let result = Lib_2.count_occurrences 0 a in
  assert (result = 0);
  print_endline "Count test passed!"

let test_score_similarity () =
  let filename = "test/test_data.txt" in
  let a, b = Files.read_and_parse filename in
  let result = Lib_2.score_similarity a b in
  assert (result = 31);
  print_endline "Similarity test passed!"

let () =
  print_endline "Running tests for part 2...";
  test_count_occurrences ();
  test_score_similarity ();
  print_endline "All tests for part 2 passed."
