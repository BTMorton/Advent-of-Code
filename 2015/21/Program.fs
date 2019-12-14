// Learn more about F# at http://fsharp.org

open System

let weapons = [
    (8, 4, 0);
    (10, 5, 0);
    (25, 6, 0);
    (40, 7, 0);
    (74, 8, 0);
]
let armor = [
    (0, 0, 0);
    (13, 0, 1);
    (31, 0, 2);
    (53, 0, 3);
    (75, 0, 4);
    (102, 0, 5);
]
let rings = [
    (0, 0, 0);
    (0, 0, 0);
    (25, 1, 0)
    (50, 2, 0)
    (100, 3, 0)
    (20, 0, 1)
    (40, 0, 2)
    (80, 0, 3)
]

let getRingCombinations =
    rings
    |> List.collect (fun r1 -> rings |> List.map (fun r2 -> [r1; r2]))

let getArmorCombinations =
    armor
    |> List.collect (fun a -> getRingCombinations |> List.map (fun c -> a::c))

let getWeaponCombinations =
    weapons
    |> List.collect (fun w -> getArmorCombinations |> List.map (fun c -> w::c))

let sumItems (c1,d1,a1) (c2,d2,a2) =
    (c1 + c2, d1 + d2, a1 + a2)

let rec doBattle (health1, damage1, armor1) (health2, damage2, armor2) =
    if health1 <= 0
    then false
    else
        doBattle (health2 - (max 1 (damage1 - armor2)), damage2, armor2) (health1, damage1, armor1) |> not


[<EntryPoint>]
let main argv =
    let allCombinations = getWeaponCombinations
                            |> List.map (List.reduce sumItems)

    let part1 = allCombinations
                |> List.filter (fun (_,d,a) -> doBattle (100,d,a) (104, 8, 1) |> not)
                |> List.maxBy (fun (c,_,_) -> c)
    printfn "Part 1 %A" part1
    0 // return an integer exit code
