// Learn more about F# at http://fsharp.org

open System
open System.IO

let convertToString num =
    match num with
    | 1 -> "\u2588"
    | _ -> " "

let combineLayer (topVal, bottomVal) =
    match topVal with
    | 0 | 1 -> topVal
    | _ -> bottomVal

let rec processLayers topLayer bottomLayer =
    bottomLayer
    |> Array.zip topLayer
    |> Array.map combineLayer

let drawImage width imageData =
    imageData
    |> Seq.reduce processLayers
    |> Array.map convertToString
    |> Array.chunkBySize width
    |> Array.map (String.concat "")
    |> String.concat "\n"

[<EntryPoint>]
let main argv =
    let imageLayers = File.ReadAllLines "data.txt"
                        |> Seq.head
                        |> Seq.map (string >> int)
                        |> Seq.chunkBySize (25 * 6)

    let countNum num = Seq.filter ((=) num) >> Seq.length
    let smallestLayer = imageLayers
                        |> Seq.minBy (countNum 0)
    printfn "Part 1 %d" ((countNum 1 smallestLayer) * (countNum 2 smallestLayer))

    printfn "Part 2\n%s" (drawImage 25 imageLayers)
    0 // return an integer exit code
