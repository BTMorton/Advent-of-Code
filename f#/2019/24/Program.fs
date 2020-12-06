// Learn more about F# at http://fsharp.org

open System

type Side = Left | Right | Top | Bottom

let isAlive char = char = '#'
let loadBugs y x char = (x,y), isAlive char
let loadBugRow y = Seq.mapi (loadBugs y)

let addVector (aX,aY) (bX,bY) =
    aX + bX, aY + bY

let countNeighbours (pointX, pointY) (state: Map<int*int, bool>) (maxX, maxY) =
    [
        (0,1)
        (0,-1)
        (1,0)
        (-1,0)
    ]
    |> List.map (addVector (pointX, pointY))
    |> List.filter (fun p -> state.ContainsKey(p) && state.[p])
    |> List.length

let step (maxX, maxY) (curState: Map<int*int, bool>) =
    let rec iterate point (newState: Map<int*int, bool>) =
        let x,y = point
        let isAlive = curState.[point]
        let neighbours = countNeighbours point curState (maxX, maxY)

        let updatedState = match isAlive, neighbours with
                            //  If not live and 1 or 2 bugs nearby, infest
                            | false,1 | false,2 -> newState.Add(point, true)
                            //  If dead, or alive with exactly one nearby, remain
                            | true,1 | false,_ -> newState
                            //  Else die
                            | true,_ -> newState.Add(point, false)

        match point with
        | x, y when x = maxX && y = maxY -> updatedState
        | x, _ when x = maxX -> iterate (0,y+1) updatedState
        | _ -> iterate (x+1,y) updatedState

    iterate (0,0) curState

let display (maxX, maxY) (curState: Map<int*int, bool>) =
    for y in [0..maxY] do
        for x in [0..maxX] do
            printf "%c" (
                if curState.ContainsKey (x,y) |> not
                then '?'
                else if curState.[(x,y)]
                then '#'
                else '.')
        printfn ""
    printfn ""

let calculateRating maxX (curState: Map<int*int, bool>) =
    let getIndex (x, y) = (y * (maxX + 1)) + x

    curState
    |> Map.toList
    |> List.filter snd
    |> List.sumBy (fst >> getIndex >> pown 2)

let getMaxPoint state =
    let points = state |> Map.toSeq |> Seq.map fst
    let maxX = points |> Seq.map fst |> Seq.max
    let maxY = points |> Seq.map snd |> Seq.max
    maxX, maxY

let part1 initialState =
    let max = getMaxPoint initialState
    let maxX = fst max

    let rec iter curState seenRatings =
        let curState = step max curState
        // display max curState
        let rating = calculateRating maxX curState

        if List.contains rating seenRatings
        then rating
        else iter curState (rating::seenRatings)

    iter initialState []

let emptyLayer =
    [0..4]
    |> List.collect (fun x ->
        [0..4]
        |> List.map (fun y -> (x,y),false)
    )
    |> List.filter (fst >> (<>) (2,2))
    |> Map.ofList

let getSide side (state: Map<int*int, bool>) =
    [0..4]
    |> List.map (fun i ->
        match side with
        | Left ->  state.[(0, i)]
        | Right ->  state.[(4, i)]
        | Top ->  state.[(i, 0)]
        | Bottom -> state.[(i, 4)]
    )

let getLowerNeighbours depth point (state: Map<int, Map<int*int, bool>>) =
    let depthState = if state.ContainsKey (depth - 1)
                     then state.[depth - 1]
                     else emptyLayer

    match point with
    | 1, 2 -> getSide Left depthState
    | 3, 2 -> getSide Right depthState
    | 2, 1 -> getSide Top depthState
    | 2, 3 -> getSide Bottom depthState
    | _ -> []

let getUpperNeighbours depth (pointX, pointY) (state: Map<int, Map<int*int, bool>>) =
    let depthState = if state.ContainsKey (depth + 1)
                     then state.[depth + 1]
                     else emptyLayer
    let xNeighbours =
        match pointX with
        | 0 -> [depthState.[(1, 2)]]
        | 4 -> [depthState.[(3, 2)]]
        | _ -> []
    let yNeighbours =
        match pointY with
        | 0 -> [depthState.[(2, 1)]]
        | 4 -> [depthState.[(2, 3)]]
        | _ -> []

    List.append xNeighbours yNeighbours

let getDepthNeighbours depth (pointX, pointY) (state: Map<int, Map<int*int, bool>>) =
    let curDepthState =
        if state.ContainsKey depth
        then state.[depth]
        else emptyLayer

    let levelNeighbours =
        [
            (0,1)
            (0,-1)
            (1,0)
            (-1,0)
        ]
        |> List.map (addVector (pointX, pointY))
        |> List.filter (fun p -> curDepthState.ContainsKey(p))
        |> List.map (fun p -> curDepthState.[p])

    let lowerNeighbours = getLowerNeighbours depth (pointX, pointY) state
    let upperNeighbours = getUpperNeighbours depth (pointX, pointY) state

    List.concat [levelNeighbours; lowerNeighbours; upperNeighbours]

let doStep (curState: Map<int, Map<int*int, bool>>) (newState: Map<int, Map<int*int, bool>>) depth =
    let rec iterate point (newState: Map<int, Map<int*int, bool>>) =
        if point = (2,2)
        then iterate (3, 2) newState
        else
        let depthState =
            if newState.ContainsKey depth
            then newState.[depth]
            else emptyLayer

        let isAlive =
            if depthState.ContainsKey point
            then depthState.[point]
            else false

        let neighbours =    getDepthNeighbours depth point curState
                            |> List.filter id
                            |> List.length

        let updatedState =  match isAlive, neighbours with
                            //  If not live and 1 or 2 bugs nearby, infest
                            | false, 1 | false, 2 -> newState.Add(depth, depthState.Add(point, true))
                            //  If dead, or alive with exactly one nearby, remain
                            | true, 1 | false, _ -> newState
                            //  Else die
                            | true, _ -> newState.Add(depth, depthState.Add(point, false))

        match point with
        | 1, 2 -> iterate (3, 2) updatedState
        | 4, 4 -> updatedState
        | 4, y -> iterate (0, y + 1) updatedState
        | x, y -> iterate (x + 1, y) updatedState

    iterate (0,0) newState

let stepDepth (curState: Map<int, Map<int*int, bool>>) =
    let depth = curState
                |> Map.toList
                |> List.map fst
    let min = depth |> List.min |> (+) -1
    let max = depth |> List.max |> (+) 1

    [min..max]
    |> List.fold (doStep curState) curState

let part2 (initialState: Map<int*int, bool>) iterations =
    let initialState = initialState.Remove (2,2)

    // display (4,4) initialState

    let rec iter curStates iterations =
        if iterations <= 0
        then curStates
        else iter (stepDepth curStates) (iterations - 1)

    iter (Map.ofList [(0, initialState)]) iterations

[<EntryPoint>]
let main argv =
    let bugs =
        [
            "#.###";
            ".....";
            "#..#.";
            "##.##";
            "..#.#";
        ]
        |> Seq.mapi loadBugRow
        |> Seq.collect id
        |> Map.ofSeq

    printfn "Part 1: %d" (part1 bugs)
    let result = (part2 bugs 200)
    let count = result
                |> Map.toList
                |> List.map snd
                |> List.collect (Map.toList >> List.filter snd)
                |> List.length

    printfn "Part 2: %d" count
    0 // return an integer exit code
