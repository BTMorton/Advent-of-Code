// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System
open AOCHelpers.AOC

type CacheMap = Map<string * int, Map<string, int64>>

let rec findInCache (insertions: Map<string, string>) depth (cacheMap: CacheMap) (pair, insert) =
    if cacheMap.ContainsKey((pair, depth))
    then cacheMap.[(pair, depth)]
    else if depth <= 0
    then new Map<string, int64>([])
    else
        let fPair = sprintf "%c%s" pair.[0] insert
        let sPair = sprintf "%s%c" insert pair.[1]

        let first =
            if insertions.ContainsKey fPair
            then findInCache insertions (depth - 1) cacheMap (fPair, insertions.[fPair])
            else new Map<string, int64>([])
        let second = 
            if insertions.ContainsKey sPair
            then findInCache insertions (depth - 1) cacheMap (sPair, insertions.[sPair])
            else new Map<string, int64>([])

        [(insert, 1L)]
        |> Seq.append (first |> Map.toSeq)
        |> Seq.append (second |> Map.toSeq)
        |> Seq.groupBy fst
        |> Seq.map (fun (insert, seq) -> (insert, seq |> Seq.sumBy snd))
        |> Map.ofSeq

let buildCache (insertions: Map<string, string>) oldCacheMap curDepth =
    insertions
    |> Map.toSeq
    |> Seq.fold (
        fun (cacheMap: CacheMap) (pair, insert) ->
            cacheMap.Add(
                (pair, curDepth),
                findInCache insertions curDepth cacheMap (pair, insert)
            )
        ) oldCacheMap

let buildCacheLevel insertions targetLevel =
    [1..targetLevel]
    |> Seq.fold (buildCache insertions) (new CacheMap([]))

let applyInsertions (template: string) depth (cacheMap: CacheMap) =
    template
    |> Seq.map (sprintf "%c")
    |> Seq.countBy id
    |> Seq.map (fun (k, n) -> (k, int64 n))
    |> Seq.append (template
        |> Seq.pairwise
        |> Seq.map String.Concat
        |> Seq.map (fun pair -> cacheMap.[(pair, depth)])
        |> Seq.collect Map.toSeq
    )
    |> Seq.groupBy fst
    |> Seq.map (fun (insert, seq) -> (insert, seq |> Seq.sumBy snd))
    |> Seq.map snd

let countPolymer depth template insertions =
    let counts = buildCacheLevel insertions depth
                 |> applyInsertions template depth
    
    Seq.max counts - Seq.min counts

let part1 = 
    countPolymer 10

let part2 = 
    countPolymer 40

[<EntryPoint>]
let main argv =
    let input = IO.File.ReadAllLines "real_data.txt"

    let template = input |> Seq.head
    let insertions = input
                     |> Seq.skip 1
                     |> Seq.filter ((<>) "")
                     |> Seq.map (split " -> " >> seqTo2Tuple)
                     |> Map.ofSeq

    printfn "Part 1: %d" (part1 template insertions)
    printfn "Part 2: %d" (part2 template insertions)

    0 // return an integer exit code