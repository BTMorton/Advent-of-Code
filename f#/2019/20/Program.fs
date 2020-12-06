// Learn more about F# at http://fsharp.org

open System

type Portal = string * bool
type Point =
    | Wall
    | Space
    | Portal of Portal
type PointMap = Map<int*int, Point>
type RouteMap = Map<Portal, Map<Portal, int>>

type Coord = int * int

type QueuedStep = {
    currentPortal: Portal;
    steps: int;
    depth: int;
    visited: Map<int, Portal list>;
    portals: string list
}

type RouteStep = {
    position: Coord;
    steps: int;
    visited: Coord list;
}

let directions = [
    (0,1)
    (0,-1)
    (1,0)
    (-1,0)
]

let addVector (aX,aY) (bX,bY) =
    aX + bX, aY + bY

let IsPortal p =
    match p with
    | Portal _ -> true
    | _ -> false

let charToPoint char =
    match char with
    | '.' -> Space
    | x when Char.IsLetter x -> Portal (Char.ToString x, false)
    | _ -> Wall

let getNeighbouring predicate (map: Map<int * int, Point>) coords =
    directions
    |> List.map (addVector coords)
    |> List.filter map.ContainsKey
    |> List.filter ((fun p -> map.[p]) >> predicate)

let getNeighbouringSpaces = getNeighbouring ((=) Space)

let getPortalChar point =
    match point with
    | Portal (str, _) -> str
    | _ -> ""

let IsOutside map (inX, inY) (outX, outY) =
    let points = map |> Map.toList |> List.map fst
    let midX = points |> List.maxBy fst |> fst |> (/) <| 2
    let midY = points |> List.maxBy snd |> snd |> (/) <| 2

    if (inX - outX) <> 0
    then (inX > outX) <> (inX > midX)
    else (inY > outY) <> (inY > midY)

let parseInput input =
    let charMap =
        input
        |> Array.mapi (fun y row ->
            row
            |> Seq.mapi (fun x c -> (x,y), charToPoint c)
            |> Seq.toArray
        )
        |> Array.collect id
        |> Map.ofArray

    let rec combinePortals (map: Map<int*int, Point>) portalsToVisit =
        match portalsToVisit with
        | [] -> map
        | portal::rem ->
            match map.[portal] with
            | Portal (str, _) ->
                let neighbouringPortal = getNeighbouring IsPortal map portal |> List.head
                let portalName = [str; getPortalChar map.[neighbouringPortal]] |> List.sort |> (fun l -> String.Join("", l))
                let outside = IsOutside map portal neighbouringPortal
                let map = map.Add(portal, Portal (portalName, outside))
                let map = map.Add(neighbouringPortal, Wall)
                combinePortals map rem
            | _ -> combinePortals map rem

    let portals =
        charMap
        |> Map.toList
        |> List.filter (snd >> IsPortal)
        |> List.map fst
        |> List.filter (getNeighbouringSpaces charMap >> List.length >> (<) 0)

    combinePortals charMap portals

let rec doBFS isSolution getChildren queue =
    if List.isEmpty queue
    then None
    else
        let head = List.head queue
        if isSolution head
        then Some head
        else
            doBFS isSolution getChildren (List.append (List.tail queue) (getChildren head))

let rec iterBFS isSolution getChildren solutions queue =
    if List.isEmpty queue
    then solutions
    else
        let head = List.head queue
        let solutions =
            if isSolution head
            then head::solutions
            else solutions

        iterBFS isSolution getChildren solutions (List.append (List.tail queue) (getChildren head))

let findRoutesBetweenPortals (map: PointMap) =
    let portals = map |> Map.toList |> List.filter (snd >> IsPortal)

    let findMatchedPortals portalCoord =
        let isSolution item =
            match map.[item.position] with
            | Portal _ -> true
            | _ -> false

        let getChildren item =
            let visited = item.position::item.visited

            item.position
            |> getNeighbouring ((<>) Wall) map
            |> List.filter (fun p -> List.contains p visited |> not)
            |> List.map (fun p -> {
                position = p;
                visited = visited;
                steps = item.steps + 1;
            })

        iterBFS isSolution getChildren [] [{ position = portalCoord; visited = []; steps = 0; }]

    portals
    |> List.map (fun (coord, portal) ->
        match portal with
        | Portal (str, outside) ->
            let routeMap =
                findMatchedPortals coord
                |> List.filter (fun r -> r.position <> coord)
                |> List.map (fun r ->
                    match map.[r.position] with
                    | Portal (str, outside) -> Some ((str, outside), r.steps - 1)
                    | _ -> None
                )
                |> List.choose id
                |> Map.ofList
            Some ((str, outside), routeMap)
        | _ -> None
    )
    |> List.choose id
    |> Map.ofList

let getDepthChange outside = if outside then -1 else 1

let findBestRoute (map: RouteMap) partTwo =
    let solution (item: QueuedStep) =
        match item.currentPortal with
        | ("ZZ", _) -> not (partTwo && item.depth > 0)
        | _ -> false

    let getChildren (item: QueuedStep) =
        let updateVisited (visited: Map<int, Portal list>) depth portal =
            if visited.ContainsKey depth
            then visited.Add(depth, portal::visited.[depth])
            else visited.Add(depth, [portal])

        let visited = updateVisited item.visited item.depth item.currentPortal

        match item.currentPortal with
        //  If we get to the end or back to the start, stop
        | ("ZZ", _) -> []
        | ("AA", _) -> []
        | (str, outside) ->
            if partTwo && outside && item.depth <= 0
            then []
            else
            let portalExit = (str, not outside)
            let newDepth = item.depth + getDepthChange outside
            let visited = updateVisited visited newDepth portalExit
            let connected =
                //  Get the portals connected to the opposite of this one
                map.[portalExit]
                |> Map.toList
                |> List.filter (fst >> (fun p -> List.contains p visited.[newDepth] |> not))

            connected
            |> List.map (fun (portal, steps) -> {
                currentPortal = portal;
                portals = ((fst portal)::item.portals);
                steps = item.steps + steps;
                visited = visited;
                depth = newDepth;
            })

    let startingPortals =
        //  Get the portals connected to the starting portal
        map.[("AA", true)]
        |> Map.toList
        //  Only want inside portals
        |> List.filter (fst >> snd >> not)
        |> List.map (fun (portal, steps) -> {
            currentPortal = portal;
            portals = [fst portal];
            steps = steps;
            visited = Map.empty;
            depth = 0;
        })

    doBFS solution getChildren startingPortals
    |> (fun i ->
        if i.IsNone
        then -1, []
        else i.Value.steps - 1, i.Value.portals)

[<EntryPoint>]
let main argv =
    let input =
        IO.File.ReadAllLines "data.txt"
        |> parseInput

    let routes = (findRoutesBetweenPortals input)
    printfn "Part 1 %A" (findBestRoute routes false)
    printfn "Part 2 %A" (findBestRoute routes true)
    0 // return an integer exit code
