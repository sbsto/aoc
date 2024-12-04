let () =
  let file_str = Files.read_and_parse "bin/data.txt" in
  let pairs = Lib_1.pairs_of_string file_str in
  print_endline (string_of_int (Lib_1.sum_pair_products pairs))
