// Learn more about F# at http://fsharp.org

open System
open IntCode.IntCode

let getPointsInArea opts maxX maxY =
    [0L..(maxX - 1L)]
    |> List.collect (fun x ->
        [0L..(maxY - 1L)]
        |> List.collect (fun y ->
            let result = execute64 opts [x;y]
            result.output
        )
    )
    |> List.sum

let findSquareFit opts squareX squareY =
    let inTractor xVal yVal =
        let result = execute64 opts [xVal; yVal]
        result.output
        |> List.head
        |> (=) 1L

    let rec followBeam x y =
        let inBeam = inTractor x y
        if inBeam
        then if inTractor (x - (squareX - 1L)) (y + (squareY - 1L))
             then (x - (squareX - 1L), y)
             else followBeam (x + 1L) y
        else followBeam x (y + 1L)

    followBeam squareX 0L

[<EntryPoint>]
let main argv =
    let opts = loadFile "data.txt"

    printfn "Part 1: %d" (getPointsInArea opts 50L 50L)
    let result = (findSquareFit opts 100L 100L)
    printfn "Part 2: %A %d" result (fst result * 10000L + snd result)
    0 // return an integer exit code
