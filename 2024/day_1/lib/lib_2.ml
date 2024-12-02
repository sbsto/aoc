let count_occurrences value a =
  let rec count acc a =
    match a with
    | [] -> acc
    | x :: xs -> if x = value then count (acc + 1) xs else count acc xs
  in
  count 0 a

let score value a = value * count_occurrences value a

let score_similarity a b : int =
  let rec score_similarity acc a b =
    match (a, b) with
    | [], _ -> acc
    | x :: xs, ys ->
        let new_score = acc + score x ys in
        score_similarity new_score xs ys
  in
  score_similarity 0 a b
