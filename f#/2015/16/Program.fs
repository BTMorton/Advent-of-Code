// Learn more about F# at http://fsharp.org

open System
open System.Text.RegularExpressions

type Sue = {
    Sue: int option;
    Children: int option;
    Cats: int option;
    Samoyeds: int option;
    Pomeranians: int option;
    Akitas: int option;
    Vizslas: int option;
    Goldfish: int option;
    Trees: int option;
    Cars: int option;
    Perfumes: int option;
}

let sueRegex = Regex "Sue (?<val>[0-9]+)"
let childrenRegex = Regex "children: (?<val>[0-9]+)"
let catsRegex = Regex "cats: (?<val>[0-9]+)"
let samoyedsRegex = Regex "samoyeds: (?<val>[0-9]+)"
let pomeraniansRegex = Regex "pomeranians: (?<val>[0-9]+)"
let akitasRegex = Regex "akitas: (?<val>[0-9]+)"
let vizslasRegex = Regex "vizslas: (?<val>[0-9]+)"
let goldfishRegex = Regex "goldfish: (?<val>[0-9]+)"
let treesRegex = Regex "trees: (?<val>[0-9]+)"
let carsRegex = Regex "cars: (?<val>[0-9]+)"
let perfumesRegex = Regex "perfumes: (?<val>[0-9]+)"

let getValue input (regex: Regex) =
    let m = regex.Match input
    if m.Success
    then m.Groups.["val"].Value |> int |> Some
    else None

let parseInput input =
    {
        Sue = getValue input sueRegex;
        Children = getValue input childrenRegex;
        Cats = getValue input catsRegex;
        Samoyeds = getValue input samoyedsRegex;
        Pomeranians = getValue input pomeraniansRegex;
        Akitas = getValue input akitasRegex;
        Vizslas = getValue input vizslasRegex;
        Goldfish = getValue input goldfishRegex;
        Trees = getValue input treesRegex;
        Cars = getValue input carsRegex;
        Perfumes = getValue input perfumesRegex;
    }

let matchSue (toMatch: string list) (input: string) =
    toMatch
    |> List.filter (fun m -> input.Contains(m))
    |> List.length
    |> (<=) 3

[<EntryPoint>]
let main argv =
    let input = IO.File.ReadAllLines "data.txt"
    let matches = [
        "children: 3";
        "cats: 7";
        "samoyeds: 2";
        "pomeranians: 3";
        "akitas: 0";
        "vizslas: 0";
        "goldfish: 5";
        "trees: 3";
        "cars: 2";
        "perfumes: 1";
    ]

    let matched = input
                    |> Seq.filter (matchSue matches)

    printfn "Part 1: %A" matched

    let sues = input |> Seq.map parseInput
    let test = [
        (fun sue -> sue.Children.IsNone || sue.Children.Value = 3)
        (fun sue -> sue.Cats.IsNone || sue.Cats.Value > 7)
        (fun sue -> sue.Samoyeds.IsNone || sue.Samoyeds.Value = 2)
        (fun sue -> sue.Pomeranians.IsNone || sue.Pomeranians.Value < 3)
        (fun sue -> sue.Akitas.IsNone || sue.Akitas.Value = 0)
        (fun sue -> sue.Vizslas.IsNone || sue.Vizslas.Value = 0)
        (fun sue -> sue.Goldfish.IsNone || sue.Goldfish.Value < 5)
        (fun sue -> sue.Trees.IsNone || sue.Trees.Value > 3)
        (fun sue -> sue.Cars.IsNone || sue.Cars.Value = 2)
        (fun sue -> sue.Perfumes.IsNone || sue.Perfumes.Value = 1)
    ]

    let part2 = sues
                |> Seq.filter (fun sue -> test |> List.fold (fun b t -> b && t sue) true)
    printfn "Part 2: %A" part2
    printfn "Hello World from F#!"
    0 // return an integer exit code
