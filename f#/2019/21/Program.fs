// Learn more about F# at http://fsharp.org

open System
open IntCode.IntCode

let tryPart part opts input =
    let input = input |> List.collect (fun l -> (l + "\n") |> Seq.toList |> List.map int64)
    let result = execute64 opts input
    let output = result.output
    let num = List.head output
    if num > 256L
    then printfn "Part %d: %d" part num
    else
        printfn "Error during Part %d:\n%s" part (output |> List.tail |> List.rev |> List.map char |> (fun l -> String.Join("", l)))

[<EntryPoint>]
let main argv =
    let opts = loadFile "data.txt"

    tryPart 1 opts [
            "NOT A J";
            "NOT C T";
            "OR T J";
            "AND D J";
            "WALK";
        ]

    tryPart 2 opts [
            "NOT C J";
            "NOT B T";
            "OR T J";
            "AND H J";
            "AND D J";
            "NOT A T";
            "OR T J";
            "RUN";
        ]
    0 // return an integer exit code
