let addPresentsToHouses max (houses: Map<int, int>) step =
    [step..step..max]
    |> List.fold (fun (h: Map<int, int>) i ->
        let newCount = h.[i] + (step * 10);
        h.Add(i, newCount)
    ) houses

let addPresentsTo50Houses max (houses: Map<int, int>) step =
    let upper = min max (step * 50)
    [step..step..upper]
    |> List.fold (fun (h: Map<int, int>) i ->
        let newCount = h.[i] + (step * 11);
        h.Add(i, newCount)
    ) houses

let findFirstHouseToGet presentCount presentOp =
    let iter = presentCount / 2

    let houses = [1..presentCount]
                 |> List.map (fun i -> i,0)
                 |> Map.ofList

    [1..iter]
    |> List.fold (presentOp iter) houses
    |> Map.toList
    |> List.filter (snd >> (<=) presentCount)
    |> List.minBy fst
    |> fst

[<EntryPoint>]
let main argv =
    let input = 29000000
    printfn "Part 1: %d" (findFirstHouseToGet input addPresentsToHouses)
    printfn "Part 2: %d" (findFirstHouseToGet input addPresentsTo50Houses)
    0 // return an integer exit code
