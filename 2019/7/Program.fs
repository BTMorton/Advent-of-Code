// Learn more about F# at http://fsharp.org

open System
open IntCode.IntCode
open System.IO

let readFile (fileName: string) = seq {
    use sr = new StreamReader(fileName)
    while not sr.EndOfStream do
        yield sr.ReadLine()
}

let execute opts inputData =
    let ops = Array.copy opts
    let output = execute64 ops inputData
    output.output |> List.rev
                  |> List.head

let rec executeAmplifier opts seenPhases result =
    let list = new Set<int64>([0L..4L])
              |> Set.difference <| seenPhases
              |> Set.toList

    if list.Length = 1
        then
            let phase = list.[0]
            execute opts [phase; result], seenPhases.Add phase
        else
            list |> List.map (fun phase -> execute opts [phase; result]
                                           |> executeAmplifier opts (seenPhases.Add phase))
                |> List.maxBy fst

let generateAmplifiers opts phases =
    phases
    |> List.map (fun phase -> {
        memory = opts |> Array.mapi (fun i c -> int64(i),c) |> Map.ofArray;
        curIndex = 0L;
        input = [phase];
        output = [];
        state = Running;
        relativeBase = 0L;
    })

let rec generatePhases min max seenPhases =
    let list = new Set<int64>([min..max])
              |> Set.difference <| seenPhases
              |> Set.toList

    match list with
    | [x] -> [[x]]
    | phases -> phases
                |> List.map (fun x -> seenPhases.Add x
                                      |> generatePhases min max
                                      |> List.map (fun l -> x::l))
                |> List.collect id

let runAmplifier amp newInput =
    let input =
        amp.input
            |> List.rev
            |> (fun x -> newInput :: x)
            |> List.rev
    // printfn "input data %A" input
    let result = executeComputer {
        amp with
            input = input;
            state = Running
    }
    // printfn "output data %A" result.output
    result

let rec runAmplifiers startInput (amps: Computer list) =
    match amps with
    | amp::tail -> let result = runAmplifier amp startInput
                   result :: runAmplifiers (List.head result.output) tail
    | _ -> []

let getPipelineOutput = List.map (fun x -> x.output)
                        >> List.rev
                        >> List.head
                        >> List.head

let rec recurseAmplifiers amps =
    let lastAmp = amps |> List.rev |> List.head
    match lastAmp.state with
    | Halted -> amps
    | _ -> let startArg = if lastAmp.output.IsEmpty then 0L else lastAmp.output |> List.head
           recurseAmplifiers (runAmplifiers startArg amps)

[<EntryPoint>]
let main argv =
    // let opts = [|3;31;3;32;1002;32;10;32;1001;31;-2;31;1007;31;0;33;1002;33;7;33;1;33;31;31;1;32;31;31;4;31;99;0;0;0|]
    // let phases = [1;0;4;3;2]
    // let amps = generateAmplifiers opts phases
    // let results = runAmplifiers 0 amps
    // let max = results |> List.rev
    //                   |> List.head
    //                   |> (fun x -> x.output)
    //                   |> List.head
    // printfn "max %d" max
    let opts = readFile "input_data.txt"
                |> Seq.head
                |> (fun x -> x.Split[|','|])
                |> Array.map int64

    let results = generatePhases 0L 4L (new Set<int64>([]))
                        |> List.map
                            (generateAmplifiers opts
                            >> runAmplifiers 0L
                            >> getPipelineOutput)
    printfn "Part 1 %d" (List.max results)

    let results = generatePhases 5L 9L (new Set<int64>([]))
                        |> List.map
                            (generateAmplifiers opts
                            >> recurseAmplifiers
                            >> getPipelineOutput)
    printfn "Part 2 %d" (List.max results)
    0 // return an integer exit code
