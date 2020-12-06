// Learn more about F# at http://fsharp.org

open System

let isAsteroid = (=) '#'
let loadAsteroidMapRow y line =
    line
    |> Seq.mapi (fun x c -> (x,y),(isAsteroid c))

let loadAsteroidMap lines =
    lines
    |> Seq.mapi loadAsteroidMapRow
    |> Seq.collect id
    |> Map.ofSeq

let rec gcd a b =
  if b = 0
    then abs a
  else gcd b (a % b)

let findVisibleAsteroidsForPoint (map: Map<int*int, bool>) angles point =
    let findFirst (posX, posY) (xInc, yInc) =
        let rec iter x y =
            if not (map.ContainsKey (x,y))
            then None
            else
                if map.[(x,y)]
                then Some (x,y)
                else iter (x + xInc) (y + yInc)

        iter (posX + xInc) (posY + yInc)

    angles
    |> Seq.map (findFirst point)
    |> Seq.choose id

let getAngles (maxX, maxY) =
    let getSmallestEquiv (x,y) =
        let div = gcd (abs x) (abs y)
        if div = 0
        then x,y
        else x/div,y/div

    let getAngle (x,y) = atan2 (float x) (float y)

    seq {
        for x in -maxX..maxX do
            for y in -maxY..maxY do
                yield (x,y)
    }
    |> Seq.map getSmallestEquiv
    |> Seq.filter ((<>) (0,0))
    |> Seq.distinct
    |> Seq.sortBy getAngle
    |> Seq.rev

let getMapMax map =
    let points = map |> Map.toSeq |> Seq.map fst
    let maxX = points |> Seq.maxBy fst |> fst
    let maxY = points |> Seq.maxBy snd |> snd
    maxX, maxY

let findVisibleAsteroids (map: Map<int*int, bool>) =
    let maxes = getMapMax map
    let asteroids = map |> Map.toSeq |> Seq.filter snd |> Seq.map fst

    let angles = getAngles maxes
    asteroids
    |> Seq.map (fun p -> p,findVisibleAsteroidsForPoint map angles p)
    |> Seq.distinct

let countVisibleAsteroids (map: Map<int*int, bool>) =
    findVisibleAsteroids map
    |> Seq.map (fun (p,l) -> p,Seq.length l)

let shootAsteroids (map: Map<int*int, bool>) point =
    let countAsteroids map = map |> Map.toSeq |> Seq.filter snd |> Seq.length
    let angles = getAngles (getMapMax map)

    let rec iter shots map =
        if countAsteroids map <= 1
        then shots
        else
            let shot =  findVisibleAsteroidsForPoint map angles point
            iter (Seq.append shots shot) (shot |> Seq.fold (fun m p -> m.Add(p, false)) map)

    iter Seq.empty map

[<EntryPoint>]
let main argv =
    let asteroidMap = IO.File.ReadAllLines "data.txt"
                        |> loadAsteroidMap

    let mostVisibleAsteroids = countVisibleAsteroids asteroidMap
                                |> Seq.maxBy snd
    printfn "Part 1 %A" mostVisibleAsteroids

    let shootPoint = fst mostVisibleAsteroids
    let shotAsteroids = shootAsteroids asteroidMap shootPoint
    printfn "Part 2 %A" (shotAsteroids |> Seq.item 199 |> (fun (x,y) -> (x * 100) + y))
    0 // return an integer exit code
