// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System
open AOCHelpers.AOC

let linearFuelCost target (pos, count) =
    count * (pos - target |> abs)

let complexFuelCost target (pos, count) =
    let diff = pos - target |> abs
    let cost = (diff * (diff + 1)) / 2
    count * cost

let calculateFuelRequirement fuelCost groups target =
    groups
    |> Seq.map (fuelCost target)
    |> Seq.fold (+) 0

let findMinimumFuelRequirement fuelCost input = 
    let groups = input |> Seq.countBy id

    { 0 .. (Seq.max input) }
    |> Seq.map (calculateFuelRequirement fuelCost groups)
    |> Seq.min

let part1 = 
    findMinimumFuelRequirement linearFuelCost

let part2 = 
    findMinimumFuelRequirement complexFuelCost

[<EntryPoint>]
let main argv =
    let input = readCommaSepIntList "real_data.txt"

    printfn "Part 1: %d" (part1 input)
    printfn "Part 2: %d" (part2 input)

    0 // return an integer exit code