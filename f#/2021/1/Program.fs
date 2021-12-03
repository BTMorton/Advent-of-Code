// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System

// Define a function to construct a message to print
let part1 =
    Seq.pairwise
    >> Seq.map (fun (a, b) -> a - b)
    >> Seq.filter ((>) 0)
    >> Seq.length

let tripwise (source: seq<'T>) =
    seq {
        use ie = source.GetEnumerator() 
        if ie.MoveNext() then
            let aref = ref ie.Current
            if ie.MoveNext() then
                let bref = ref ie.Current
                while ie.MoveNext() do
                    let c = ie.Current 
                    yield (!aref, !bref, c)
                    aref := !bref
                    bref := c
    }

let part2 =
    tripwise
    >> Seq.map (fun (a, b, c) -> a + b + c)
    >> part1

[<EntryPoint>]
let main argv =
    let lines = IO.File.ReadAllLines "data.txt"
    let inputs = lines |> Seq.map (int)

    printfn "Part 1 Total Changes: %d" (part1 inputs)
    printfn "Part 2 Total Changes: %d" (part2 inputs)
    0 // return an integer exit code