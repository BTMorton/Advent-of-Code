// Learn more about F# at http://fsharp.org

open System
open System.IO

let getDimensions (str: string) =
    str.Split [|'x'|]
    |> Array.map int

let getSides (dims: int array) =
    [
        dims.[0] * dims.[1];
        dims.[1] * dims.[2];
        dims.[2] * dims.[0];
    ]

let getRequiredPaper (sides: int list) =
    sides
    |> List.sum
    |> (*) 2
    |> (+) (List.min sides)

[<EntryPoint>]
let main argv =
    let input = IO.File.ReadAllLines "input_data.txt"
                |> Seq.map getDimensions
    let paper = input |> Seq.map getSides
                      |> Seq.sumBy getRequiredPaper

    printfn "Part 1 %d" paper

    let wrap = input
                    |> Seq.map (Array.sort
                                >> Array.rev
                                >> Array.tail)
                    |> Seq.sumBy (Array.sum >> (*) 2)
    let ribbons = input |> Seq.sumBy (Array.fold (*) 1)
    printfn "Part 2 %d" (wrap + ribbons)
    0 // return an integer exit code
