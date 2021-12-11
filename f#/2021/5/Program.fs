// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System
open AOCHelpers.AOC

type Pipe = Point * Point

let parsePipe = 
    split "->"
    >> Seq.map (
        trim
        >> parsePoint
    )
    >> (fun seq -> Pipe (Seq.item 0 seq, Seq.item 1 seq))

let isHorizontalPipe (((x1, _), (x2, _)): Pipe) =
    x1 = x2

let isVerticalPipe (((_, y1), (_, y2)): Pipe) =
    y1 = y2

let generateRange a b =
    let step = b - a |> sign
    seq { a .. step .. b }

let getPipeValues pipe =
    match pipe with
    | ((x, y1), (_, y2)) when isHorizontalPipe pipe -> generateRange y1 y2 |> Seq.map (fun y -> (x, y))
    | ((x1, y), (x2, _)) when isVerticalPipe pipe -> generateRange x1 x2 |> Seq.map (fun x -> (x, y))
    | ((x1, y1), (x2, y2)) -> Seq.zip (generateRange x1 x2) (generateRange y1 y2)

let isHorVertPipe pipe =
    isHorizontalPipe pipe || isVerticalPipe pipe

let addPointToMap (map: Map<Point, int>) point =
    map.Add(point,
        if map.ContainsKey point
        then map.[point] + 1
        else 1
    )

let countPipeValues = 
    Seq.fold addPointToMap (new Map<Point, int>([]))

let countCrossingPipes =
    Seq.map getPipeValues
    >> Seq.collect id
    >> countPipeValues
    >> Map.filter (fun _ count -> count > 1)
    >> Seq.length

let part1 =
    Seq.filter isHorVertPipe
    >> countCrossingPipes

let part2 =
    countCrossingPipes

[<EntryPoint>]
let main argv =
    let lines = IO.File.ReadAllLines "real_data.txt"
                |> Seq.map parsePipe

    printfn "Part 1: %d" (part1 lines)
    printfn "Part 2: %d" (part2 lines)
    0 // return an integer exit code