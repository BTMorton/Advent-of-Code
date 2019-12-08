open IntCode
open System.IO

let readFile (fileName: string) = seq {
    use sr = new StreamReader(fileName)
    while not sr.EndOfStream do
        yield sr.ReadLine()
}

let processWithInput arr noun verb =
    Array.set arr 1 noun
    Array.set arr 2 verb

    try
        IntCode.execute arr
        Some(arr.[0])
    with
        | _ -> None

[<EntryPoint>]
let main argv =
    let opts = readFile "input_data.txt"
                |> Seq.head
                |> (fun x -> x.Split[|','|])
                |> Array.map int

    let part1 = (processWithInput (Array.copy opts) 12 2)
    printfn "Part 1: %d" (Option.get part1)

    [ for i in 0 .. 9999 -> async { return (processWithInput (Array.copy opts) (i / 100) (i % 100)), i } ]
    |> Async.Parallel
    |> Async.RunSynchronously
    |> Array.filter (fun (v, _) -> v = Some(19690720))
    |> Array.get
    <| 0
    |> snd
    |> printfn "Part 2: Final pair is %A"
    0 // return an integer exit code
