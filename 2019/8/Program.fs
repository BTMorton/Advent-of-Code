// Learn more about F# at http://fsharp.org

open System
open System.IO

let readFile (fileName: string) = seq {
    use sr = new StreamReader(fileName)
    while not sr.EndOfStream do
        yield sr.ReadLine()
}

let convertToString num =
    match num with
    | 0 -> " "
    | 1 -> "\u2588"
    | _ -> ""

let combineLayer (topVal, bottomVal) =
    match topVal with
    | 0 -> 0
    | 1 -> 1
    | _ -> bottomVal


let rec processLayers (imageData: int array seq) =
    let head = Seq.head imageData
    if Seq.length imageData = 1
    then head
    else
        imageData
        |> Seq.tail
        |> processLayers
        |> Array.zip head
        |> Array.map combineLayer

let drawImage width imageData =
    let image = processLayers imageData
                |> Array.map convertToString
                |> Array.chunkBySize width
                |> Array.map (fun row -> String.Join("", row))
    String.Join("\n", image)

[<EntryPoint>]
let main argv =
    let imageLayers = File.ReadAllLines "data.txt"
                        |> Seq.head
                        |> Seq.map (string >> int)
                        |> Seq.chunkBySize (25 * 6)

    let part1 = imageLayers
                |> Seq.map (Seq.countBy id)
                |> Seq.minBy ((Seq.find (fst >> (=) 0)) >> snd)
                |> Seq.filter (fst >> (<>) 0)
                |> Seq.map snd
                |> Seq.fold (*) 1
    printfn "Part 1 %d" part1

    printfn "Part 2\n%s" (drawImage 25 imageLayers)
    0 // return an integer exit code
