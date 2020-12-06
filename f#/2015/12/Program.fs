// Learn more about F# at http://fsharp.org

open System
open System.Text.RegularExpressions

let findAllNumbers (str: string) =
    let matches = (Regex ("-?[0-9]+")).Matches str
    [0..matches.Count - 1]
    |> List.map ((fun i -> (matches.Item i).Value) >> int)

let isBracket c = Array.contains c [| '{'; '['; ']'; '}' |]

let rec hasRed list =
    match list with
    | r::e::d::_ when r = 'r' && e = 'e' && d = 'd' -> true
    | _::rest -> hasRed rest
    | _ -> false

let rec filterRedObjects (before: char list) (str: char list) =
    let index = List.tryFindIndex isBracket str

    match index with
    | Some x -> let char = str.[x]
                let prev, next = List.splitAt (x + 1) str
                // printfn "char %c" char
                match char with
                | '{' -> let object, remaining = filterRedObjects [] next
                         let newBefore = before @ prev @ (if hasRed object then [] else object)
                        //  printfn "%A" newBefore
                        //  printfn "%A" remaining
                         filterRedObjects newBefore remaining
                | '[' -> let array, remaining = filterRedObjects [] next
                         let newBefore = before @ prev @ (List.filter (fun c -> c <> 'r') array)
                        //  printfn "%A" newBefore
                        //  printfn "%A" remaining
                         filterRedObjects newBefore remaining
                | ']' | '}' -> before @ prev, next
                | _ -> before @ str, []
    | _ -> before @ str, []

let filterObjects (input: string) =
    let objects = input
                    |> Seq.toList
                    |> filterRedObjects []
                    |> fst
    String.Join("", objects)

let sumAllNumbers (input: string) =
    input
    |> findAllNumbers
    |> List.sum

[<EntryPoint>]
let main argv =
    let input = IO.File.ReadAllLines "data.txt"
                |> Seq.head

    let part1 = input |> sumAllNumbers
    printfn "Part 1: %d" part1

    let part2 = input |> filterObjects |> sumAllNumbers
    printfn "Part 2 %d" part2
    0 // return an integer exit code
