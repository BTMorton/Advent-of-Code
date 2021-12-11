// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System
open AOCHelpers.AOC

type Result = Complete | Missing of list<char> | Error of char

let scoreError result =
    match result with
    | Error(')') -> 3
    | Error(']') -> 57
    | Error('}') -> 1197
    | Error('>') -> 25137
    | _ -> 0

let rec scoreMissing score result =
    match result with
    | ')'::tail -> scoreMissing ((5L * score) + 1L) tail
    | ']'::tail -> scoreMissing ((5L * score) + 2L) tail
    | '}'::tail -> scoreMissing ((5L * score) + 3L) tail
    | '>'::tail -> scoreMissing ((5L * score) + 4L) tail
    | _ -> score

let rec parseChunk (expected: list<char>) (str: list<char>) =
    match str with
    | '('::tail -> parseChunk (List.append [')'] expected) tail
    | '['::tail -> parseChunk (List.append [']'] expected) tail
    | '{'::tail -> parseChunk (List.append ['}'] expected) tail
    | '<'::tail -> parseChunk (List.append ['>'] expected) tail
    | c::tail when c = (List.head expected) -> parseChunk (List.tail expected) tail
    | c::tail -> Error(c)
    | [] when List.isEmpty expected -> Complete
    | [] -> Missing(expected)

let part1 =
    Seq.sumBy (parseChunk [] >> scoreError)

let pickMiddle results =
    let mid = (Seq.length results) / 2
    results
    |> Seq.sort
    |> Seq.item mid

let part2 =
    Seq.map (parseChunk [])
    >> Seq.choose (fun r -> match r with | Missing(l) -> Some(l) | _ -> None)
    >> Seq.map (scoreMissing 0L)
    >> pickMiddle

[<EntryPoint>]
let main argv =
    let lines = IO.File.ReadAllLines "real_data.txt"
                |> Seq.map Seq.toList

    printfn "Part 1: %d" (part1 lines)
    printfn "Part 2: %d" (part2 lines)

    0 // return an integer exit code