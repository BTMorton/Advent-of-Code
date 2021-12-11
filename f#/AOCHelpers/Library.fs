namespace AOCHelpers

module AOC =
    
    type Point = int * int

    let addPoint ((x1, y1): Point) ((x2, y2): Point) =
        (x1 + x2, y1 + y2)

    let split (split: string) (string: string) =
        string.Split split
    
    let trim (string: string) =
        string.Trim()

    let parsePoint =
        split ","
        >> Seq.map int
        >> Seq.take 2
        >> (fun seq -> Point (Seq.item 0 seq, Seq.item 1 seq))

    let readIntLines =
        System.IO.File.ReadAllLines 
        >> Seq.map int

    let readCommaSepList =
        System.IO.File.ReadAllLines
        >> Seq.head
        >> split ","

    let readCommaSepIntList =
        readCommaSepList
        >> Seq.map int

    let createMap (input: seq<seq<'a>>) =
        let maxY = input |> Seq.length |> (+) -1
        let maxX = input |> Seq.head |> Seq.length |> (+) -1

        { 0 .. maxY }
        |> Seq.zip input
        |> Seq.collect (fun (row, y) -> { 0 .. maxX } |> Seq.zip row |> Seq.map (fun (point, x) -> (x, y, point)))
        |> Seq.fold (fun (map: Map<Point, 'a>) (x, y, point) -> map.Add((x, y), point)) (new Map<Point, 'a>([]))

    let readIntMap =
        System.IO.File.ReadAllLines
        >> Seq.map (Seq.map (fun c -> int c - int '0'))
        >> createMap

    let hvDirections = [
        (0,1)
        (0,-1)
        (1,0)
        (-1,0)
    ]

    let diagDirections =  [
        (1,1)
        (1,-1)
        (-1,1)
        (-1,-1)
    ]
    let allDirections = hvDirections @ diagDirections

    let opSpecifiedNeighbours (directions: List<Point>) (op: 'a -> 'a -> 'a) (inputMap: Map<Point, 'a>) point =
        directions
        |> Seq.fold
            (fun (map: Map<Point, 'a>) dir ->
                let target = addPoint point dir
                if map.ContainsKey target
                then map.Add(target, (op inputMap.[point] map.[target]))
                else map
            )
            inputMap

    let opHVNeighbours op map point =
        opSpecifiedNeighbours hvDirections op map point

    let opDiagNeighbours op map point =
        opSpecifiedNeighbours diagDirections op map point

    let opNeighbours op map point =
        opSpecifiedNeighbours allDirections op map point

    let opAllSpecifiedNeighbours (directions: List<Point>) (op: 'a -> 'a -> 'a) (inputMap: Map<Point, 'a>) =
        inputMap
        |> Map.fold (fun map point _ -> opSpecifiedNeighbours directions op map point) inputMap

    let opAllHVNeighbours op map =
        opAllSpecifiedNeighbours hvDirections op map

    let opAllDiagNeighbours op map =
        opAllSpecifiedNeighbours diagDirections op map

    let opAllNeighbours op map =
        opAllSpecifiedNeighbours allDirections op map

    let findSpecifiedNeighbours directions (map: Map<Point, 'a>) point =
        directions
        |> Seq.map (addPoint point)
        |> Seq.filter map.ContainsKey
        |> Seq.map (fun point -> (point, map.[point]))

    let findHVNeighbours map point =
        findSpecifiedNeighbours hvDirections map point

    let findDiagNeighbours map point =
        findSpecifiedNeighbours diagDirections map point

    let findNeighbours map point =
        findSpecifiedNeighbours allDirections map point


    let rec applySteps stepCount op input =
        match stepCount with
        | 0 -> input
        | _ -> applySteps (stepCount - 1) op (op input)