// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System
open AOCHelpers.AOC

let part1 _ = 
    0

let part2 _ = 
    0

[<EntryPoint>]
let main argv =
    let input = IO.File.ReadAllLines "test_data.txt"
    // let input = readIntLines "test_data.txt"

    printfn "Part 1: %d" (part1 input)
    printfn "Part 2: %d" (part2 input)

    0 // return an integer exit code