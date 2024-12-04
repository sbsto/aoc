open Core
open Core_bench

let () =
  let matrix = Files.read_and_parse "bin/data.txt" in

  Command_unix.run
    (Bench.make_command
       [
         Bench.Test.create ~name:"score_similarity" (fun () ->
             ignore (Lib_2.count_cross_in_matrix matrix));
       ])
