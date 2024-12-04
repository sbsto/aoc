let test_sum_pair_products () =
  let file_str = Files.read_and_parse "test/test_data.txt" in
  let pairs = Lib_1.pairs_of_string file_str in
  assert (Lib_1.sum_pair_products pairs = 161);
  print_endline "test_sum_pair_products passed"

let () =
  test_sum_pair_products ();
