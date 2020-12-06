// Learn more about F# at http://fsharp.org

open System

let rec hasExPair (inArr: int list) =
    inArr
        |> List.countBy id
        |> List.exists (fun (_, v) -> v = 2)

let rec hasPair inArr =
    match inArr with
    | a::b::_ when a = b -> true
    | _::remain -> hasPair remain
    | _ -> false

let rec isIncrementing inArr =
    let sortArr = List.sort inArr
    List.fold2 (fun res a b -> res && a = b) true sortArr inArr

let getDigits input =
    input.ToString()
        |> Seq.toList
        |> List.map (fun i -> int i - int '0')

[<EntryPoint>]
let main argv =
    let min = 272091
    let max = 815432
    printfn "From %d to %d" min max

    let increasing = [min..max] |> List.map getDigits
                            |> List.filter isIncrementing
    let part1Matches = increasing |> List.filter hasPair
                            |> List.length
    let part2Matches = increasing |> List.filter hasExPair
                            |> List.length

    printfn "Part 1: %d" part1Matches
    printfn "Part 2: %d" part2Matches

    0 // return an integer exit code
