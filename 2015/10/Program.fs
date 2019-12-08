// Learn more about F# at http://fsharp.org

open System

let rec countChars curChar (str: int list) =
    match List.tryFindIndex ((<>) curChar) str with
    | Some count -> count
    | _ -> str.Length

let rec lookAndSay (input: int list) output =
    // printfn "str %A counts %A" str counts
    if input.IsEmpty then output
    else
        let curChar = List.head input
        let count = (countChars curChar input)
        lookAndSay (List.skip count input) (curChar::count::output)

let loopLookAndSay iter (input: int list) =
    [1..iter] |> List.fold (fun s _ -> lookAndSay s [] |> List.rev) input

[<EntryPoint>]
let main argv =
    let input = "3113322113"
                |> Seq.toList
                |> List.map (string >> int)
    let part1 = loopLookAndSay 40 input
    printfn "Part 1 %d" (part1 |> List.length)

    let part2 = loopLookAndSay 10 part1
    printfn "Part 2 %d" (part2 |> List.length)
    0 // return an integer exit code
