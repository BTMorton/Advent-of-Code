// Learn more about F# at http://fsharp.org

open System

let testData = seq {
    "COM)B";
    "B)C";
    "C)D";
    "D)E";
    "E)F";
    "B)G";
    "G)H";
    "D)I";
    "E)J";
    "J)K";
    "K)L";
    "I)SAN";
    "K)YOU";
}

let addOrbitToMap (map: Map<string, string>) (orbiting, object: string) = map.Add(object, orbiting)

let parseOrbit (orbit: string) =
    let split = orbit.Split [|')'|]
    split.[0], split.[1]

let generateTree (orbits: seq<string>) =
    orbits |> Seq.map parseOrbit
           |> Seq.fold addOrbitToMap (new Map<string, string>([]))

let rec getAllOrbits (tree: Map<string, string>) key =
    let orbiting = tree.TryFind key
    match orbiting with
    | None -> []
    | Some x -> x :: (getAllOrbits tree x)

[<EntryPoint>]
let main argv =
    let testData = IO.File.ReadAllLines "input_data.txt"
    let tree = generateTree testData
    let count = Map.toList tree |> List.sumBy (fun (k, _) -> (getAllOrbits tree k) |> List.length)
    printfn "Part 1: %d" count

    let youOrbits = getAllOrbits tree "YOU" |> Set.ofList
    let santaOrbits = getAllOrbits tree "SAN" |> Set.ofList
    let transfers = Set.union
                        (Set.difference youOrbits santaOrbits)
                        (Set.difference santaOrbits youOrbits)
                    |> Set.count
    printfn "Part 2: %d" transfers
    0 // return an integer exit code
