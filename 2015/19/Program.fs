// Learn more about F# at http://fsharp.org

open System

let addToMap (map: Map<string, string list>) search replace =
    if map.ContainsKey search
    then map.Add(search, replace::map.[search])
    else map.Add(search, [replace])

let parseInput (map: Map<string, string list>) (line: string) =
    let split = line.Split("=>")
                |> Array.map (fun s -> s.Trim())
    addToMap map split.[0] split.[1]

let applyReplacement (search: string) ((before,after): string list * string list) (replacement: string) =
    String.Join(search, before) + replacement + String.Join(search, after)

let applyReplacements (input: string) ((search: string), replacements) =
    let splits = input.Split(search) |> Seq.toList
    let count = splits.Length

    if count = 1 then []
    else [1..count-1]
         |> List.collect (fun i -> List.map (applyReplacement search (List.splitAt i splits)) replacements)

let findReplacements (map: Map<string, string list>) (input: string) =
    map
    |> Map.toList
    |> List.collect (applyReplacements input)
    |> List.distinct

let countAllReplacements (input: int * string) ((search, replacements): string * string list) =
    let splits = replacements
                 |> List.fold (fun i r -> List.collect (fun (s: string) -> s.Split(r) |> Seq.toList) i) [snd input]
    let count = (splits |> Seq.length) - 1 + (fst input)
    count, String.Join(search, splits)

let reverseReplacements (map: Map<string, string list>) (start: string) =
    let rec iter pair =
        let replaced = map
                        |> Map.toList
                        |> List.fold countAllReplacements pair

        if snd replaced = snd pair
        then pair
        else iter replaced

    iter (0,start)

[<EntryPoint>]
let main argv =
    let input = IO.File.ReadAllLines "data.txt"

    let molecule = input |> Seq.head
    let replacements = input
                        |> Seq.tail
                        |> Seq.fold parseInput (Map.ofList [])

    let allCombinations = molecule |> findReplacements replacements
    printfn "Part 1: %d" (allCombinations |> Seq.length)
    let part2 = (reverseReplacements replacements molecule)
    printfn "Part 2: %s steps %d" (snd part2) (fst part2)
    0 // return an integer exit code
