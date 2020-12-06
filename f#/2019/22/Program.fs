// Learn more about F# at http://fsharp.org

open System
open System.Text.RegularExpressions

type Instruction =
    | Stack
    | Cut of int64
    | Deal of int64

let dealNewStack (len, cardPos) = len, (len - 1L) - cardPos
let cut at (len, cardPos) =
    let newPos = cardPos - at
    len, if newPos < 0L then len + newPos else newPos

let dealWithIncrement inc (len, cardPos) =
    len, cardPos * inc % len

let modulo n len =
    (n % len + len) % len

let getForwardFn instr =
    match instr with
    | Stack -> dealNewStack
    | Cut x -> cut x
    | Deal x -> dealWithIncrement x

let revCut at (len, cardPos) =
    let newPos = modulo (at + cardPos + len) len
    len, newPos

let cutRegex = Regex "cut (?<num>-?[0-9]+)"
let dealRegex = Regex "deal with increment (?<num>-?[0-9]+)"

let parseInstruction instr =
    if instr = "deal into new stack"
    then Some Stack
    else
    
    let m = cutRegex.Match instr
    if m.Success
    then m.Groups.["num"].Value |> int64 |> Cut |> Some
    else

    let m = dealRegex.Match instr
    if m.Success
    then m.Groups.["num"].Value |> int64 |> Deal |> Some
    else None

let findCard card =
    List.mapi (fun i c -> i,c)
    >> List.find (snd >> (=) card)
    >> fst

let modPow a b len = bigint.ModPow(a, b, len)
let modInv a len = modPow a (len - bigint 2) len
let fixMod a len = (a % len + len) % len

let applyFn (len: bigint) ((incrementMult: bigint), (offsetDiff: bigint)) instr =
    match instr with
    | Stack ->  let incrementMult = incrementMult * bigint -1
                let offsetDiff = offsetDiff + incrementMult
                fixMod incrementMult len, fixMod offsetDiff len
    | Cut x ->  let offsetDiff = offsetDiff + (bigint x * incrementMult)
                fixMod incrementMult len, fixMod offsetDiff len
    | Deal x -> let incrementMult = incrementMult * (modInv (bigint x) len)
                fixMod incrementMult len, fixMod offsetDiff len

let getSeq (iter: bigint) (len: bigint) ((incrementMult: bigint), (offsetDiff: bigint)) =
    let increment = modPow incrementMult iter len
    let offset = offsetDiff * ((bigint 1) - increment) * (modInv (((bigint 1) - incrementMult ) % len) len)
    let offset = offset % len
    increment, offset

[<EntryPoint>]
let main argv =
    let inputs = 
        IO.File.ReadAllLines "data.txt"
        |> Array.map parseInstruction
        |> Array.choose id

    let actions = inputs |> Array.map getForwardFn |> Array.reduce (>>)
    printfn "Part 1: %d" (actions (10007L, 2019L) |> snd)

    let len = bigint 119315717514047L
    let iter = bigint 101741582076661L
    let card = bigint 2020L

    let increment, offset =
        inputs
        |> Array.fold (applyFn len) (bigint 1, bigint 0)
        |> getSeq iter len

    let result = (offset + card * increment) % len
    printfn "Part 2: %A" result
    0