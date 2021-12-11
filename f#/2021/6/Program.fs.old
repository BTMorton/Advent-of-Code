// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System
open AOCHelpers.AOC

let step = 
    Seq.collect (fun (timer, pop) ->
        match timer with
        | 0L -> [ (6L, pop); (8L, pop) ]
        | _ -> [ ((timer - 1L), pop) ]
    )
    >> Seq.groupBy fst
    >> Seq.map (fun (days, l) -> (days, Seq.fold (fun pop (_, p) -> pop + p) 0L l))

let run days =
    applySteps days step
    >> Seq.map snd
    >> Seq.fold (+) 0L

let part1 = 
    run 80

let part2 = 
    run 256

[<EntryPoint>]
let main argv =
    let input = readCommaSepIntList "real_data.txt"
                |> Seq.countBy id
                |> Seq.map (fun (days, count) -> (int64 days, int64 count))

    printfn "Part 1: %d" (part1 input)
    printfn "Part 2: %d" (part2 input)

    0 // return an integer exit code