let diff report =
  let rec diff report acc =
    match report with
    | [] | [ _ ] -> acc
    | x :: xs -> diff xs (List.append acc [ List.hd xs - x ])
  in

  diff report []

let is_increasing report =
  let diffs = diff report in
  let rec is_increasing diffs =
    match diffs with [] -> true | x :: xs -> x > 0 && is_increasing xs
  in
  is_increasing diffs

let is_decreasing report =
  let diffs = diff report in
  let rec is_decreasing diffs =
    match diffs with [] -> true | x :: xs -> x < 0 && is_decreasing xs
  in

  is_decreasing diffs

let has_outliers report =
  let diffs = diff report in
  let rec has_outliers diffs =
    match diffs with
    | [] -> false
    | x :: xs -> abs x > 3 || abs x < 1 || has_outliers xs
  in

  has_outliers diffs

let is_safe report =
  let has_outliers = has_outliers report in
  let is_increasing = is_increasing report in
  let is_decreasing = is_decreasing report in
  (not has_outliers) && (is_increasing || is_decreasing)

let count_safe_reports reports =
  let rec count_safe_reports acc reports =
    match reports with
    | [] -> acc
    | report :: reports ->
        if is_safe report then count_safe_reports (acc + 1) reports
        else count_safe_reports acc reports
  in
  count_safe_reports 0 reports
