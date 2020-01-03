open IntCode.IntCode
open System

let processWithInput arr noun verb =
    Array.set arr 1 noun
    Array.set arr 2 verb

    try
        execute64 arr []
        |> (fun c -> c.memory.[0L])
        |> Some
    with
        | _ -> None

[<EntryPoint>]
let main argv =
    let opts = loadFile "input_data.txt"

    let part1 = (processWithInput (Array.copy opts) 12L 2L)
    printfn "Part 1: %d" (Option.get part1)

    [ for i in 0L .. 9999L -> async { return (processWithInput (Array.copy opts) (i / 100L) (i % 100L)), i } ]
    |> Async.Parallel
    |> Async.RunSynchronously
    // [|0 .. 9999|]
    // |> Array.map (fun i -> (processWithInput (Array.copy opts) (i / 100) (i % 100)), i)
    |> Array.filter (fun (v, _) -> v = Some(19690720L))
    |> Array.head
    |> snd
    |> printfn "Part 2: Final pair is %A"
    0 // return an integer exit code
