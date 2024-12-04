let () =
  let matrix = Files.read_and_parse "bin/data.txt" in
  let sum_pair_products = Lib_1.count_word_in_matrix matrix in
  print_endline (string_of_int sum_pair_products)
