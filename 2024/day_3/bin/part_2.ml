let () =
  let file_str = Files.read_and_parse "bin/data.txt" in
  let pairs = Lib_2.pairs_of_string file_str in
  let total_sum = Lib_2.sum_pair_products pairs in
  print_endline (string_of_int total_sum)
