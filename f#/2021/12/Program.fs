// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System
open AOCHelpers.AOC

type CaveMap = Map<string, list<string>>

let visitSmallCavesOnce visited (cave: string) =
    Char.ToUpper cave.[0] = cave.[0] || not (List.contains cave visited)

let smallCaveVisitedTwice =
    List.countBy id 
    >> List.exists (fun (c: string, count) -> Char.IsLower c.[0] && count > 1)

let visitSmallCavesTwice visited (cave: string) =
    match cave with
    | c when Char.IsUpper c.[0] -> true
    | c when not (List.contains cave visited) -> true
    | _ -> smallCaveVisitedTwice visited |> not

let rec traverseCaves caveSelector (map: CaveMap) targetPosition currentPosition (route: list<string>) = 
    map.[currentPosition]
    |> List.filter (caveSelector route)
    |> List.fold (fun (routes: list<list<string>>) nextPosition ->
        let newRoute = nextPosition :: route

        match nextPosition with
        | x when x = targetPosition -> [newRoute]
        | x when map.ContainsKey x -> (traverseCaves caveSelector map targetPosition nextPosition newRoute)
        | _ -> []
        |> List.append routes 
        |> List.filter (List.length >> (<) 0)
    ) []

let traverseMap caveSelector (map: CaveMap) =
    traverseCaves caveSelector map "end" "start" ["start"]

let part1 = 
    traverseMap visitSmallCavesOnce
    >> List.length

let part2 = 
    traverseMap visitSmallCavesTwice
    >> List.length

[<EntryPoint>]
let main argv =
    let input = IO.File.ReadAllLines "real_data.txt"
                |> Seq.collect (split "-" >> (fun arr -> [(arr.[0], arr.[1]);(arr.[1], arr.[0])]))
                |> Seq.filter (snd >> (<>) "start")
                |> Seq.groupBy fst
                |> Seq.map (fun (k, v) -> (k, v |> Seq.map snd |> List.ofSeq))
                |> Map.ofSeq

    printfn "Part 1: %d" (part1 input)
    printfn "Part 2: %d" (part2 input)

    0 // return an integer exit code