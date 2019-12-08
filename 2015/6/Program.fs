// Learn more about F# at http://fsharp.org

open System
open System.Text.RegularExpressions
open System.IO

let readFile (fileName: string) = seq {
    use sr = new StreamReader(fileName)
    while not sr.EndOfStream do
        yield sr.ReadLine()
}

type Action =
    On
    | Off
    | Toggle

type Point = int * int
type Instruction = {
    action: Action;
    fromLED: Point;
    toLED: Point;
}

let parsePoint (str: string) =
    let point = str.Split [|','|]
                |> Seq.map int
                |> List.ofSeq
    point.[0], point.[1]

let parseAction str =
    match str with
    | "turn on" -> Some On
    | "turn off" -> Some Off
    | "toggle" -> Some Toggle
    | _ -> None

let parseInstruction str =
    let reg = Regex("(?<action>turn (on|off)|toggle) (?<from>[0-9]+,[0-9]+) through (?<to>[0-9]+,[0-9]+)")
    let matches = reg.Match str
    let action = matches.Groups.["action"].Value |> parseAction
    let fromLED = matches.Groups.["from"].Value |> parsePoint
    let toLED = matches.Groups.["to"].Value |> parsePoint

    match action with
    | Some act ->
        Some {
            action = act;
            fromLED = fromLED;
            toLED = toLED;
        }
    | None -> None

let generateRange (fromX, fromY) (toX, toY) =
    [ for x = fromX to toX do
        for y = fromY to toY do
            (x,y) ]

let getAction2 action =
    match action with
    | Toggle -> (fun (lights: int array) x y -> Array.set lights (x * 1000 + y) (lights.[(x * 1000 + y)] + 2))
    | On -> (fun (lights: int array) x y -> Array.set lights (x * 1000 + y) (lights.[(x * 1000 + y)] + 1))
    | Off -> (fun (lights: int array) x y -> Array.set lights (x * 1000 + y) (max (lights.[(x * 1000 + y)] - 1) 0))

let processInstruction2 (lights: int array) instr =
    let fromX, fromY = instr.fromLED
    let toX, toY = instr.toLED

    for x = fromX to toX do
        for y = fromY to toY do
            (getAction2 instr.action) lights x y

    lights

let getAction1 action =
    match action with
    | Toggle -> (fun (lights: bool array) x y -> Array.set lights (x * 1000 + y) (not lights.[(x * 1000 + y)]))
    | On -> (fun (lights: bool array) x y -> Array.set lights (x * 1000 + y) true)
    | Off -> (fun (lights: bool array) x y -> Array.set lights (x * 1000 + y) false)

let processInstruction1 (lights: bool array) instr =
    let fromX, fromY = instr.fromLED
    let toX, toY = instr.toLED

    for x = fromX to toX do
        for y = fromY to toY do
            (getAction1 instr.action) lights x y

    lights

[<EntryPoint>]
let main argv =
    let input = readFile "data.txt"
                |> Seq.map parseInstruction
                |> Seq.fold (fun l i -> match i with
                                        | Some x -> x :: l
                                        | _ -> l) []
                |> List.rev

    let lights1 = [| for x = 0 to 999 do
                        for y = 0 to 999 do
                            false |]
    printfn "Part 1 %d" (input
                        |> Seq.fold processInstruction1 lights1
                        |> Array.filter id
                        |> Array.length)

    let lights2 = [| for x = 0 to 999 do
                        for y = 0 to 999 do
                            0 |]
    printfn "Part 2 %d" (input
                        |> Seq.fold processInstruction2 lights2
                        |> Array.sum)
    0 // return an integer exit code
