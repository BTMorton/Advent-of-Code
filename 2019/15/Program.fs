// Learn more about F# at http://fsharp.org

open System
open IntCode.IntCode

type Tile = Wall | Space | Target
type Direction = North | South | West | East
type Position = int * int
type AreaMap = Map<Position, Tile>
type DistanceMap = Map<Position, int>

let getTileFromOutput output =
    match output with
    | 1L -> Space
    | 2L -> Target
    | _ -> Wall

let getCommand dir =
    match dir with
    | North -> 1L
    | South -> 2L
    | West -> 3L
    | East -> 4L

let getMovement dir =
    match dir with
    | North -> (0, 1)
    | South -> (0, -1)
    | West -> (-1, 0)
    | East -> (1, 0)

let getDirectionLeft dir =
    match dir with
    | North -> West
    | South -> East
    | West -> South
    | East -> North

let getDirectionRight dir =
    match dir with
    | North -> East
    | South -> West
    | West -> North
    | East -> South

let addPositions (posAX, posAY) (posBX,posBY) = (posAX + posBX),(posAY + posBY)

let mapArea program =
    let computer = createComputer program []

    let rec iter computer (map: AreaMap) currentPosition nextDirection =
        let nextPosition = addPositions currentPosition (getMovement nextDirection)
        let resultComputer = resumeComputer { computer with input = [ (getCommand nextDirection) ]; output = [] }
        if resultComputer.state = Halted
        then map
        else
        let result = resultComputer.output |> List.head |> getTileFromOutput
        let map = map.Add(nextPosition, result)
        let newPosition = if result = Wall then currentPosition else nextPosition
        let newDirection = if result = Wall then getDirectionRight nextDirection else getDirectionLeft nextDirection

        if newPosition = (0,0) && currentPosition <> (0,0)
        then map
        else
        iter resultComputer map newPosition newDirection

    iter computer ([((0,0),Space)] |> Map.ofList) (0,0) North

let calculateDistances (map: AreaMap) startPoint =
    let rec iter currentPosition currentStepCount (distances: DistanceMap) =
        let nextStepCount = currentStepCount + 1

        [North; South; West; East]
        |> List.map (getMovement >> (addPositions currentPosition))
        |> List.filter (fun p -> map.[p] <> Wall)
        |> List.filter (fun p -> not (distances.ContainsKey p) || distances.[p] > nextStepCount)
        |> List.fold (fun (d: DistanceMap) p -> iter p nextStepCount (d.Add(p, nextStepCount))) distances

    iter startPoint 0 ([(startPoint, 0)] |> Map.ofList)


[<EntryPoint>]
let main argv =
    let opts = loadFile "data.txt"

    let map = mapArea opts
    let targetLocation = map |> Map.toList |> List.filter (snd >> (=) Target) |> List.head |> fst
    let distances = calculateDistances map targetLocation

    printfn "Part 1: %d" distances.[(0,0)]

    printfn "Part 2: %d" (distances |> Map.toList |> List.map snd |> List.max)
    0 // return an integer exit code
