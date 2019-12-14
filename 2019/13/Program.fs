// Learn more about F# at http://fsharp.org

open System
open IntCode.IntCode

type GameStep = {
    computer: Computer;
    currentScore: int64;
    blockCount: int;
    display: Map<int64*int64, int64>;
}

let addToMap (map: Map<int64 * int64, int64>) (tile: int64 list) =
    map.Add((tile.[0], tile.[1]), tile.[2])

let countBlocks (map: Map<int64 * int64, int64>) =
    map
    |> Map.toList
    |> List.countBy snd
    |> List.tryFind (fst >> (=) 2L)
    |> Option.defaultValue (2L, 0)
    |> snd

let getBall map =
    map
    |> Map.toList
    |> List.filter (snd >> (=) 4L)
    |> List.head
    |> fst

let getPaddle map =
    map
    |> Map.toList
    |> List.filter (snd >> (=) 3L)
    |> List.minBy snd
    |> fst

let getJoystickMovement (map: Map<int64*int64, int64>) =
    let ball = getBall map
    let paddle = getPaddle map

    sign ((fst ball) - (fst paddle)) |> int64
    // 0L

let getChar char =
    match char with
    | 1L -> "\u2588"
    | 2L -> "X"
    | 3L -> "="
    | 4L -> "O"
    | _ -> " "

let display map =
    let points = map |> Map.toList |> List.map fst
    let maxX = points |> List.maxBy fst |> fst
    let maxY = points |> List.maxBy snd |> snd

    [0L..maxY]
    |> List.map (fun y -> [0L..maxX]
                            |> List.map (fun x -> getChar (if map.ContainsKey((x,y)) then map.[(x,y)] else 0L))
                            |> (fun l -> String.Join("", l)))
    |> List.iter (printfn "%s")
    printfn "Score %d" (if map.ContainsKey((-1L,0L)) then map.[(-1L,0L)] else 0L)

let gameStep state =
    if state.computer.state = Halted || state.blockCount <= 0
    then None
    else
    let result = resumeComputer state.computer
    let map = result.output
                |> List.rev
                |> List.chunkBySize 3
                |> List.fold addToMap state.display
    let blockCount = countBlocks map
    let score = if map.ContainsKey((-1L,0L)) then map.[(-1L, 0L)] else 0L
    // display map
    let state = {
        computer = {
            result with
                output = [];
                input = [(getJoystickMovement map)];
        };
        currentScore = score;
        blockCount = blockCount;
        display = map;
    }

    Some (state,state)

let playGame game =
    Array.set game 0 2L
    let computer = {
        memory = game |> Array.mapi (fun i c -> (int64 i),c) |> Map.ofArray;
        curIndex = 0L;
        relativeBase = 0L;
        input = [];
        output = [];
        state = Running;
    }
    let state = {
        computer = computer;
        blockCount = -1;
        currentScore = 0L;
        display = Map.empty;
    }

    Seq.unfold gameStep state
    // |> Seq.takeWhile (fun s -> s.computer.state <> Halted && s.blockCount > 0)
    |> Seq.rev
    |> Seq.head
    |> (fun s -> display s.display |> (fun _ -> s))
    |> (fun s -> (printfn "State: %A; Block Count: %d" s.computer.state s.blockCount) |> (fun _ -> s))
    |> (fun s -> s.currentScore)

[<EntryPoint>]
let main argv =
    let opts = IO.File.ReadAllLines "data.txt"
                |> Array.head
                |> (fun s -> s.Split(","))
                |> Array.map int64

    let result = execute64 opts []
                 |> (fun c -> c.output)
                 |> List.rev
                 |> List.chunkBySize 3
                 |> List.fold addToMap Map.empty

    printfn "Part 1: %d" (countBlocks result)
    printfn "Part 2: %d" (playGame opts)
    0 // return an integer exit code
