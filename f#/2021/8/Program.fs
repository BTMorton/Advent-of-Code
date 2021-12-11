// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System
open AOCHelpers.AOC

let calculateOutput (input, output) =
    let resolved1 = input |> Seq.find (Seq.length >> (=) 2) |> Set.ofSeq
    let resolved4 = input |> Seq.find (Seq.length >> (=) 4) |> Set.ofSeq

    output
    |> Seq.map (fun s ->
        match Seq.length s with
        | 2 -> '1'
        | 3 -> '7'
        | 4 -> '4'
        | 7 -> '8'
        | 5 when (s |> Set.ofSeq |> Set.intersect resolved1 |> Seq.length |> (=) 2) -> '3'
        | 5 when (s |> Set.ofSeq |> Set.intersect resolved4 |> Seq.length |> (=) 3) -> '5'
        | 5 -> '2'
        | 6 when (s |> Set.ofSeq |> Set.intersect resolved1 |> Seq.length |> (=) 1) -> '6'
        | 6 when (s |> Set.ofSeq |> Set.intersect resolved4 |> Seq.length |> (=) 4) -> '9'
        | 6 -> '0'
        | _ -> ' '
    )
    |> String.Concat
    |> int

let isSimpleSegment s =
    match Seq.length s with
    | 2 | 3 | 4 | 7 -> true
    | _ -> false

let part1 = 
    Seq.collect snd
    >> Seq.filter isSimpleSegment
    >> Seq.length

let part2 = 
    Seq.map calculateOutput
    >> Seq.sum

[<EntryPoint>]
let main argv =
    let lines = IO.File.ReadAllLines "real_data.txt"
                |> Seq.map (
                    split "|"
                    >> Array.map (trim >> split " " >> Seq.ofArray)
                    >> (fun seq -> (Seq.item 0 seq, Seq.item 1 seq))
                )

    printfn "Part 1: %d" (part1 lines)
    printfn "Part 2: %d" (part2 lines)

    0 // return an integer exit code