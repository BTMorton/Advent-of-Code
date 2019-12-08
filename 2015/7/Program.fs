// Learn more about F# at http://fsharp.org

open System
open System.Text.RegularExpressions

type Action =
    | Assign
    | And
    | Or
    | LShift
    | RShift
    | Not

type VarInstruction = { action: Action; source: string }
type TwoVarInstruction = { action: Action; a: string; b: string }

type Instruction =
    | Var of VarInstruction
    | TwoVar of TwoVarInstruction

let parseCircuit line =
    let reg = Regex "((?<assign>[a-z0-9]+)|((?<aVar>[a-z0-9]+ )?(?<key>[A-Z]+) ((?<bVar>[a-z0-9]+)))) -> (?<target>[a-z]+)"
    let m = reg.Match line
    if m.Success
    then
        let target = m.Groups.["target"].Value
        if m.Groups.["key"].Success
        then match m.Groups.["key"].Value with
                | "NOT" -> Some (target, Var {
                                    action = Not
                                    source = m.Groups.["bVar"].Value
                                })
                | "LSHIFT" -> Some (target, TwoVar {
                                       action = LShift
                                       a = m.Groups.["aVar"].Value.Trim()
                                       b = m.Groups.["bVar"].Value
                                   })
                | "RSHIFT" -> Some (target, TwoVar {
                                       action = RShift
                                       a = m.Groups.["aVar"].Value.Trim()
                                       b = m.Groups.["bVar"].Value
                                   })
                | "AND" -> Some (target, TwoVar {
                                   action = And
                                   a = m.Groups.["aVar"].Value.Trim()
                                   b = m.Groups.["bVar"].Value
                                })
                | "OR" -> Some (target, TwoVar {
                                   action = Or
                                   a = m.Groups.["aVar"].Value.Trim()
                                   b = m.Groups.["bVar"].Value
                               })
                | _ -> None
        else Some (target, Var {
                action = Assign
                source = m.Groups.["assign"].Value
            })
    else None


let rec processWire (map: Map<string, Instruction>) (valueCache: Map<string, int option>) startPoint: (Map<string, int option> * int) option =
    let getValue (map: Map<string, Instruction>) (valueCache: Map<string, int option>) key =
        if valueCache.ContainsKey key
        then valueCache, valueCache.[key]
        else
            let result = processWire map valueCache key
            match result with
            | Some (newCache, value) -> newCache.Add(key, Some(value)), Some(value)
            | _ -> valueCache.Add (key, None), None

    // printfn "Loading instruction %A" startPoint
    if not (map.ContainsKey startPoint)
    then match Int32.TryParse(startPoint) with
         | (true, num) -> Some (valueCache, num)
         | _ -> None
    else
        let instr = map.[startPoint]
        match instr with
        | Var i ->
            match i.action with
            | Assign -> processWire map valueCache i.source
            | Not -> match processWire map valueCache i.source with
                     | Some (newCache, v) -> Some (newCache, ~~~v)
                     | None -> None
            | _ -> None
        | TwoVar i ->
            let (aCache, aVar) = getValue map valueCache i.a
            let (bCache, bVar) = getValue map aCache i.b
            if aVar.IsNone || bVar.IsNone
            then None
            else
                match i.action with
                | And -> Some (bCache, aVar.Value &&& bVar.Value)
                | Or -> Some (bCache, aVar.Value ||| bVar.Value)
                | LShift -> Some (bCache, aVar.Value <<< bVar.Value)
                | RShift -> Some (bCache, aVar.Value >>> bVar.Value)
                | _ -> None

[<EntryPoint>]
let main argv =
    let actions = IO.File.ReadAllLines "data.txt"
                  |> Seq.map parseCircuit
                  |> Seq.choose id
    let actionMap = new Map<string, Instruction>(actions)

    let part1 = processWire actionMap (new Map<string, int option>([])) "a"
    if part1.IsNone
    then printfn "Part 1 error"
    else
        let part1Value = snd part1.Value
        printfn "Part 1 %d" part1Value

        let newMap = actionMap.Add("b", Var {
            action = Assign;
            source = string part1Value
        })
        let part2 = processWire newMap (new Map<string, int option>([])) "a"
        match part2 with
        | Some v -> printfn "Part 2 %d" (snd v)
        | _ -> printfn "Part 2 error"

    0 // return an integer exit code
