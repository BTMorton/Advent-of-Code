let convertToString num = if num = 1 then "\u2588" else " "
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
    let imageLayers = System.IO.File.ReadAllLines "data.txt"
                        |> Seq.head
                        |> Seq.map (string >> int)
                        |> Seq.chunkBySize (25 * 6)

    let countNum num = Seq.filter ((=) num) >> Seq.length
    let smallestLayer = imageLayers
                        |> Seq.minBy (countNum 0)
    printfn "Part 1 %d" ((countNum 1 smallestLayer) * (countNum 2 smallestLayer))

    printfn "Part 2\n%s" (drawImage 25 imageLayers)
    0 // return an integer exit code
