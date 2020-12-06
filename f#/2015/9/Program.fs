// Learn more about F# at http://fsharp.org

open System
open System.Text.RegularExpressions

type Route = {
    fromCity: string;
    toCity: string;
    distance: int;
}

let distanceRegex = Regex ("(?<from>[a-z]+) to (?<to>[a-z]+) = (?<distance>[0-9]+)", RegexOptions.IgnoreCase)

let parseRoute (route: string) =
    let m = distanceRegex.Match route
    if not m.Success
    then None
    else Some {
        fromCity = m.Groups.["from"].Value;
        toCity = m.Groups.["to"].Value;
        distance = m.Groups.["distance"].Value |> int;
    }

let addToMap distance cityA cityB (map: Map<string, Map<string, int>>) =
    map.Add(cityA,
        if map.ContainsKey cityA
        then map.[cityA].Add(cityB, distance)
        else new Map<string, int>([(cityB, distance)])
    )

let createRouteMap (map: Map<string, Map<string, int>>) (route: Route) =
    map
    |> addToMap route.distance route.fromCity route.toCity
    |> addToMap route.distance route.toCity route.fromCity

let routeMap = IO.File.ReadAllLines "data.txt"
                |> Seq.map parseRoute
                |> Seq.choose id
                |> Seq.fold createRouteMap (new Map<string, Map<string, int>>([]))

let getDistance from toCity =
    routeMap.[from].[toCity]

let getCities =
    routeMap
    |> Map.toSeq
    |> Seq.map fst

let rec findShortestRoute visited currentDistance =
    let toVisit = getCities
                    |> Set.ofSeq
                    |> Set.difference <| (visited |> Set.ofList)

    let lastVisit = List.head visited

    if toVisit.IsEmpty
    then seq {(visited, currentDistance)}
    else
        toVisit
        |> Seq.collect (fun city -> findShortestRoute (city::visited) (currentDistance + routeMap.[city].[lastVisit]))

[<EntryPoint>]
let main argv =
    let part1 = getCities
                |> Seq.collect (fun x -> findShortestRoute [x] 0)
                |> Seq.minBy snd
    printfn "Part 1 %A" part1

    let part2 = getCities
                |> Seq.collect (fun x -> findShortestRoute [x] 0)
                |> Seq.maxBy snd
    printfn "Part 2 %A" part2
    0 // return an integer exit code
