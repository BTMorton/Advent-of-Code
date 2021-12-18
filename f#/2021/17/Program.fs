// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System
open AOCHelpers.AOC

let isInt =
    (fun x -> x - round x)
    >> (=) 0.0

let isTriangularNumber =
    abs
    >> (*) 8
    >> (+) 1
    >> float
    >> sqrt
    >> isInt

let triangularNumber n =
    (n * (n + 1)) / 2

let calculateTriangularNumber =
    abs
    >> (*) 8
    >> (+) 1
    >> float
    >> sqrt
    >> (+) -1.0
    >> (*) 0.5
    >> int

let findTriangularNumbers min max =
    [ min .. max ]
    |> Seq.filter isTriangularNumber
    // |> Seq.map calculateTriangularNumber

let findValidInf (minX, minY) (maxX, maxY) =
    let validX = findTriangularNumbers minX maxX
                 |> Seq.map calculateTriangularNumber
    
    let absMinY = abs minY 
    let absMaxY = abs maxY 
    let validY = findTriangularNumbers absMaxY (absMinY * absMinY)
                 |> Seq.collect (fun h -> findTriangularNumbers (h - absMinY) (h - absMaxY))
                 |> Set.ofSeq
                 |> Seq.map calculateTriangularNumber

    Seq.allPairs validX validY
    |> Set.ofSeq
    
let findValidStepDir minX maxX step =
    [ minX .. maxX ]
    |> Seq.map ((+) (step - 1 |> triangularNumber))
    |> Seq.map float
    |> Seq.map ((*) (1.0 / float step))
    |> Seq.filter isInt
    |> Seq.map int

let findValidStep (minX, minY) (maxX, maxY) step =
    let validX = (findValidStepDir minX maxX step) |> Seq.filter ((<=) step)
    let validY = (findValidStepDir minY maxY step)
    Seq.allPairs validX validY

let part1 (_, minY) (_, maxY) = 
    let absMinY = abs minY 
    let absMaxY = abs maxY 

    findTriangularNumbers absMaxY (absMinY * absMinY)
    |> Seq.collect (fun h -> findTriangularNumbers (h - absMinY) (h - absMaxY))
    |> Seq.max

let part2 (minX, minY: int) (maxX, maxY) = 
    let min = Math.Max(maxX, (abs minY))

    [1..minX]
    |> Seq.collect (findValidStep (minX, minY) (maxX, maxY))
    |> Seq.append (findValidInf (minX, minY) (maxX, maxY))
    |> Set.ofSeq
    |> Set.count

[<EntryPoint>]
let main argv =
    // let input = IO.File.ReadAllLines "test_data.txt"
    // let input = readIntLines "test_data.txt"
    // let min = (20, -10)
    // let max = (30, -5)

    let min = (111, -154)
    let max = (161, -101)

    printfn "Part 1: %d" (part1 min max)
    printfn "Part 2: %d" (part2 min max)

    0 // return an integer exit code