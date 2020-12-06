// Learn more about F# at http://fsharp.org

open System
open System.Text.RegularExpressions

let happinessRegex = Regex ("(?<person1>[a-z]+) would ((?<gain>gain)|(?<lose>lose)) (?<change>[0-9]+) happiness units by sitting next to (?<person2>[a-z]+).", RegexOptions.IgnoreCase)

let addToMap happiness personA personB (map: Map<string, Map<string, int>>) =
    map.Add(personA,
        if map.ContainsKey personA
        then map.[personA].Add(personB, happiness)
        else new Map<string, int>([(personB, happiness)])
    )

let parseInput (map: Map<string, Map<string, int>>) (input: string) =
    let m = happinessRegex.Match input
    let happiness = m.Groups.["change"].Value
                    |> int
                    |> (if m.Groups.["gain"].Success then (+) else (-)) 0
    printfn "%d" happiness
    addToMap happiness m.Groups.["person1"].Value m.Groups.["person2"].Value map

let getHappinessChange (map: Map<string, Map<string, int>>) person1 person2 =
    map.[person1].[person2]
    + map.[person2].[person1]

let getPeople (map: Map<string, Map<string, int>>) =
    map
    |> Map.toSeq
    |> Seq.map fst

let rec findOptimalSeating seated currentHappiness (map: Map<string, Map<string, int>>) =
    let toSeat = getPeople map
                |> Set.ofSeq
                |> Set.difference <| (seated |> Set.ofList)

    let lastSeated = List.head seated

    if toSeat.IsEmpty
    then let firstSeated = seated |> List.rev |> List.head
         seq {(seated, (currentHappiness + getHappinessChange map firstSeated lastSeated))}
    else
        toSeat
        |> Seq.collect (fun person -> findOptimalSeating (person::seated) (currentHappiness + getHappinessChange map person lastSeated) map)

[<EntryPoint>]
let main argv =
    let happinessMap =  IO.File.ReadAllLines "data.txt"
                        |> Seq.fold parseInput (new Map<string, Map<string, int>>([]))

    let part1 = getPeople happinessMap
                |> Seq.collect (fun x -> findOptimalSeating [x] 0 happinessMap)
    printfn "Part 1 %A" (part1 |> Seq.maxBy snd)

    let part2 = getPeople happinessMap
                 |> Seq.fold (fun m p -> m |> addToMap 0 "You" p |> addToMap 0 p "You") happinessMap
                 |> findOptimalSeating ["You"] 0
    printfn "Part 2 %A" (part2 |> Seq.maxBy snd)
    0 // return an integer exit code
