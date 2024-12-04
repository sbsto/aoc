type direction = N | NE | E | SE | S | SW | W | NW
type coords = int * int
type value = { coords : coords; char : string; direction : direction }

let show_direction direction =
  match direction with
  | N -> "N"
  | NE -> "NE"
  | E -> "E"
  | SE -> "SE"
  | S -> "S"
  | SW -> "SW"
  | W -> "W"
  | NW -> "NW"

let show_value value =
  let x, y = value.coords in
  Printf.sprintf "(%d, %d) %s %s" x y value.char
    (show_direction value.direction)

let get_neighbour matrix coords direction  =
  let length = List.length matrix in
  let height = List.length (List.hd matrix) in
  let x, y = coords in
  let neighbour_coords =
    match direction with
    | N -> if y > 0 then Some (x, y - 1) else None
    | NE -> if x < length - 1 && y > 0 then Some (x + 1, y - 1) else None
    | E -> if x < length - 1 then Some (x + 1, y) else None
    | SE ->
        if x < length - 1 && y < height - 1 then Some (x + 1, y + 1) else None
    | S -> if y < height - 1 then Some (x, y + 1) else None
    | SW -> if x > 0 && y < height - 1 then Some (x - 1, y + 1) else None
    | W -> if x > 0 then Some (x - 1, y) else None
    | NW -> if x > 0 && y > 0 then Some (x - 1, y - 1) else None
  in
  match neighbour_coords with
  | Some coords ->
      let x, y = coords in
      Some { coords; char = List.nth (List.nth matrix y) x; direction }
  | None -> None

let get_neighbours matrix coords =
  [ N; NE; E; SE; S; SW; W; NW ]
  |> List.map (fun dir -> get_neighbour matrix coords dir)
  |> List.filter_map (fun c -> c)

let step matrix cells char =
  cells
  |> List.map (fun v -> get_neighbour matrix v.coords v.direction)
  |> List.filter_map (fun x -> x)
  |> List.filter (fun c -> c.char = char)

let check_char matrix (x, y) =
  let c = List.nth (List.nth matrix y) x in
  if c = "X" then
    let ms =
      get_neighbours matrix (x, y) |> List.filter (fun c -> c.char = "M")
    in
    let as_ = step matrix ms "A" in
    let ss = step matrix as_ "S" in
    List.length ss
  else 0

let count_word_in_matrix matrix =
  matrix
  |> List.mapi (fun y row ->
         List.mapi (fun x _ -> check_char matrix (x, y)) row)
  |> List.flatten |> List.fold_left ( + ) 0
