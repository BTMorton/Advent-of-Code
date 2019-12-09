// Learn more about F# at http://fsharp.org

open System

let findFill total containers =
    let rec iterate (used: int list) remain containers = seq {
        match remain, containers with
        | 0, _ -> yield used
        | x, _ when x < 0 -> ()
        | _, [] -> ()
        | x, c::cs ->
            yield! iterate (c::used) (x-c) cs
            yield! iterate used x cs
    }

    iterate [] total containers

[<EntryPoint>]
let main argv =
    let containers = IO.File.ReadAllLines "data.txt"
                     |> Seq.toList
                     |> List.map int

    let combinations = findFill 150 containers

    printfn "Part 1 %d" (combinations |> Seq.length)

    let min = combinations
              |> Seq.map List.length
              |> Seq.min
    let part2 = combinations
                |> Seq.filter (List.length >> (=) min)
                |> Seq.length
    printfn "Part 2 %d" part2
    0 // return an integer exit code
