// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System
open AOCHelpers.AOC

type EnergyMap = Map<Point, int>

let printMap (map: EnergyMap) =
    let maxX = map |> Map.toSeq |> Seq.map fst |> Seq.map fst |> Seq.max
    let maxY = map |> Map.toSeq |> Seq.map fst |> Seq.map snd |> Seq.max

    for y in { 0 .. maxY } do
        for x in { 0 .. maxX } do
            printf "%d" map.[(x, y)]
        printfn ""
    printfn ""

let increaseEnergy =
    Map.fold (fun (map: EnergyMap) key value -> map.Add(key, value + 1)) (new EnergyMap([]))

let rec handleFlash (input: EnergyMap) =
    let flashPoints = input |> Map.filter (fun _ value -> value > 9) |> Map.toSeq |> Seq.map fst

    if Seq.isEmpty flashPoints
    then input
    else
        flashPoints
        |> Seq.fold (fun map point ->
            opNeighbours (fun _ value -> if value = 0 then 0 else value + 1) map point
            |> (fun map -> map.Add(point, 0))
        ) input
        |> handleFlash

let step (inputMap: EnergyMap, flashCount) =
    let outMap =
        inputMap
        |> increaseEnergy
        |> handleFlash
    // printMap outMap
    (outMap, flashCount + (outMap |> Map.filter (fun _ value -> value = 0) |> Map.count))

let part1 input = 
    applySteps 100 step (input, 0)
    |> snd

let rec findAllFlash input stepCount =
    let (result, count) = step (input, 0)
    
    if result |> Seq.length |> (=) count
    then stepCount + 1
    else findAllFlash result (stepCount + 1)

let part2 input = 
    findAllFlash input 0

[<EntryPoint>]
let main argv =
    let input = readIntMap "real_data.txt"
    // printMap input

    printfn "Part 1: %d" (part1 input)
    printfn "Part 2: %d" (part2 input)

    0 // return an integer exit code