// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System
open AOCHelpers.AOC

type Snailfish = Both of Snailfish * Snailfish | Left of Snailfish * int | Right of int * Snailfish | Base of int * int
type ExplodeResult = Exploded of Option<Snailfish> * int * int | NotExploded
type SplitResult = Split of Snailfish | NotSplit

let rec formatFish fish = 
    match fish with
    | Both(a, b) -> sprintf "[%s,%s]" (formatFish a) (formatFish b)
    | Left(a, b) -> sprintf "[%s,%d]" (formatFish a) b
    | Right(a, b)-> sprintf "[%d,%s]" a (formatFish b)
    | Base(a, b)  -> sprintf "[%d,%d]" a b

let rec magnitude fish =
    match fish with
    | Both(a, b) -> (3 * (magnitude a)) + (2 * (magnitude b))
    | Left(a, b) -> (3 * (magnitude a)) + (2 * b)
    | Right(a, b)-> (3 * a) + (2 * (magnitude b))
    | Base(a, b) -> (3 * a) + (2 * b)

let rec applyRight fish n =
    match fish with
    | Base(a, b) -> Base(a + n, b)
    | Right(a, b) -> Right(a + n, b)
    | Left(a, b) -> Left(applyRight a n, b)
    | Both(a, b) -> Both(applyRight a n, b)

let rec applyLeft fish n =
    match fish with
    | Base(a, b) -> Base(a, b + n)
    | Left(a, b) -> Left(a, b + n)
    | Right(a, b) -> Right(a, applyLeft b n)
    | Both(a, b) -> Both(a, applyLeft b n)

let rec explode fish depth =
    match fish with
    | Both(a, b) -> match explode a (depth + 1) with
                     | Exploded(None, x, y) -> Exploded(Some(Right(0, applyRight b y)), x, 0)
                     | Exploded(Some(f), x, 0) -> Exploded(Some(Both(f, b)), x, 0)
                     | Exploded(Some(f), x, y) -> Exploded(Some(Both(f, applyRight b y)), x, 0)
                     | NotExploded ->
                        match explode b (depth + 1) with
                        | Exploded(None, x, y) -> Exploded(Some(Left(applyLeft a x, 0)), 0, y)
                        | Exploded(Some(f), 0, y) -> Exploded(Some(Both(a, f)), 0, y)
                        | Exploded(Some(f), x, y) -> Exploded(Some(Both(applyLeft a x, f)), 0, y)
                        | NotExploded -> NotExploded
    | Left(a, b) -> match explode a (depth + 1) with
                     | Exploded(None, x, y) -> Exploded(Some(Base(0, b + y)), x, 0)
                     | Exploded(Some(f), x, 0) -> Exploded(Some(Left(f, b)), x, 0)
                     | Exploded(Some(f), x, y) -> Exploded(Some(Left(f, b + y)), x, 0)
                     | NotExploded -> NotExploded
    | Right(a, b) -> match explode b (depth + 1) with
                     | Exploded(None, x, y) -> Exploded(Some(Base(a + x, 0)), 0, y)
                     | Exploded(Some(f), 0, y) -> Exploded(Some(Right(a, f)), 0, y)
                     | Exploded(Some(f), x, y) -> Exploded(Some(Right(a + x, f)), 0, y)
                     | NotExploded -> NotExploded
    | Base(a, b) -> if depth > 4 then Exploded(None, a, b) else NotExploded

let splitN n =
    let half = n / 2
    Base(half, n - half)

let rec split fish =
    match fish with
    | Both(a, b) -> match split a with
                    | Split(fa) -> Split(Both(fa, b))
                    | NotSplit -> match split b with
                                     | Split(fb) -> Split(Both(a, fb))
                                     | NotSplit -> NotSplit
    | Left(a, b) -> match split a with
                     | Split(fa) -> Split(Left(fa, b))
                     | NotSplit when b >= 10 -> Split(Both(a, splitN b))
                     | NotSplit -> NotSplit
    | Right(a, b) when a >= 10 -> Split(Both(splitN a, b))
    | Right(a, b) -> match split b with
                     | Split(fb) -> Split(Right(a, fb))
                     | NotSplit -> NotSplit
    | Base(a, b) when a >= 10 -> Split(Left(splitN a, b))
    | Base(a, b) when b >= 10 -> Split(Right(a, splitN b))
    | Base(a, b) -> NotSplit

let rec action fish =
    match explode fish 1 with
    | Exploded(None, _, _) -> failwith "Unable to explode fish"
    | Exploded(Some(newFish), _, _) -> action newFish
    | NotExploded -> match split fish with
                            | Split(newFish) -> action newFish
                            | NotSplit -> fish

let addFish a b =
    action (Both(a, b))

let rec parseFish index (line: string) =
    if line.[index + 1] = '['
    then
        let (a, index) = parseFish (index + 1) line
        if line.[index + 1] = '['
        then
            let (b, index) = parseFish (index + 1) line
            (Both(a, b), index + 1)
        else
            (Left(a, charToInt (line.[index + 1])), index + 3)
    else
        let a = charToInt (line.[index + 1])
        if line.[index + 3] = '['
        then
            let (b, index) = parseFish (index + 3) line
            (Right(a, b), index + 1)
        else
            (Base(a, charToInt (line.[index + 3])), index + 5)

let part1 (fish: list<Snailfish>) = 
    fish
    |> List.reduce addFish
    |> magnitude

let part2 fish = 
    Seq.allPairs fish fish
    |> Seq.map (fun (a, b) -> addFish a b)
    |> Seq.map magnitude
    |> Seq.max

[<EntryPoint>]
let main argv =
    let input = IO.File.ReadAllLines "real_data.txt"
                |> Seq.map (parseFish 0)
                |> Seq.map fst
                |> Seq.toList

    printfn "Part 1: %d" (part1 input)
    printfn "Part 2: %d" (part2 input)

    0 // return an integer exit code