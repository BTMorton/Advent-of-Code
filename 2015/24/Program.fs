// Learn more about F# at http://fsharp.org

open System

let groupProd = List.fold (*) 1L

let rec combinations list choose =
    match choose, list with
    | 0,_ -> [[]]
    | _, [] -> []
    | k, next::rem ->
        combinations rem (k - 1)
        |> List.map ((@) [next])
        |> (@) (combinations rem k)

let findMinGroup values target =
    let rec tryCombinations choose =
        let results =
            combinations values choose
            |> List.filter (List.sum >> (=) target)

        if results.Length = 0
        then tryCombinations (choose + 1)
        else
            results
            |> List.minBy groupProd

    tryCombinations 1

[<EntryPoint>]
let main argv =
    let values =
        IO.File.ReadAllLines "data.txt"
        |> Array.map int64
        |> Array.toList
        |> List.rev

    let total = List.sum values

    let part1Total = total / 3L
    let minGroup = findMinGroup values part1Total
    printfn "Part 1: %A %d" minGroup (groupProd minGroup)

    let part2Total = total / 4L
    let minGroup = findMinGroup values part2Total
    printfn "Part 2: %A %d" minGroup (groupProd minGroup)
    0 // return an integer exit code
