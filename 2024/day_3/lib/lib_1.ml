let extract_mul_args str =
  let regexp = Str.regexp "mul(\\([0-9]+\\),\\([0-9]+\\))" in
  let rec loop acc pos =
    try
      let _ = Str.search_forward regexp str pos in
      let x = int_of_string (Str.matched_group 1 str) in
      let y = int_of_string (Str.matched_group 2 str) in
      loop ((x, y) :: acc) (Str.match_end ())
    with Not_found -> List.rev acc
  in
  loop [] 0

let pairs_of_string str =
  extract_mul_args str

let sum_pair_products pairs = 
  List.fold_left (fun acc (x, y) -> acc + x * y) 0 pairs


