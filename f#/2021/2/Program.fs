// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System

let inputToVector  [| direction; movement |] =
    let intMovement = int movement

    match direction with
    | "forward" -> ((int movement), 0)
    | "down" -> (0, (int movement))
    | "up" -> (0, -(int movement))
    | _ -> (0,  0)

let sumVector (a1, b1) (a2, b2) =
    (a1 + a2, b1 + b2)

let part1 =
    Seq.map (inputToVector << fun (s : string) -> s.Split ' ')
    >> Seq.fold sumVector (0, 0)
    >> fun (h, v) -> h * v

let part2Step (h, v, aim) [| direction; movement |] =
    let intMovement = int movement
    let movementMultiplied = aim * intMovement

    match direction with
    | "forward" -> (h + intMovement, v + movementMultiplied, aim)
    | "down" -> (h, v, aim + intMovement)
    | "up" -> (h, v, aim - intMovement)
    | _ -> (h, v, aim)

let part2 =
    Seq.map (fun (s : string) -> s.Split ' ')
    >> Seq.fold part2Step (0, 0, 0)
    >> fun (h, v, _) -> h * v

[<EntryPoint>]
let main argv =
    let inputs = IO.File.ReadAllLines "data.txt"

    printfn "Part 1: %d" (part1 inputs)
    printfn "Part 2: %d" (part2 inputs)
    0 // return an integer exit code