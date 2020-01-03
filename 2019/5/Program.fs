// Learn more about F# at http://fsharp.org

open System
open IntCode.IntCode

[<EntryPoint>]
let main argv =
    let opts = loadFile "input_data.txt"

    let part1result = (execute64 (Array.copy opts) [1L])
    part1result.output
    |> List.head
    |> printfn "Part 1: %d"

    let part2result = (execute64 (Array.copy opts) [5L])
    part2result.output
    |> List.head
    |> printfn "Part 2: %d"

    // let test = [|3;21;1008;21;8;20;1005;20;22;107;8;21;20;1006;20;31;
    //             1106;0;36;98;0;0;1002;21;125;20;4;20;1105;1;46;104;
    //             999;1105;1;46;1101;1000;1;20;4;20;1105;1;46;98;99|]
    // IntCode.execute test
    // let test1 = [|3;9;8;9;10;9;4;9;99;-1;8|]
    // IntCode.execute test1
    // let test2 = [|3;9;7;9;10;9;4;9;99;-1;8|]
    // IntCode.execute test2
    // let test3 = [|3;3;1108;-1;8;3;4;3;99|]
    // IntCode.execute test3
    // let test4 = [|3;3;1107;-1;8;3;4;3;99|]
    // IntCode.execute test4

    // let test5 = [|3;12;6;12;15;1;13;14;13;4;13;99;-1;0;1;9|]
    // IntCode.execute test5
    // let test6 = [|3;3;1105;-1;9;1101;0;0;12;4;12;99;1|]
    // IntCode.execute test6
    0 // return an integer exit code
