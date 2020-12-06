// Learn more about F# at http://fsharp.org

open System

type Direction =
    Up
    | Down
    | Left
    | Right

let convertToDirections c =
    match c with
    | '^' -> Some Up
    | '>' -> Some Right
    | '<' -> Some Left
    | 'v' -> Some Down
    | _ -> None

let filteredList list dir =
    match dir with
    | Some x -> x::list
    | None -> list

let directionsToCoords dir =
    match dir with
    | Up -> (0, 1)
    | Down -> (0, -1)
    | Right -> (1, 0)
    | Left -> (-1, 0)

let addCoords (x,y) (dirX,dirY) =
    (x + dirX), y + dirY

let mapSome compFn mapFn index value  =
    match index with
    | x when compFn x -> Some (mapFn value)
    | _ -> None

[<EntryPoint>]
let main argv =
    let input = IO.File.ReadAllLines "data.txt"
                |> Seq.head
                |> Seq.map convertToDirections
                |> Seq.fold filteredList []
                |> List.rev

    let houses = input
                 |> List.map directionsToCoords
                 |> List.scan addCoords (0,0)
                 |> List.countBy id
    printfn "Part 1 %d" houses.Length

    // let input = "^v^v^vv^v^v^"
    //             |> Seq.map convertToDirections
    //             |> Seq.fold filteredList []
    //             |> List.rev
    let santa = input
                |> List.map directionsToCoords
                |> List.mapi (fun i d -> i,d)
                |> List.filter (fun (i,_) -> i % 2 = 0)
                |> List.map snd
                |> List.scan addCoords (0,0)
                // |> List.rev
                // |> List.head
    let roboSanta = input
                    |> List.map directionsToCoords
                    |> List.mapi (fun i d -> i,d)
                    |> List.filter (fun (i,_) -> i % 2 <> 0)
                    |> List.map snd
                    |> List.scan addCoords (0,0)
                    // |> List.rev
                    // |> List.head
    let my = santa @ roboSanta
    printfn "Santa %A robo %A" (santa |> List.rev |> List.head) (roboSanta |> List.rev |> List.head)
    printfn "Part 2 %d" (my |> List.distinct |> List.length)
    // let visits = santa @ roboSanta
    //             |> List.distinct
    // printfn "Part 2 %d" visits.Length

    let alt = (input
                      |> List.map directionsToCoords
                      |> List.scan (fun (a,b) ch -> (addCoords b ch, a)) ((0,0),(0,0))
                      |> List.map fst)
    let santa::robo::tail = alt |> List.rev
    printfn "Santa %A robo %A" santa robo

    printfn "Alt %d" (alt |> List.distinct |> List.length)

    printfn "Diff %A" (Set.difference (Set.ofList alt) (Set.ofList my))
    0 // return an integer exit code
