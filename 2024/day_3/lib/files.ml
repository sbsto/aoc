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
  let length = in_channel_length ic in
  really_input_string ic length
