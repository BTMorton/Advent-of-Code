// Learn more about F# at http://fsharp.org

open System

let floorCounter char =
    match char with
    | '(' -> 1
    | ')' -> -1
    | _ -> 0

let floorsMap (map: Map<int, int>) (index, char) =
    map.Add(index, map.[index - 1] + (floorCounter char))

let floorCount floor char =
    floor + floorCounter char

[<EntryPoint>]
let main argv =
    let input = IO.File.ReadAllLines "input_data.txt"
                |> Seq.head

    let floor = input
                |> Seq.sumBy floorCounter
    printfn "Part 1 %d" floor

    let floors = input
                        |> Seq.map floorCounter
                        |> Seq.scan (+) 0
    printfn "floors %A" floors
    let basementIndex = floors
                        |> Seq.mapi (fun i f -> (i,f))
                        |> Seq.filter (fun (_,f) -> f = -1)
                        |> Seq.head
                        |> fst
    printfn "Part 2 %d" basementIndex

    0 // return an integer exit code
