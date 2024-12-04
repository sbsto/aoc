type direction = NE | SE | SW | NW
type coords = int * int
type value = { coords : coords; char : string; direction : direction }

let show_direction direction =
  match direction with NE -> "NE" | SE -> "SE" | SW -> "SW" | NW -> "NW"

let show_value value =
  let x, y = value.coords in
  Printf.sprintf "(%d, %d) %s %s" x y value.char
    (show_direction value.direction)

let get_neighbour matrix coords direction =
  let length = List.length matrix in
  let height = List.length (List.hd matrix) in
  let x, y = coords in
  let neighbour_coords =
    match direction with
    | NE -> if x < length - 1 && y > 0 then Some (x + 1, y - 1) else None
    | SE ->
        if x < length - 1 && y < height - 1 then Some (x + 1, y + 1) else None
    | SW -> if x > 0 && y < height - 1 then Some (x - 1, y + 1) else None
    | NW -> if x > 0 && y > 0 then Some (x - 1, y - 1) else None
  in
  match neighbour_coords with
  | Some coords ->
      let x, y = coords in
      Some { coords; char = List.nth (List.nth matrix y) x; direction }
  | None -> None

let get_neighbours matrix coords =
  [ NE; SE; SW; NW ]
  |> List.map (fun dir -> get_neighbour matrix coords dir)
  |> List.filter_map (fun c -> c)

let check_neighbours neighbours =
  match neighbours with
  | [ top_right; bottom_right; bottom_left; top_left ] ->
      ((top_right.char = "S" && bottom_left.char = "M")
      || (top_right.char = "M" && bottom_left.char = "S"))
      && ((top_left.char = "S" && bottom_right.char = "M")
         || (top_left.char = "M" && bottom_right.char = "S"))
  | _ -> false

let check_char matrix (x, y) =
  let c = List.nth (List.nth matrix y) x in
  c = "A" && check_neighbours (get_neighbours matrix (x, y))

let count_cross_in_matrix matrix =
  matrix
  |> List.mapi (fun y row ->
         List.mapi (fun x _ -> check_char matrix (x, y)) row)
  |> List.flatten
  |> List.fold_left (fun acc x -> if x then acc + 1 else acc) 0
