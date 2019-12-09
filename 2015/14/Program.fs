// Learn more about F# at http://fsharp.org

open System
open System.Text.RegularExpressions

let reindeerRegex = Regex ("(?<name>[a-z]+) can fly (?<speed>[0-9]+) km/s for (?<fly>[0-9]+) seconds, but then must rest for (?<rest>[0-9]+) seconds.", RegexOptions.IgnoreCase)

type Reindeer = {
    name: string;
    speed: int;
    flyTime: int;
    restTime: int;
}

let parseInput (input: string) =
    let m = reindeerRegex.Match input
    {
        name = m.Groups.["name"].Value
        speed = m.Groups.["speed"].Value |> int
        flyTime = m.Groups.["fly"].Value |> int
        restTime = m.Groups.["rest"].Value |> int
    }

let calculateDistance time reindeer =
    let { speed = speed; flyTime = flyTime; restTime = restTime; name = name; } = reindeer
    let roundTripTime = flyTime + restTime
    let fullRuns = time / roundTripTime
    let remTime = time % roundTripTime

    let res = ((fullRuns * flyTime) + (min remTime flyTime)) * speed
    name, res

let calculatePoints endTime allReindeer =
    [1..endTime]
    |> Seq.collect (fun i ->
        let runs = allReindeer |> Seq.map (calculateDistance i)
        let highest = runs |> Seq.maxBy snd |> snd
        runs |> Seq.filter (snd >> (=) highest)
            |> Seq.map fst
    )
    |> Seq.countBy id

[<EntryPoint>]
let main argv =
    let reindeer = IO.File.ReadAllLines "data.txt"
                   |> Seq.map parseInput

    let part1 = reindeer
                |> Seq.map (calculateDistance 2503)
                |> Seq.maxBy snd
    printfn "Part 1 %A" part1
    let part2 = calculatePoints 2503 reindeer
                |> Seq.maxBy snd
    printfn "Part 2 %A" part2

    0 // return an integer exit code