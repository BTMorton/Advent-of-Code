// Learn more about F# at http://fsharp.org

open System
open IntCode.IntCode

let generateComputers opts max =
    [0L..(max - 1L)]
    |> List.map (fun i -> i, createComputer opts [i])
    |> Map.ofList

let addPacketToQueue (packetQueue: Map<int64, int64 list>) (packet: int64 list) =
    let packetList =
        if packetQueue.ContainsKey packet.[0]
        then packetQueue.[packet.[0]]
        else []

    packetQueue.Add(packet.[0], packet.[2]::packet.[1]::packetList)

let processPacketOutputs (packetQueue: Map<int64, int64 list>) (output: int64 list) =
    output
    |> List.rev
    |> List.chunkBySize 3
    |> List.fold addPacketToQueue packetQueue

let readNextPacket (packetQueue: Map<int64, int64 list>) index =
    let sentPackets =
        if packetQueue.ContainsKey index
        then packetQueue.[index]
        else []

    let nextInput, remaining =
        match List.rev sentPackets with
        | x::y::tail -> [x;y], List.rev tail
        | _ -> [-1L],[]

    nextInput, packetQueue.Add(index, remaining)

let forwardPacketInputs ((packetQueue: Map<int64, int64 list>), (network: Map<int64, Computer>)) index =
    if network.ContainsKey(index) |> not
    then packetQueue, network
    else
        let input, map = readNextPacket packetQueue index

        map, network.Add(index, { network.[index] with input = input; output = []; })

let runNetworkComputers =
    Map.toList
    >> List.map (fun (i, c) -> i, resumeComputer c)
    >> Map.ofList

let processNetworkOutput (packetQueue: Map<int64, int64 list>) (network: Map<int64, Computer>) =
    let networkList = network |> Map.toList
    let packetQueue =
        networkList
        |> List.map (snd >> fun c -> c.output)
        |> List.fold processPacketOutputs packetQueue

    networkList
    |> List.map fst
    |> List.fold forwardPacketInputs (packetQueue, network)

let runNetwork (packetQueue: Map<int64, int64 list>) (network: Map<int64, Computer>) =
    runNetworkComputers network
    |> processNetworkOutput packetQueue

let part1 (network: Map<int64, Computer>) =
    let rec iter (packetsSent: Map<int64, int64 list>) (network: Map<int64, Computer>) =
        let packetQueue, network = runNetwork packetsSent network

        if packetQueue.ContainsKey 255L
        then packetQueue.[255L]
        else iter packetQueue network

    iter Map.empty network

let isQueueIndexEmpty (packetQueue: Map<int64, int64 list>) index =
    packetQueue.ContainsKey index |> not || (packetQueue.[index] |> List.isEmpty)

let isNetworkIdle (network: Map<int64, Computer>) =
    network
    |> Map.toList
    |> List.forall (fun (_, comp) -> comp.input.[0] = -1L)

let sendNATPacket (lastNATPacket: int64 list) (network: Map<int64, Computer>) =
    match lastNATPacket with
    | y::x::_ ->
        let newComputer = { network.[0L] with input = [x;y]; }
        (Some y), network.Add(0L, newComputer)
    | _ -> None, network

let part2 (network: Map<int64, Computer>) =
    let rec iter lastYPacketSent lastNATPacket (packetQueue: Map<int64, int64 list>) (network: Map<int64, Computer>) =
        let packetQueue, network = runNetwork packetQueue network

        let lastNATPacket, packetQueue =
            if isQueueIndexEmpty packetQueue 255L
            then lastNATPacket, packetQueue
            else
                let packet = packetQueue.[255L] |> List.take 2
                packet, packetQueue.Add(255L, [])

        if isNetworkIdle network
        then
            match sendNATPacket lastNATPacket network with
            | Some y, _ when y = lastYPacketSent -> y
            | Some y, network -> iter y [] packetQueue network
            | None, _ -> iter lastYPacketSent lastNATPacket packetQueue network
        else iter lastYPacketSent lastNATPacket packetQueue network

    runNetwork ([255L,[]] |> Map.ofList) network
    ||> iter -1L []

[<EntryPoint>]
let main argv =
    let opts = loadFile "data.txt"
    let network = generateComputers opts 50L

    printfn "Part 1: %d" (part1 network |> List.rev |> List.item 1)
    printfn "Part 2: %d" (part2 network)
    0 // return an integer exit code
