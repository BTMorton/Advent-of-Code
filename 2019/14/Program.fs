// Learn more about F# at http://fsharp.org

open System

type Element = int64 * string
type ElementMap = Map<string, int64 * Element array>

let parseElement (element: string) =
    let [| count; material |] = element.Split(" ")
    (int64 count), material

let parseInput (map: ElementMap) (line: string) =
    let parts = line.Split("=>") |> Array.map (fun s -> s.Trim())
    let requirements = parts.[0].Split(",") |> Array.map ((fun s -> s.Trim()) >> parseElement)
    let count, material = parseElement parts.[1]
    map.Add(material, (count, requirements))

let getMaterialRequiredFor (elements: ElementMap) material count =
    let generated, requirements = elements.[material]
    let difference = count % generated
    let multiplier = (count / generated) + (sign difference |> int64)

    requirements
    |> Array.map (fun (c, mat) -> c * multiplier, mat)
    |> (fun reqs -> (generated * multiplier), reqs)


let rec getRequiredOreFor (elements: ElementMap) (oreCount, leftovers: Map<string, int64>) (count,element) =
    if element = "ORE" then (oreCount + count), leftovers
    else
        let leftover = if leftovers.ContainsKey(element) then leftovers.[element] else 0L
        if leftover >= count
        then oreCount, leftovers.Add(element, (leftover - count))
        else
            let generated, requirements = getMaterialRequiredFor elements element (count - leftover)

            requirements
            |> Array.fold (getRequiredOreFor elements) (oreCount, (leftovers.Add(element, (generated + leftover - count))))

let getRequiredOre (elements: ElementMap) =
    getRequiredOreFor elements (0L, Map.empty) (1L,"FUEL")
    |> fst


let getTotalFuelForOre (elements: ElementMap) totalOre =
    let (oneFuelOre,_) = getRequiredOreFor elements (0L, Map.empty) (1L,"FUEL")

    let rec iter upper lower =
        let midPoint = ((upper - lower) / 2L) + lower
        if upper = midPoint || lower = midPoint
        then midPoint
        else
            let (usedOre,_) = getRequiredOreFor elements (0L, Map.empty) (midPoint,"FUEL")
            if usedOre = totalOre
            then midPoint
            else if usedOre > totalOre
            then iter midPoint lower
            else iter upper midPoint

    iter (2L * totalOre / oneFuelOre) (totalOre / oneFuelOre)

[<EntryPoint>]
let main argv =
    let elements =  IO.File.ReadAllLines "data.txt"
                    |> Array.fold parseInput Map.empty

    printfn "Part 1: %d" (getRequiredOre elements)
    printfn "Part 2: %A" (getTotalFuelForOre elements 1000000000000L)
    0 // return an integer exit code