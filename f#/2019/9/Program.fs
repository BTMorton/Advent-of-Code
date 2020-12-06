// Learn more about F# at http://fsharp.org

open System
open IntCode.IntCode

[<EntryPoint>]
let main argv =
    let input = loadFile "data.txt"

    printfn "Part 1: %A" ((execute64 input [1L]).output |> List.rev)
    printfn "Part 2: %A" ((execute64 input [2L]).output |> List.rev)

    printfn "Hello World from F#!"
    0 // return an integer exit code
