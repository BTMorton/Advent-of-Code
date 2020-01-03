// Learn more about F# at http://fsharp.org

open System
open IntCode.IntCode

type Point = int * int
type RobotDirection =
    Up | Down | Left | Right
type CameraPoint =
    Scaffold
    | Space
    | NewLine
    | Robot of RobotDirection
type CameraMap = Map<Point, CameraPoint>

let getMovement direction =
    match direction with
    | Up -> (0,-1)
    | Down -> (0,1)
    | Left -> (-1,0)
    | Right -> (1,0)

let getDirectionLeft direction =
    match direction with
    | Up -> Left
    | Down -> Right
    | Left -> Down
    | Right -> Up

let getDirectionRight direction =
    match direction with
    | Up -> Right
    | Down -> Left
    | Left -> Up
    | Right -> Down

let getPointFromChar chr =
    match chr with
    | '#' -> Scaffold
    | '<' -> Robot Left
    | '>' -> Robot Right
    | '^' -> Robot Up
    | 'v' -> Robot Down
    | '\n' -> NewLine
    | _ -> Space

let getCharFromPoint point =
    match point with
    | Scaffold -> '#'
    | Robot Left -> '<'
    | Robot Right -> '>'
    | Robot Up -> '^'
    | Robot Down -> 'v'
    | NewLine -> '\n'
    | Space -> ' '

let addPoint (aX,aY) (bX,bY) = (aX+bX),(aY+bY)

let isIntersection (map: CameraMap) point =
    if not (map.ContainsKey point) then false
    else if map.[point] = Space then false
    else
    [Up;Down;Left;Right]
    |> List.map (getMovement >> addPoint point)
    |> List.forall (fun p -> map.ContainsKey(p) && map.[p] <> Space)

let createMap opts =
    let result = execute64 opts []
    result.output
    |> List.rev
    |> List.map (char >> getPointFromChar)
    |> List.scan (fun ((x, y), _) p -> if p = NewLine
                                       then ((-1, y + 1), p)
                                       else ((x + 1, y), p)) ((-1, 0),NewLine)
    |> List.filter (fun (_, p) -> p <> NewLine)
    |> Map.ofList

let printMap map =
    let points = map |> Map.toList |> List.map fst
    let maxX = points |> List.maxBy fst |> fst
    let maxY = points |> List.maxBy snd |> snd

    for y in [0..maxY] do
        for x in [0..maxX] do
            if not (map.ContainsKey(x,y))
            then printf " "
            else if (isIntersection map (x,y))
            then printf "O"
            else printf "%c" (getCharFromPoint map.[(x,y)])
        printfn ""

let sumPoint (x,y) = x * y

let ToRobot (coord,point) =
    match point with
    | Robot x -> Some (coord,x)
    | _ -> None

let traverseMap (map: CameraMap) =
    let getPoint point =
        if map.ContainsKey point
        then map.[point]
        else Space

    let getNextDirection curPos curDirection =
        let nextDir = getDirectionLeft curDirection
        let nextPos = addPoint curPos (getMovement nextDir)
        if getPoint nextPos <> Space
        then Some(nextDir, "L")
        else
        let nextDir = getDirectionRight curDirection
        let nextPos = addPoint curPos (getMovement nextDir)
        if getPoint nextPos <> Space
        then Some(nextDir, "R")
        else None

    let rec traverse curPos curDirection curSteps actions =
        let nextPos = addPoint curPos (getMovement curDirection)
        if getPoint nextPos <> Space
        then traverse nextPos curDirection (curSteps + 1) actions
        else
        let actions = if curSteps > 0 then (string curSteps)::actions else actions
        let next = getNextDirection curPos curDirection
        if next.IsNone
        then actions
        else
        let (nextDir, action) = next.Value
        let nextPos = addPoint curPos (getMovement nextDir)
        traverse nextPos nextDir 1 (action::actions)

    let current = map |> Map.toList |> List.choose ToRobot |> List.head
    traverse (fst current) (snd current) 0 []

let splitPath path =
    let rec split (separator: string list) currentParts (remainingParts: string list) =
        let len = separator.Length

        match remainingParts with
        | [] -> [currentParts |> List.rev]
        | x when x.Length <= len -> [currentParts |> List.rev]
        | x when separator = (x |> List.take len) ->
            (currentParts |> List.rev)::separator::(split separator [] (x |> List.skip len))
        | first::remain ->
            split separator (first::currentParts) remain

    let countInstruction (parts: string list) = String.Join(",", parts).Length

    let alreadyMatched curMatches part =
        curMatches |> List.exists ((=) part)

    let rec iter (currentMatches: string list list) (pathParts: string list list) =
        let processMatch (curMatch: string list) (currentMatches2: string list list) (pathParts2: string list list) =
            let newPathParts =
                pathParts2
                |> List.collect (fun p -> if alreadyMatched currentMatches p then [p] else split curMatch [] p)
                |> List.filter (List.length >> (<) 0)
            let remainingParts = newPathParts |> List.filter (alreadyMatched currentMatches2 >> not)
            let newMatches = curMatch::currentMatches2

            //  Can only set three instructions and use them 9 times ((n * 2) + 1 < 20)
            if newMatches.Length > 3 || newPathParts.Length > 9
            then []
            else if remainingParts.Length = 0
            then [newMatches]
            else
            iter newMatches newPathParts

        match pathParts with
        | [] -> [currentMatches]
        | current::rem when alreadyMatched currentMatches current -> iter currentMatches rem
        | current::rem ->
            [2..current.Length]
            |> Seq.map (fun i -> List.splitAt i current)
            |> Seq.takeWhile (fun (l,_) -> countInstruction l < 20)
            |> Seq.collect (fun (l,r) ->
                let remain = if r.Length = 0 then rem else r::rem
                processMatch l currentMatches remain
            )
            |> Seq.toList

    iter [] [path]

let getChar index =
    match index with
    | 0 -> "A"
    | 1 -> "B"
    | 2 -> "C"
    | _ -> "_"

let replaceMatches (matches: string list) command =
    matches
    |> List.mapi (fun i m -> (getChar i), m)
    |> List.fold (fun (s: string) (c,m) -> s.Replace(m,c)) command

[<EntryPoint>]
let main argv =
    let opts = loadFile "data.txt"

    let map = createMap opts
    printMap map
    let intersections = map |> Map.toList |> List.filter (fst >> isIntersection map)
    printfn "Part 1: %d" (intersections |> List.sumBy (fst >> sumPoint))

    let path = traverseMap map |> List.rev
    let result = splitPath path

    if result.IsEmpty
    then printfn "Part 2 failed"
    else
        let matches = result.Head |> List.rev |> List.map (fun l -> String.Join(",", l))
        let fullCommand = String.Join(",", path) |> replaceMatches matches
        let input = fullCommand::matches |> List.map (fun s -> s + "\n") |> List.collect (Seq.map int64 >> Seq.toList)

        Array.set opts 0 2L
        let result = execute64 opts (List.concat [input; ['n'; '\n'] |> List.map int64])

        printfn "Part 2: %d" (result.output |> List.head)
    0 // return an integer exit code
