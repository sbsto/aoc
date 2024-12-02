module IntMap = Hashtbl.Make(struct
  type t = int
  let equal = Int.equal
  let hash = Hashtbl.hash
end)

let count_occurrences value a =
  let rec count acc a =
    match a with
    | [] -> acc
    | x :: xs -> if x = value then count (acc + 1) xs else count acc xs
  in
  count 0 a

let precompute_scores b =
  let table = IntMap.create (List.length b) in
  List.iter (fun value ->
    if not (IntMap.mem table value) then
      let score = value * count_occurrences value b in
      IntMap.add table value score
  ) b;
  table

let score_similarity a b : int =
  let table = precompute_scores b in
  let rec score_similarity acc a =
    match a with
    | [] -> acc
    | x :: xs ->
        let new_score = acc + (if IntMap.mem table x then IntMap.find table x else 0) in
        score_similarity new_score xs
  in
  score_similarity 0 a
