// Learn more about F# at http://docs.microsoft.com/dotnet/fsharp

open System
open AOCHelpers.AOC

type PacketContents = Group of list<Packet> | Literal of uint64
and Packet = {
    version: int
    typeId: int
    content: PacketContents
}

let rec collectLiteralPackets (chunks: list<char[]>) =
    match chunks with
    | [] -> []
    | chunk::_ when chunk.[0] = '0' -> [(chunk.[1..])]
    | chunk::tail -> (chunk.[1..])::(collectLiteralPackets tail)


let rec parsePacketsLength (substr: string) =
    let (packet, len) = parsePacket substr
    let rem = substr.Substring(len)

    if rem |> Seq.forall ((=) '0')
    then [packet]
    else packet::(parsePacketsLength rem)

and parsePacketsTarget (substr: string) target =
    if target = 0
    then ([], 0)
    else 
        let (packet, len) = parsePacket substr
        let (subpackets, totalLen) = parsePacketsTarget (substr.Substring(len)) (target - 1)
        (packet::subpackets, len + totalLen)

and parseLiteralPackage (substr: string) =
    let numPackets =
        substr
        |> Seq.chunkBySize 5
        |> Seq.toList
        |> collectLiteralPackets

    let num =
        numPackets
        |> Seq.collect id
        |> String.Concat

    (Literal(Convert.ToUInt64(num, 2)), 5 * numPackets.Length)

and parsePacketGroup (substr: string) =
    let (subPackets, len) =
        match substr.[0] with
        | '1'-> let target = Convert.ToInt32(substr.Substring(1, 11), 2)
                let (subpackets, len) = parsePacketsTarget (substr.Substring(12)) target
                (subpackets, len + 12)
        | _  -> let len = Convert.ToInt32(substr.Substring(1, 15), 2)
                (parsePacketsLength (substr.Substring(16, len)), 16 + len)

    (Group(subPackets), len)

and parsePacket (str: string) =
    let ver = Convert.ToInt32(str.Substring(0, 3), 2)
    let typeId = Convert.ToInt32(str.Substring(3, 3), 2)

    let (content, pktLen) =
        match typeId with 
        | 4 -> parseLiteralPackage (str.Substring(6))
        | x -> parsePacketGroup (str.Substring(6))
    
    let packet = {
        version = ver;
        typeId = typeId;
        content = content;
    }

    (packet, pktLen + 6)

let rec sumPackets packets =
    packets
    |> Seq.sumBy sumPacket

and sumPacket packet =
    match packet.content with
    | Group(subPackets) -> packet.version + sumPackets subPackets
    | _ -> packet.version

let rec applyPackets packet =
    match packet.content with
    | Literal(n) -> n
    | Group(subpackets) ->
        match packet.typeId with
        | 0 -> subpackets |> Seq.map applyPackets |> Seq.sum
        | 1 -> subpackets |> Seq.map applyPackets |> Seq.reduce (*)
        | 2 -> subpackets |> Seq.map applyPackets |> Seq.min
        | 3 -> subpackets |> Seq.map applyPackets |> Seq.max
        | 5 -> subpackets |> Seq.take 2 |> Seq.map applyPackets |> seqTo2Tuple |> (fun (a, b) -> if a > b then 1UL else 0UL) 
        | 6 -> subpackets |> Seq.take 2 |> Seq.map applyPackets |> seqTo2Tuple |> (fun (a, b) -> if a < b then 1UL else 0UL)
        | 7 -> subpackets |> Seq.take 2 |> Seq.map applyPackets |> seqTo2Tuple |> (fun (a, b) -> if a = b then 1UL else 0UL)
        | _ ->  failwith "Invalid packet operator"

let part1 = 
    sumPacket

let part2 = 
    applyPackets

[<EntryPoint>]
let main argv =
    let input = IO.File.ReadAllLines "real_data.txt"
                |> Seq.head
    // let input = "9C0141080250320F1802104A08"
                |> Seq.map (fun c -> Convert.ToInt32(c.ToString(), 16))
                |> Seq.map (fun n -> "000" + Convert.ToString(n, 2))
                |> Seq.map (fun str -> str.Substring(str.Length - 4))
                |> String.concat ""
                |> parsePacket
                |> fst

    printfn "Part 1: %d" (part1 input)
    printfn "Part 2: %d" (part2 input)

    0 // return an integer exit code