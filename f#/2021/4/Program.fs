// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System

let seqHasBingo (picked: Set<int>) =
    Seq.exists (picked.Contains >> not)
    >> not

let pickColumn card column =
    card
    |> Seq.map (Seq.item column)

let doesCardHaveBingo picked (card: list<list<int>>) = 
    let rowHasBingo = card |> Seq.exists (seqHasBingo picked)
    let colHasBingo = seq { 0 .. 4 } |> Seq.map (pickColumn card) |> Seq.exists (seqHasBingo picked)

    rowHasBingo || colHasBingo

let scoreWinningCard (picked: Set<int>) =
    Seq.collect (Seq.filter (picked.Contains >> not))
    >> Seq.sum

let rec findBestCard numbers cards position =
    let picked = numbers |> Seq.take position |> Set.ofSeq
    let winningCard = cards |> Seq.tryFind (doesCardHaveBingo picked)

    match winningCard with
    | Some card ->
        let cardScore = scoreWinningCard picked card
        let latestNumber = Seq.item (position - 1) numbers
        cardScore * latestNumber
    | None -> findBestCard numbers cards (position + 1)


let rec findWorstCard numbers cards position =
    let picked = numbers |> Seq.take position |> Set.ofSeq
    let remainingCards = cards |> Seq.filter (doesCardHaveBingo picked >> not) |> Seq.toList

    match remainingCards with
    | [] ->
        let latestNumber = Seq.item (position - 1) numbers
        cards
        |> Seq.head
        |> scoreWinningCard picked
        |> (*) latestNumber
    | _ -> findWorstCard numbers remainingCards (position + 1)

let part1 numbers cards =
    findBestCard numbers cards 1

let part2 numbers cards =
    findWorstCard numbers cards 1

[<EntryPoint>]
let main argv =
    // let lines = IO.File.ReadAllLines "test_data.txt"
    let lines = IO.File.ReadAllLines "real_data.txt"
    let numbers = lines
                |> Seq.head
                |> (fun str -> str.Split [| ',' |])
                |> Seq.map int
                |> Seq.toList
    let cards = lines
                |> Seq.skip 2
                |> Seq.chunkBySize 6
                |> Seq.map (
                    Seq.take 5
                    >> Seq.map (
                        fun str -> str.Split [| ' ' |]
                        >> Seq.filter ((=) "" >> not)
                        >> Seq.map int
                        >> Seq.toList
                    )
                    >> Seq.toList
                )
                |> Seq.toList

    printfn "Part 1: %d" (part1 numbers cards)
    printfn "Part 2: %d" (part2 numbers cards)
    0 // return an integer exit code