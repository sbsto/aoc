let test_sum_pair_products () =
  let file_str = Files.read_and_parse "test/test_data_2.txt" in
  let pairs = Lib_2.pairs_of_string file_str in
  let total_sum = Lib_2.sum_pair_products pairs in
  assert (total_sum = 48);
  print_endline "test_sum_pair_products 2 passed"

let () =
  test_sum_pair_products ();
