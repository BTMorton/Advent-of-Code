// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System
open AOCHelpers.AOC

let isLowPoint (map: Map<Point, int>) point =
    findHVNeighbours map point
    |> Seq.map snd
    |> Seq.forall ((<) map.[point])

let rec traverseBasin (map: Map<Point, int>) (basin: Set<Point>) point =
    let newNeighbours = findHVNeighbours map point
                        |> Seq.filter (snd >> (>) 9)
                        |> Seq.map fst
                        |> Seq.filter (basin.Contains >> not)
    
    if Seq.isEmpty newNeighbours
    then basin
    else
        let newBasin = (newNeighbours |> Set.ofSeq |> Set.union basin)
        newNeighbours
            |> Seq.fold (traverseBasin map) newBasin

let calculateBasinSize (map: Map<Point, int>) point =
    traverseBasin map (new Set<Point>([])) point
    |> Set.count

let part1 input = 
    input
    |> Map.filter (fun point _ -> isLowPoint input point)
    |> Map.toSeq
    |> Seq.sumBy (snd >> (+) 1)

let part2 input = 
    input
    |> Map.filter (fun point _ -> isLowPoint input point)
    |> Map.toSeq
    |> Seq.map (fst >> calculateBasinSize input)
    |> Seq.sortDescending
    |> Seq.take 3
    |> Seq.fold (*) 1

[<EntryPoint>]
let main argv =
    let input = readIntMap "real_data.txt"

    printfn "Part 1: %d" (part1 input)
    printfn "Part 2: %d" (part2 input)

    0 // return an integer exit code