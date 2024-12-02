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
  let rec parse_lines acc1 acc2 =
    try
      let line = input_line ic |> String.trim in
      let cols = Str.split (Str.regexp "[ \t]+") line in
      match cols with
      | [ col1; col2 ] ->
          let num1 = int_of_string col1 in
          let num2 = int_of_string col2 in
          parse_lines (num1 :: acc1) (num2 :: acc2)
      | _ -> failwith "Malformed line"
    with End_of_file ->
      close_in ic;
      (List.rev acc1, List.rev acc2)
  in
  parse_lines [] []
