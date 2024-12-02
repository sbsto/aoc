let () =
  let filename = "bin/data.txt" in
  let a, b = Files.read_and_parse filename in
  let result = Lib_2.score_similarity a b in
  print_endline (string_of_int result)
