// Learn more about F# at http://fsharp.org

open System

let rec getFuelForMass moduleMass = (moduleMass / 3) - 2

let rec calculateRequiredFuel moduleMass =
    let fuelRequired = getFuelForMass moduleMass

    if fuelRequired < 0 then 0
    else fuelRequired + calculateRequiredFuel fuelRequired



[<EntryPoint>]
let main argv =
    let lines = IO.File.ReadAllLines "input_data.txt"
    let totalFuel = lines |> Seq.sumBy (int >> getFuelForMass)
    printfn "Part 1 Total Fuel: %d" totalFuel

    let totalFuel = lines |> Seq.sumBy (int >> calculateRequiredFuel)
    printfn "Part 2 Total Fuel: %d" totalFuel
    0 // return an integer exit code

