// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System
open AOCHelpers.AOC

let rec findPathCosts (map: Map<Point, int>) target (tempMap: Map<Point, int>) (costMap: Map<Point, int>) pos =
    let currentTotal = tempMap.[pos]
    let newCostMap = costMap.Add(pos, currentTotal)
    if pos = target
    then currentTotal
    else 
        let newTempMap = findHVNeighbours map pos
                        |> Seq.toList
                        |> List.filter (fst >> costMap.ContainsKey >> not)
                        |> List.map (
                            fun (nextPos, nextVal) -> 
                                let nextTotal = currentTotal + nextVal
                                match tempMap.TryFind(nextPos) with
                                | Some(v) when v < nextTotal -> (nextPos, v)
                                | _ -> (nextPos, nextTotal)
                            )
                        |> List.fold (fun (state: Map<Point, int>) (nextPos, nextVal) -> state.Add(nextPos, nextVal)) (tempMap.Remove pos)

        newTempMap
        |> Map.toList
        |> List.minBy snd
        |> fst
        |> findPathCosts map target newTempMap newCostMap

let rec expandMap (mapToClone: list<Point * int>) (offsetX, offsetY) =
    let maxX = mapToClone |> Seq.map (fst >> fst) |> Seq.max
    let maxY = mapToClone |> Seq.map (fst >> snd) |> Seq.max
    let startX = (maxX + 1) * offsetX
    let startY = (maxY + 1) * offsetY
    mapToClone
    |> List.map (fun ((x, y), v) ->
        match v + offsetX + offsetY with
        | n when n >= 10 -> ((startX + x, startY + y), n - 9)
        | n -> ((startX + x, startY + y), n))


let part1 map =
    let maxX = map |> Map.toSeq |> Seq.map (fst >> fst) |> Seq.max
    let maxY = map |> Map.toSeq |> Seq.map (fst >> snd) |> Seq.max

    findPathCosts map (maxX, maxY) (new Map<Point, int>([((0,0),0)])) (new Map<Point, int>([])) (0, 0)

let part2 map = 
    let maxX = map |> Map.toSeq |> Seq.map (fst >> fst) |> Seq.max
    let maxY = map |> Map.toSeq |> Seq.map (fst >> snd) |> Seq.max

    let mapList = map |> Map.toList
    // for y in 0.. maxY do
    //     for x in 0 .. maxX do
    //         match map.TryFind (x, y) with
    //         | Some(v) -> printf "%d" v
    //         | _ -> printf "."
    //     printfn ""

    let newMap = 
        [ 0 .. 4 ]
        |> List.allPairs [ 0 .. 4 ]
        |> List.collect (expandMap mapList)
        |> Map.ofList

    // for y in 0.. (maxY + 1) * 5 do
    //     for x in 0 .. (maxX + 1) * 5 do
    //         match newMap.TryFind (x, y) with
    //         | Some(v) -> printf "%d" v
    //         | _ -> printf "."
    //     printfn ""

    findPathCosts newMap ((maxX * 5) + 4, (maxY * 5) + 4) (new Map<Point, int>([((0,0),0)])) (new Map<Point, int>([])) (0, 0)

[<EntryPoint>]
let main argv =
    let input = readIntMap "real_data.txt"

    printfn "Part 1: %d" (part1 input)
    printfn "Part 2: %d" (part2 input)

    0 // return an integer exit code