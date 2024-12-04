let test_count_word_in_matrix () =
  let matrix = Files.read_and_parse "test/test_data.txt" in
  let sum_pair_products = Lib_1.count_word_in_matrix matrix in
  assert (sum_pair_products = 18);
  print_endline "test_count_word_in_matrix passed"

let () =
  test_count_word_in_matrix ();
