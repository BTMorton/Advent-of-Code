// Learn more about F# at http://fsharp.org

open System
let a = 'a' |> int
let charToInt = int >> (+) -a
let intToChar = (+) <| a >> char

let rec increment (list: int list) =
    match list with
    | [] -> []
    | head::tail ->
         match head with
         | x when x >= 25 -> 0::(increment tail)
         | 7 | 10 | 13 -> (head + 2)::tail
         | _ -> (head + 1)::tail

let rec countDoubles input =
    match input with
    | a::b::rest when a = b -> 1 + countDoubles rest
    | _::rest -> countDoubles rest
    | _ -> 0

let rec hasTripleIncrement input =
    match input with
    //  This is - because the list is reversed
    | a::b::c::_ when (a - 1) = b && (b - 1) = c -> true
    | _::rest -> hasTripleIncrement rest
    | _ -> false

let isValid input =
    hasTripleIncrement input &&
    (countDoubles input) >= 2

let rec skipInvalidChars input =
    match input with
    | [] -> []
    | head::rest ->
        match head with
        | 8 | 11 | 14 -> (head + 1)::(rest |> List.map (fun _ -> 0))
        | _ -> head::(skipInvalidChars rest)

let display = List.rev >> List.map intToChar >> (fun i -> String.Join("", i))

let rec iterate input =
    let output = increment input
    if isValid output
    then output
    else iterate output

[<EntryPoint>]
let main argv =
    let input = "hepxcrrq"
                |> Seq.map charToInt
                |> Seq.toList
                |> skipInvalidChars
                |> List.rev

    let part1 = input |> iterate
    printfn "Part 1: %s" (display part1)
    let part2 = part1 |> iterate
    printfn "Part 2: %s" (display part2)

    0 // return an integer exit code
