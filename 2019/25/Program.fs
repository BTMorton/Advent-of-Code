// Learn more about F# at http://fsharp.org

open System
open IntCode.IntCode

let printOutput output =
    printfn "%s"
        (output
        |> List.rev
        |> List.map char
        |> (fun l -> String.Join("", l)))

let processInput line =
    line
    |> Seq.map int64
    |> Seq.toList

let runProgram computer =
    let rec iter computer =
        let computer = resumeComputer computer
        printOutput computer.output

        let line = Console.ReadLine ()

        if line = "exit"
        then ()
        else
            let computer = {
                computer with
                    input = processInput (line + "\n")
                    output = []
            }
            iter computer

    iter computer

[<EntryPoint>]
let main argv =
    let opts = loadFile "data.txt"

    let computer = createComputer opts []
    runProgram computer
    0 // return an integer exit code
