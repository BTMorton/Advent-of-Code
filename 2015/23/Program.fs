// Learn more about F# at http://fsharp.org

open System

let tryGetValue (regMap: Map<string, int64>) reg =
    if regMap.ContainsKey reg
    then regMap.[reg]
    else 0L

let half reg (regMap: Map<string, int64>) pointer =
    let prev = tryGetValue regMap reg
    regMap.Add(reg, prev / 2L), pointer + 1

let triple reg (regMap: Map<string, int64>) pointer =
    let prev = tryGetValue regMap reg
    regMap.Add(reg, prev * 3L), pointer + 1

let increment reg (regMap: Map<string, int64>) pointer =
    let prev = tryGetValue regMap reg
    regMap.Add(reg, prev + 1L), pointer + 1

let jump offset (regMap: Map<string, int64>) pointer =
    regMap, pointer + offset

let jumpIfEven reg offset (regMap: Map<string, int64>) pointer =
    let value = tryGetValue regMap reg
    let next = if value % 2L = 0L then pointer + offset else pointer + 1
    regMap, next

let jumpIfOne reg offset (regMap: Map<string, int64>) pointer =
    let value = tryGetValue regMap reg
    let next = if value = 1L then pointer + offset else pointer + 1
    regMap, next

let errorCase input (regMap: Map<string, int64>) _ =
    printfn "Unexpected instruction format %s" input
    regMap, -1

let getInstruction (instr: string) =
    let instrParts = instr.Split[|' '; ','|]
                     |> Array.filter ((<>) "")

    match instrParts.[0] with
    | "hlf" -> half instrParts.[1]
    | "tpl" -> triple instrParts.[1]
    | "inc" -> increment instrParts.[1]
    | "jmp" -> jump (instrParts.[1] |> int)
    | "jie" -> jumpIfEven instrParts.[1] (instrParts.[2] |> int)
    | "jio" -> jumpIfOne instrParts.[1] (instrParts.[2] |> int)
    | _ -> errorCase instr

let runProgram aInit (instructions: Map<int, Map<string, int64> -> int -> Map<string, int64> * int>) =
    let rec exec (regMap: Map<string, int64>) pointer =
        if instructions.ContainsKey pointer |> not
        then regMap
        else
            instructions.[pointer] regMap pointer
            ||> exec

    exec ([("a", aInit); ("b",0L)] |> Map.ofList) 0

[<EntryPoint>]
let main argv =
    let instructions =
        IO.File.ReadAllLines "data.txt"
        |> Array.map getInstruction
        |> Array.mapi (fun i c -> i,c)
        |> Map.ofArray

    let results = runProgram 0L instructions
    printfn "Part 1: %d" (tryGetValue results "b")

    let results = runProgram 1L instructions
    printfn "Part 2: %d" (tryGetValue results "b")
    0 // return an integer exit code
