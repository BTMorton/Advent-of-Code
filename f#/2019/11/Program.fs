// Learn more about F# at http://fsharp.org

open System
open IntCode.IntCode

type Direction = Up | Right | Down | Left

let getNextDirection direction turn =
    match direction, turn with
    | Up,0L -> Left
    | Up,_ -> Right
    | Right,0L -> Up
    | Right,_ -> Down
    | Down,0L -> Right
    | Down,_ -> Left
    | Left,0L -> Down
    | Left,_ -> Up

let getMovement direction =
    match direction with
    | Up -> (0,-1)
    | Right -> (1,0)
    | Down -> (0,1)
    | Left -> (-1,0)

let paintPanel opts start =
    let computer = createComputer opts [if start then 1L else 0L]

    let rec iter (panel: Map<int*int, bool>) (posX, posY) currentDirection computer =
        let result = executeComputer computer
        if result.state = Halted
        then panel
        else
            // printfn "%A" result
            if Seq.length result.output > 1
            then
                let output = result.output |> Seq.rev
                let newPanel = panel.Add((posX, posY), (output |> Seq.item 0) = 1L)
                let newDirection = getNextDirection currentDirection (output |> Seq.item 1)
                let moveX, moveY = getMovement newDirection
                let newPosition = posX + moveX, posY + moveY
                let currentSquare = if panel.ContainsKey(newPosition) && panel.[(newPosition)] then 1L else 0L

                let newComputer = { result with input = [currentSquare]; output = []; state = Running }
                // printfn "%A" newComputer
                iter newPanel newPosition newDirection newComputer
            else
                // let currentSquare = if panel.ContainsKey((posX, posY)) && panel.[((posX, posY))] then 1L else 0L
                // let newComputer = { result with input = [currentSquare]; output = [] }

                // iter panel (posX, posY) currentDirection newComputer
                panel

    iter (Map.ofList [((0,0), start)]) (0,0) Up computer

let getMapMax map =
    let points = map |> Map.toSeq |> Seq.map fst
    let maxX = points |> Seq.maxBy fst |> fst
    let maxY = points |> Seq.maxBy snd |> snd
    maxX, maxY

let getMapMin map =
    let points = map |> Map.toSeq |> Seq.map fst
    let minX = points |> Seq.minBy fst |> fst
    let minY = points |> Seq.minBy snd |> snd
    minX, minY

let printPanel panel =
    let (minX, minY) = getMapMin panel
    let (maxX, maxY) = getMapMax panel
    printfn "%d,%d - %d,%d" minX minY maxX maxY
    [minY..maxY]
    |> Seq.map (fun y -> [minX..maxX]
                         |> Seq.collect (fun x -> if panel.ContainsKey (x,y) && panel.[(x,y)] then "\u2588" else " ")
                         |> (fun s -> String.Join("", s)))
    |> (fun s -> String.Join("\n", s))
    |> printfn "%s"


[<EntryPoint>]
let main argv =
    let input = loadFile "data.txt"

    let panel = paintPanel input false
    printfn "Part 1: %d" (panel |> Map.count)
    printfn "Part 2"
    printPanel (paintPanel input true)
    0 // return an integer exit code
