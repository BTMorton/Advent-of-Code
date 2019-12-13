// Learn more about F# at http://fsharp.org

open System
open System.Text.RegularExpressions

type Coords = {
    x: int;
    y: int;
    z: int;
}

type Moon = {
    Position: Coords;
    Velocity: Coords;
}

let moonRegex = Regex "<x=(?<x>-?[0-9]+), y=(?<y>-?[0-9]+), z=(?<z>-?[0-9]+)>"
let parseInput line =
    let m = moonRegex.Match(line)
    if not m.Success then None
    else
        Some {
            x = m.Groups.["x"].Value |> int;
            y = m.Groups.["y"].Value |> int;
            z = m.Groups.["z"].Value |> int;
        }

let getCoordEnergy coords =
    (abs coords.x) + (abs coords.y) + (abs coords.z)

let getMoonEnergy moon =
    (getCoordEnergy moon.Position) * (getCoordEnergy moon.Velocity);

let sumCoords a b =
    {
        x = a.x + b.x;
        y = a.y + b.y;
        z = a.z + b.z;
    }

let calculateNewVelocityPart velocityPart positionPart allPositionParts =
    allPositionParts |> List.fold (fun calcPart targetPart -> calcPart + sign (targetPart - positionPart)) velocityPart

let stepMoonParts parts =
    let positionParts = List.map fst parts
    parts
    |> List.map (fun p -> calculateNewVelocityPart (snd p) (fst p) positionParts)
    |> List.zip (parts |> List.map fst)
    |> List.map (fun p -> let v = snd p
                          (fst p) + v, v)

let calculateNewVelocity allMoons moon =
    {
        x = allMoons |> List.map (fun m -> m.Position.x) |> calculateNewVelocityPart moon.Velocity.x moon.Position.x
        y = allMoons |> List.map (fun m -> m.Position.y) |> calculateNewVelocityPart moon.Velocity.y moon.Position.y
        z = allMoons |> List.map (fun m -> m.Position.z) |> calculateNewVelocityPart moon.Velocity.z moon.Position.z
    }

let stepMoon allMoons moon =
    let newVelocity = calculateNewVelocity allMoons moon
    {
        Position = sumCoords newVelocity moon.Position;
        Velocity = newVelocity;
    }

let performTimeStep moons = moons |> List.map (stepMoon moons)

let performNSteps moons n =
    [1..n]
    |> List.fold (fun l _ -> performTimeStep l) moons

let findFirstRepetition getter moons =
    let initial = moons |> List.map getter
    moons
    |> Seq.unfold (fun m -> let newMoons = performTimeStep m
                            Some(newMoons, newMoons))
    |> Seq.takeWhile (fun p -> (p |> List.map getter) <> initial)
    |> Seq.length
    |> (+) 1

let rec gcd x y =
    if y = 0L
    then abs x
    else gcd y (x % y)

let lcm x y = x * y / (gcd x y)

let findReturn moons =
    let rx = moons |> findFirstRepetition (fun m -> m.Position.x, m.Velocity.x) |> int64
    let ry = moons |> findFirstRepetition (fun m -> m.Position.y, m.Velocity.y) |> int64
    let rz = moons |> findFirstRepetition (fun m -> m.Position.z, m.Velocity.z) |> int64
    printfn "%d %d %d" rx ry rz
    [rx;ry;rz] |> List.reduce lcm

[<EntryPoint>]
let main argv =
    let moons = IO.File.ReadAllLines "data.txt"
                |> Seq.toList
                |> List.map parseInput
                |> List.choose id
                |> List.mapi (fun _ p -> { Position = p; Velocity = {x=0;y=0;z=0} })

    let result = (performNSteps moons 1000)
    printfn "Part 1: %d" (result |> Seq.sumBy getMoonEnergy)

    printfn "Part 2: %d" (findReturn moons)
    0 // return an integer exit code
