let () =
  let filename = "bin/data.txt" in
  let reports = Files.read_and_parse filename in
  let count = Lib_2.count_safe_reports reports in
  print_endline (string_of_int count)
