// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System

let addBinaryToCount count binary =
    match binary with
    | '1' -> count + 1
    | _ -> count - 1

let countBinaries current =
    Seq.zip current
    >> Seq.map (fun (count, binary) -> addBinaryToCount count binary)

let convertCountToBinary def count =
    let n = sign count
    match n with
    | 1 -> '1'
    | 0 -> def
    | _ -> '0'

let binCharToInt bit =
    match bit with
    | '1' -> 1
    | _ -> 0

let binSeqToInt acc bit =
    (acc * 2) + (binCharToInt bit)

let part1 inputs =
    let counts = Seq.fold countBinaries (inputs |> Seq.head |> Seq.map (fun _ -> 0)) inputs
    let gammaRate = counts |> Seq.map (convertCountToBinary '1') |> Seq.fold binSeqToInt 0
    let epsilonRate = counts |> Seq.map ((~-) >> (convertCountToBinary '1')) |> Seq.fold binSeqToInt 0

    gammaRate * epsilonRate

let rec findBestMatch op def (options: seq<string>) position =
    let target = options |> Seq.map (fun opt -> opt.[position]) |> Seq.fold addBinaryToCount 0 |> op |> convertCountToBinary def
    let newOptions =  options |> Seq.filter (fun opt -> opt.Chars(position) = target)
    printfn "%c %d %A" target position newOptions

    match newOptions with
    | result when Seq.length result = 1 -> result |> Seq.head
    | result when Seq.length result = 0 -> ""
    | _ -> findBestMatch op def newOptions (position + 1)

let part2 inputs =
    let oxRating = findBestMatch id '1' inputs 0 |> Seq.fold binSeqToInt 0
    let co2Rating = findBestMatch (~-) '0' inputs 0 |> Seq.fold binSeqToInt 0

    printfn "o2 %d co2 %d" oxRating co2Rating
    oxRating * co2Rating

[<EntryPoint>]
let main argv =
    let lines = IO.File.ReadAllLines "data.txt"

    printfn "Part 1: %d" (part1 lines)
    printfn "Part 2: %d" (part2 lines)
    0 // return an integer exit code