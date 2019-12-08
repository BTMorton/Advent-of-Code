// Learn more about F# at http://fsharp.org

open System
open System.IO

let readFile (fileName: string) = seq {
    use sr = new StreamReader(fileName)
    while not sr.EndOfStream do
        yield sr.ReadLine()
}

let rec constructImage inputData width height =
    let layer, rest = List.splitAt (width * height) inputData
    match rest with
    | [] -> [layer]
    | _ -> layer :: (constructImage rest width height)

let convertToString num =
    match num with
        | 0 -> " "
        | 1 -> "\u2588"
        | _ -> ""

let combineLayers (map: Map<int, int>) index value =
    let newValue = map.[index]
    match newValue with
    | 0 -> " "
    | 1 -> "\u2588"
    | _ -> value


let rec processLayers imageData =
    match imageData with
    | [] -> []
    | [x] -> x |> List.map convertToString
    | x::rest ->
        let newValues = x |> List.mapi (fun i v -> (i,v))
                          |> (fun x -> new Map<int, int>(x))

        processLayers rest
        |> List.mapi (combineLayers newValues)

let rec drawRow width (imageData: string list) =
    let row, rest = List.splitAt width imageData
    let rowDisplay = String.Join("", row)
    match rest with
    | [] -> [rowDisplay]
    | _ ->  rowDisplay :: drawRow width rest

let drawImage width imageData =
    let image = processLayers imageData
    String.Join("\n", (drawRow width image))

[<EntryPoint>]
let main argv =
    let input = readFile "data.txt"
                |> Seq.head
                |> List.ofSeq
                |> List.map (string >> int)

    let image = constructImage input 25 6
    let smallestLayer = image
                        |> List.minBy (List.filter ((=) 0) >> List.length)
    printfn "smallestLayer %A" smallestLayer
    let part1 = smallestLayer
                |> List.countBy id
                |> List.filter (fst >> (<>) 0)
                |> List.map snd
                |> List.fold (*) 1
    printfn "Part 1 %d" part1

    printfn "Part 2\n%s" (drawImage 25 image)
    0 // return an integer exit code
