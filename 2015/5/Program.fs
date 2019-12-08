// Learn more about F# at http://fsharp.org

open System
open System.IO

let readFile (fileName: string) = seq {
    use sr = new StreamReader(fileName)
    while not sr.EndOfStream do
        yield sr.ReadLine()
}

let isVowel char =
    match char with
    | 'a'|'e'|'i'|'o'|'u' -> true
    | _ -> false

let hasVowels str =
    str
    |> Seq.filter isVowel
    |> Seq.length
    |> (<=) 3

let rec hasDouble str =
    match str with
    | a::b::_ when a = b -> true
    | _::rest -> hasDouble rest
    | _ -> false

let hasNoInvalid (str: string) =
    let invalid = [ "ab"; "cd"; "pq"; "xy" ]

    let count = invalid
                |> List.filter str.Contains
                |> List.length
    count = 0

let rec hasRepeatedLetter str =
    match str with
    | a::b::c::_ when a = c -> true
    | _::rest when rest.Length >= 3 -> hasRepeatedLetter rest
    | _ -> false

// let rec hasTwoPair (str: char list) =
//     match str with
//     | a::b::rest when (String.Concat rest).Contains(String.Concat [a;b]) -> true
//     | _::rest -> hasTwoPair rest
//     | _ -> false

let rec hasTwoPair (str: string) =
    if str.Length < 4 then false
    else
        let pair = str.Substring(0,2)
        let rest = str.Substring 2
        if rest.Contains pair
            then true
            else hasTwoPair (str.Substring 1)

[<EntryPoint>]
let main argv =
    let input = readFile "data.txt"
    printfn "Part 1 %d" (input
                         |> Seq.filter hasVowels
                         |> Seq.filter (List.ofSeq >> hasDouble)
                         |> Seq.filter hasNoInvalid
                         |> Seq.length)
    let altInput = [
        "qjhvhtzxzqqjkmpb"
        "xxyxx"
        "uurcxstgmygtbstg"
        "ieodomkazucvgmuy"
    ]
    let matchedWords = input
                        |> Seq.filter (List.ofSeq >> hasRepeatedLetter)
                        |> Seq.filter hasTwoPair//(List.ofSeq >> hasTwoPair)
                        |> Seq.length

    printfn "Part 2 %d" matchedWords
    0 // return an integer exit code
