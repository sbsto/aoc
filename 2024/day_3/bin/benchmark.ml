open Core
open Core_bench

let () =
  let file_str = Files.read_and_parse "bin/data.txt" in

  Command_unix.run
    (Bench.make_command
       [
         Bench.Test.create ~name:"score_similarity" (fun () ->
            let pairs = Lib_2.pairs_of_string file_str in
            ignore (Lib_2.sum_pair_products pairs));
       ])
