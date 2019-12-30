// Learn more about F# at http://fsharp.org

open System

type Coord = int * int

type KeyPoint =
    | Entrance of Coord
    | Wall
    | Space
    | Key of char
    | Door of char

type KeyPointMap = Map<Coord, KeyPoint>

type Direction =
    Up | Down | Left | Right

type Route = {
    stepCount: int
    requiredKeys: KeyPoint list
}

type KeyPointRoutes = Map<KeyPoint, Map<KeyPoint, Route>>

let addVectors (aX,aY) (bX,bY) =
    (aX + bX), (aY + bY)

let addMovement (posX,posY) direction = 
    match direction with
    | Up -> (posX, posY - 1)
    | Down -> (posX, posY + 1)
    | Left -> (posX - 1, posY)
    | Right -> (posX + 1, posY)

let getPointFromChar char pos =
    match char with
    | '#' -> Wall
    | '@' -> Entrance pos
    | x when Char.IsLower x -> Key x
    | x when Char.IsUpper x -> Door x
    | _ -> Space

let addRowToMap (map: KeyPointMap) (y: int, row: string) =
    row
    |> Seq.mapi (fun x c -> x,c)
    |> Seq.fold (fun (m: KeyPointMap) (x,c) -> m.Add((x,y), getPointFromChar c (x,y))) map

let IsKey point = match point with | Key _ -> true | _ -> false

let IsEntrance p =
    match p with
    | Entrance _ -> true
    | _ -> false

let findStartPoints =
    Map.toList
    >> List.filter (snd >> IsEntrance)
    >> List.map snd

let generateRoutes (map: KeyPointMap) =
    let isNextSpaceOpen (map: KeyPointMap) newPosition = map.[newPosition] <> Wall

    let rec walkRoute stepCount visitedPoints passedDoors currentPosition: list<KeyPoint * Route> =
        let keyMatch =
            match map.[currentPosition] with
            | Key k -> [Key k, { stepCount = stepCount; requiredKeys = passedDoors; }]
            | _ -> []

        let passedDoors =
            match map.[currentPosition] with
            | Door d -> (Key (Char.ToLower d))::passedDoors
            | _ -> passedDoors

        [Up;Down;Left;Right]
        |> List.map (addMovement currentPosition)
        |> List.filter (fun p -> List.contains p visitedPoints |> not)
        |> List.filter (isNextSpaceOpen map)
        |> List.collect (walkRoute (stepCount + 1) (currentPosition::visitedPoints) passedDoors)
        |> List.append keyMatch

    let keys = map |> Map.toList |> List.filter (fun (_, p) -> IsKey p || IsEntrance p)
    let allKeys = keys |> List.map snd |> List.filter (IsKey)

    keys
    |> List.map (fun (coord, key) ->
        key, walkRoute 0 [] [] coord
            |> List.filter (fst >> (<>) key)
            |> (fun l ->
                allKeys
                |> List.filter ((<>) key)
                |> List.map (fun k ->
                    let routesForKey = l |> List.filter (fst >> (=) k)
                    if List.isEmpty routesForKey then None
                    else routesForKey |> List.minBy (fun (_, r) -> r.stepCount) |> Some
                )
                |> List.choose id
            )
            |> Map.ofList
    )
    |> Map.ofList

let findBestRoute (routes: KeyPointRoutes) =
    let allPoints = routes |> Map.toList |> List.map fst
    let allKeys = allPoints |> List.filter IsKey

    let rec findRoutes seenKeys (currentKeys: Map<int, KeyPoint>) (cache: Map<Map<int, KeyPoint> * KeyPoint list, KeyPoint list * int>) =
        let hasSeenKey key = List.contains key seenKeys
        let visited = seenKeys |> List.sort

        if cache.ContainsKey(currentKeys, visited)
        then cache,cache.[currentKeys, visited]
        else

        let remaining =
            allKeys
            |> List.filter (hasSeenKey >> not)

        if List.isEmpty remaining
        then cache,([], 0)
        else

        let cache, minResult =
            currentKeys
            |> Map.toList
            |> List.fold (fun (cache, prevResult) (index, currentKey) ->
                let accessibleKeys =
                    routes.[currentKey]
                    |> Map.toList
                    |> List.filter (fst >> hasSeenKey >> not)
                    |> List.filter (fun (_, r) -> List.forall hasSeenKey r.requiredKeys)

                if List.isEmpty accessibleKeys
                then cache, prevResult
                else
                accessibleKeys
                |> List.fold (fun (cache, (minRoute, minSteps)) (key,route) ->
                    let cache, (resRoute, resSteps) = findRoutes (key::seenKeys) (currentKeys.Add(index, key)) cache
                    let resRoute = key::resRoute
                    let resSteps = if resSteps = Int32.MaxValue then resSteps else resSteps + route.stepCount
                    if minSteps <= resSteps
                    then cache, (minRoute, minSteps)
                    else cache, (resRoute, resSteps)
                ) (cache, prevResult)
            ) (cache, ([], Int32.MaxValue))

        cache.Add((currentKeys, visited), minResult), minResult

    let startPoints =
        allPoints
        |> List.filter IsEntrance
        |> List.mapi (fun i e -> i,e)
        |> Map.ofList

    findRoutes [] startPoints Map.empty |> snd

let replaceStart (map: KeyPointMap) =
    let startEntrance = findStartPoints map |> List.head

    let startOpt =  match startEntrance with
                    | Entrance x -> Some x
                    | _ -> None

    if startOpt.IsNone then map
    else
    let start = startOpt.Value

    [
        (0,0)
        (0,1)
        (0,-1)
        (1,0)
        (-1,0)
    ]
    |> List.map (addVectors start)
    |> List.fold (fun (m: KeyPointMap) p -> m.Add(p, Wall))
        ([
            (1,1)
            (1,-1)
            (-1,1)
            (-1,-1)
        ]
        |> List.map (addVectors start)
        |> List.fold (fun m p -> m.Add(p, Entrance p)) map)

[<EntryPoint>]
let main argv =
    let input = IO.File.ReadAllLines "data.txt"
                |> Array.mapi (fun y r -> y,r)
                |> Array.fold addRowToMap Map.empty
    
    let allRoutes = generateRoutes input
    let foundRoutes = findBestRoute allRoutes
    printfn "Part 1: %A" foundRoutes

    let newRoutes = replaceStart input |> generateRoutes
    let foundRoutes = findBestRoute newRoutes
    printfn "Part 2: %A" foundRoutes
    0 // return an integer exit code
