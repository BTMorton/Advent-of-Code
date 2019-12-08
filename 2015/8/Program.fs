// Learn more about F# at http://fsharp.org

open System
open System.Text.RegularExpressions

let removeEscapedChars (str: string) =
    (Regex (@"(^"")|(""$)")).Replace(str, "")
    |> Regex.Unescape

let replace (search: string) (replace: string) (source: string) =
    source.Replace(search, replace)

let escapeChars (str: string) =
    str
    |> replace @"\" @"\\"
    |> replace @"""" @"\"""
    |> sprintf @"""%s"""

[<EntryPoint>]
let main argv =
    let input = IO.File.ReadAllLines "data.txt"

    let rawCount = input |> Seq.sumBy String.length
    let strippedCount = input |> Seq.map removeEscapedChars |> Seq.sumBy String.length
    printfn "Part 1 raw: %d stripped: %d Total: %d" rawCount strippedCount (rawCount - strippedCount)

    let encodedCount = input |> Seq.map escapeChars |> Seq.sumBy String.length
    printfn "Part 2 encoded: %d Total: %d" encodedCount (encodedCount - rawCount)
    0 // return an integer exit code
