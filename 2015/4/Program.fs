// Learn more about F# at http://fsharp.org

open System
open System.Security.Cryptography
open System.Text

let md5 (str: string) =
    use md5 = MD5.Create()
    str
    |> Encoding.ASCII.GetBytes
    |> md5.ComputeHash
    |> Seq.map (fun c -> c.ToString("X2"))
    |> Seq.reduce (+)

let rec findHash input (zeroes: string) num =
    let hash = md5 (input + (string num))
    match hash with
    | x when x.StartsWith zeroes -> num
    | _ -> findHash input zeroes (num + 1)


[<EntryPoint>]
let main argv =
    let input = "bgvyzdsv"
    // let input = "abcdef"
    printfn "Part 1 %d" (findHash input "00000" 1)
    printfn "Part 2 %d" (findHash input "000000" 1)
    0 // return an integer exit code
