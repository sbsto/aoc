let resolve_lists a b =
  let sorted_a = List.sort compare a in
  let sorted_b = List.sort compare b in
  if List.length sorted_a != List.length sorted_b then -1
  else
    let rec sum_diffs acc a b =
      match (a, b) with
      | [], [] -> acc
      | x :: xs, y :: ys ->
          if x = y then sum_diffs acc xs ys
          else sum_diffs (acc + abs (x - y)) xs ys
      | _ -> failwith "Length mismatch"
    in
    sum_diffs 0 sorted_a sorted_b
