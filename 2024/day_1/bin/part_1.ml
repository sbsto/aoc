let () =
  let filename = "bin/data.txt" in
  let a, b = Files.read_and_parse filename in
  let result = Lib_1.resolve_lists a b in
  print_endline (string_of_int result)
