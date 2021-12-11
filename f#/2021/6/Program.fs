// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System
open AOCHelpers.AOC

let step state = 
    match state with
    | [a;b;c;d;e;f;g;h;i] -> [b;c;d;e;f;g;h+a;i;a]
    | _ -> state

let run days =
    applySteps days step
    >> List.sum

let part1 = 
    run 80

let part2 = 
    run 256

[<EntryPoint>]
let main argv =
    let counts = readCommaSepIntList "real_data.txt"
                |> Seq.countBy id
                |> Map.ofSeq

    let input = [0..8]
                |> List.map (fun i -> if counts.ContainsKey i then int64 counts.[i] else 0L)

    printfn "Part 1: %d" (part1 input)
    printfn "Part 2: %d" (part2 input)

    0 // return an integer exit code