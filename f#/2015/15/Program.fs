// Learn more about F# at http://fsharp.org

open System
open System.Text.RegularExpressions

type Properties = {
    Capacity: int;
    Durability: int;
    Flavor: int;
    Texture: int;
    Calories: int;
}

let ingredientRegex = Regex "(?<name>[A-Za-z]+): capacity (?<capacity>-?[0-9]+), durability (?<durability>-?[0-9]+), flavor (?<flavor>-?[0-9]+), texture (?<texture>-?[0-9]+), calories (?<calories>-?[0-9]+)"

let get (m: Match) (key: string) =
    m.Groups.[key].Value
let getInt (m: Match) (key: string) =
    get m key |> int

let parseInput (input: string) =
    printfn "%s" input
    let m = ingredientRegex.Match input
    if not m.Success then None
    else Some ((get m "name"), {
        Capacity = getInt m "capacity";
        Durability = getInt m "durability";
        Flavor = getInt m "flavor";
        Texture = getInt m "texture";
        Calories = getInt m "calories";
    })

let getKeys (map: Map<string, _>) =
    map
    |> Map.toSeq
    |> Seq.map fst

let addIngredient properties ingredient count =
    {
        Capacity = properties.Capacity + (ingredient.Capacity * count);
        Durability = properties.Durability + (ingredient.Durability * count);
        Flavor = properties.Flavor + (ingredient.Flavor * count);
        Texture = properties.Texture + (ingredient.Texture * count);
        Calories = properties.Calories + (ingredient.Calories * count);
    }
let rec calculateIngredientCombinations toUse (used: Map<string, int>) currentProperties =
    let remaining = 100 - (used |> Map.toSeq |> Seq.sumBy snd)

    match toUse with
    | [ingr] -> seq {(used.Add(fst ingr, remaining), addIngredient currentProperties (snd ingr) remaining)}
    | ingr::rest ->
            [0..remaining]
            |> Seq.collect (fun i ->
                let newProperties = (addIngredient currentProperties (snd ingr) i)
                calculateIngredientCombinations rest (used.Add(fst ingr, i)) newProperties
            )
    | _ -> seq {(used, currentProperties)}

[<EntryPoint>]
let main argv =
    let ingredients = IO.File.ReadAllLines "data.txt"
                        |> Seq.map parseInput
                        |> Seq.choose id
                        |> Seq.toList

    let allCombinations = calculateIngredientCombinations ingredients (new Map<string, int>([])) {
                                Capacity = 0;
                                Durability = 0;
                                Flavor = 0;
                                Texture = 0;
                                Calories = 0;
                            }

    let result = allCombinations
                    |> Seq.maxBy (snd
                                >> (fun p -> [p.Capacity; p.Durability; p.Flavor; p.Texture])
                                >> List.map (max 0)
                                >> List.reduce (*))
    printfn "Part 1: %A %d" result (result |> snd |> (fun p -> p.Capacity * p.Durability * p.Flavor * p.Texture))

    let result = allCombinations
                    |> Seq.filter (snd >> (fun p -> p.Calories) >> (=) 500)
                    |> Seq.maxBy (snd
                                >> (fun p -> [p.Capacity; p.Durability; p.Flavor; p.Texture])
                                >> List.map (max 0)
                                >> List.reduce (*))
    printfn "Part 2: %A %d" result (result |> snd |> (fun p -> p.Capacity * p.Durability * p.Flavor * p.Texture))
    // printfn "Hello World from F#!"
    0 // return an integer exit code
