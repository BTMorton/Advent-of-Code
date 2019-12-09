// Learn more about F# at http://fsharp.org

open System

let isOn char = char = '#'
let loadLight y x char = (x,y), isOn char
let loadLightRow y = Seq.mapi (loadLight y)

let getNeighbours (pointX, pointY) (state: Map<int*int, bool>) (maxX, maxY) =
    let minNX = max 0 (pointX - 1)
    let minNY = max 0 (pointY - 1)
    let maxNX = min maxX (pointX + 1)
    let maxNY = min maxY (pointY + 1)

    [minNX..maxNX]
    |> List.collect (fun x ->
        [minNY..maxNY]
        |> List.map (fun y -> if x = pointX && y = pointY then false else state.[(x,y)])
    )

let step (maxX, maxY) (curState: Map<int*int, bool>) =
    let rec iterate point (newState: Map<int*int, bool>) =
        let x,y = point
        let isOn = curState.[point]
        let neighbours = getNeighbours point curState (maxX, maxY)
                         |> List.filter id
                         |> List.length

        let updatedState = match isOn, neighbours with
                            | false,3 -> newState.Add(point, true)
                            | true,2 | true,3 | false,_ -> newState
                            | true,_ -> newState.Add(point, false)
        match point with
        | x, y when x = maxX && y = maxY -> updatedState
        | x, _ when x = maxX -> iterate (0,y+1) updatedState
        | _ -> iterate (x+1,y) updatedState

    iterate (0,0) curState

let countLightsOn (curState: Map<int*int, bool>) =
    curState
    |> Map.toSeq
    |> Seq.filter snd
    |> Seq.length

let getMaxPoint state =
    let points = state |> Map.toSeq |> Seq.map fst
    let maxX = points |> Seq.map fst |> Seq.max
    let maxY = points |> Seq.map snd |> Seq.max
    maxX, maxY

let playGameOfLife iterations state =
    let max = getMaxPoint state
    [1..iterations]
    |> List.fold (fun m _ -> step max m) state

let playBrokenGameOfLife iterations state =
    let maxX, maxY = getMaxPoint state

    let breakState (newState: Map<int*int, bool>) =
        newState.Add((0,0), true)
             .Add((maxX,0), true)
             .Add((0,maxY), true)
             .Add((maxX,maxY), true)

    let brokenStep (newState: Map<int*int, bool>) =
        (step (maxX, maxY) newState) |> breakState

    [1..iterations]
    |> List.fold (fun m _ -> brokenStep m) (breakState state)

[<EntryPoint>]
let main argv =
    let lights = IO.File.ReadAllLines "data.txt"
                 |> Seq.mapi loadLightRow
                 |> Seq.collect id
                 |> Map.ofSeq

    let part1 = playGameOfLife 100 lights
                 |> countLightsOn
    printfn "Part 1 %d" part1

    let part2 = playBrokenGameOfLife 100 lights
                |> countLightsOn
    printfn "Part 2 %d" part2
    0 // return an integer exit code
