// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System
open AOCHelpers.AOC

let parseFold fold =
    match fold with
    | Prefix "fold along x=" rest -> (int rest, 0)
    | Prefix "fold along y=" rest -> (0, int rest)
    | _ -> (0, 0)

let applyFold point fold =
    match point with
    | v when v > fold -> (fold - v) + fold
    | v -> v

let doFoldX pointSet fold =
    pointSet
    |> Set.map (fun (x, y) -> (applyFold x fold, y))

let doFoldY pointSet fold =
    pointSet
    |> Set.map (fun (x, y) -> (x, applyFold y fold))

let doFold pointSet fold =
    match fold with
    | (0, 0) -> pointSet
    | (x, 0) -> doFoldX pointSet x
    | (0, y) -> doFoldY pointSet y
    | _ -> pointSet

let part1 (pointSet: Set<Point>) folds = 
    doFold pointSet (folds |> Seq.head)
    |> Set.count

let part2 pointSet folds =
    let newSet = folds
                 |> List.fold doFold pointSet
    let maxX = newSet |> Seq.maxBy fst |> fst
    let maxY = newSet |> Seq.maxBy snd |> snd

    for y in  0 .. maxY do
        for x in 0 .. maxX do
            if newSet.Contains (x, y)
            then printf "X"
            else printf "."

        printfn ""

    0

[<EntryPoint>]
let main argv =
    let lines = IO.File.ReadAllLines "real_data.txt"
                |> Seq.toList
    let gap = List.findIndex ((=) "") lines
    let (points, folds) = List.splitAt gap lines
    let pointSet = points
                    |> Seq.map parsePoint
                    |> Set.ofSeq
    let folds = folds
                |> List.skip 1
                |> List.map parseFold

    printfn "Part 1: %d" (part1 pointSet folds)
    printfn "Part 2: %d" (part2 pointSet folds)

    0 // return an integer exit code