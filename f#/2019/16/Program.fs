// Learn more about F# at http://fsharp.org

open System

let getIndexPattern index =
    let pattern = [1;0;-1;0] |> List.collect (List.replicate index)
    let rec getCycle() = seq {
        yield! pattern
        yield! getCycle()
    }
    getCycle()

let getFFTElement input index =
    input
    |> List.skip (index - 1)
    |> Seq.zip (getIndexPattern index)
    |> Seq.sumBy (fun (p,v) -> p * v)
    |> abs
    |> (%) <| 10


let applyFFT (input: int list) =
    [1..input.Length]
    |> List.map (getFFTElement input)

let applyFastFFT (input: int list) =
    input
    |> List.scanBack (fun a b -> (a + b) % 10) <| 0

let doMultipleApplications input iterations fn =
    [1..iterations]
    |> List.fold (fun l _ -> fn l) input

[<EntryPoint>]
let main argv =
    let input = IO.File.ReadAllLines "data.txt"
                |> Seq.head
                |> Seq.map (string >> int)
                |> Seq.toList


    let part1 = doMultipleApplications input 100 applyFFT
    printfn "Part 1: %s" (part1 |> List.take 8 |> (fun l -> String.Join("", l)))

    let repInput = input |> List.replicate 10000 |> List.concat
    let skip = repInput |> List.take 7 |> (fun l -> String.Join("", l)) |> int
    let newInput = repInput  |> List.skip skip
    let part2 = doMultipleApplications newInput 100 applyFastFFT
    printfn "Part 2: %s" (part2 |> List.take 8 |> (fun l -> String.Join("", l)))

    0 // return an integer exit code
