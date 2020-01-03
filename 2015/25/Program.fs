// Learn more about F# at http://fsharp.org

open System

let getNextCode code =
    (code * 252533L) % 33554393L

let getEntryNumber x y =
    let rec sum n =
        match n with
        | 0 -> 0
        | n -> sum (n - 1) + n

    (sum (x + y - 2)) + (x - 1)

[<EntryPoint>]
let main argv =
    let entry = getEntryNumber 3019 3010
    let code = List.replicate entry getNextCode
                  |> List.fold (fun code fn -> fn code) 20151125L

    printfn "Part 1: %d" (code)
    0 // return an integer exit code
