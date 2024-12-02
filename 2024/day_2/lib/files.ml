let absolute_path path =
  let cwd = Sys.getcwd () in
  let base_dir =
    match Str.split (Str.regexp "/_build") cwd with
    | prefix :: _ -> prefix
    | [] -> cwd
  in
  Filename.concat base_dir path

let read_and_parse filename =
  let adjusted_filename = absolute_path filename in
  let ic = open_in adjusted_filename in
  let rec parse_lines acc =
    try
      let line = input_line ic |> String.trim in
      let cols = Str.split (Str.regexp " ") line in
      let nums = List.map int_of_string cols in
      parse_lines (nums :: acc)
    with End_of_file ->
      close_in ic;
      List.rev acc
  in
  parse_lines []
