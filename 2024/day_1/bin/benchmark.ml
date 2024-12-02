open Core
open Core_bench

let () =
  (* Define the benchmark inputs *)
  let filename = "bin/data.txt" in
  let a, b = Files.read_and_parse filename in

  (* Benchmark the function *)
  Command_unix.run (Bench.make_command [
    Bench.Test.create ~name:"score_similarity" (fun () ->
      ignore (Lib_2.score_similarity a b)
    );
  ])
