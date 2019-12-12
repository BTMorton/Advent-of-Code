namespace IntCode

module IntCode =
    type ParamMode =
        | Immediate
        | Position
        | Relative

    type State =
        | Running
        | Paused
        | Halted

    type Computer =
        { memory: Map<int64, int64>
          curIndex: int64
          input: int64 list
          output: int64 list
          state: State
          relativeBase: int64 }

    let private getParamMode paramModes paramNo =
        let div = (10.0 ** ((paramNo |> double) - 1.0)) |> int64

        let mode =
            paramModes
            |> (/)
            <| div
            |> (%)
            <| 10L
        // printfn "Param modes %d no %d mode %d calc %d" paramModes paramNo mode calc
        match mode with
        | 2L -> Relative
        | 1L -> Immediate
        | _ -> Position

    let private lookup computer index =
        // printfn "index %d exists %b" index (computer.memory.ContainsKey index)
        if computer.memory.ContainsKey index then computer.memory.[index]
        else 0L

    let private getIndex computer paramModes paramNo =
        let { curIndex = startIndex } = computer
        let mode = (getParamMode paramModes paramNo)
        let index = startIndex + paramNo
        // printfn "Getting param %d with mode %A and index %d and lookup %d and base %d" paramNo mode index
        // (lookup computer index) computer.relativeBase

        match mode with
        | Immediate -> index
        | Position -> lookup computer index
        | Relative -> computer.relativeBase + (lookup computer index)

    let private calcValue computer paramModes paramNo = getIndex computer paramModes paramNo |> lookup computer

    let private storeValue computer paramModes paramNo value =
        let { memory = map } = computer
        let storeIndex = getIndex computer paramModes paramNo
        // printfn "setting %d to %d mode %A index %d" value storeIndex mode index
        { computer with memory = map.Add(storeIndex, value) }

    let private twoParamOp (op: int64 -> int64 -> int64) paramModes computer =
        let { curIndex = index } = computer
        let input1 = (calcValue computer paramModes 1L)
        let input2 = (calcValue computer paramModes 2L)
        let result = (op input1 input2)
        // printfn "twoParamOp %d %d %d" input1 input2 result
        let newComputer = storeValue computer paramModes 3L result
        { newComputer with curIndex = index + 4L }

    let private store paramModes computer =
        let { memory = arr; curIndex = index; input = inputData } = computer

        match inputData with
        | input :: remaining ->
            let newComputer = storeValue computer paramModes 1L input

            // printfn "store %d" input
            { newComputer with
                  curIndex = index + 2L
                  input = remaining }
        | [] -> { computer with state = Paused }

    let private print paramModes computer =
        let { memory = arr; curIndex = index } = computer
        let output = (calcValue computer paramModes 1L)
        // printfn "Output: %d" output
        { computer with
              curIndex = index + 2L
              output = output :: computer.output }

    let private jumpIf comp paramModes computer =
        let { memory = arr; curIndex = index } = computer
        let param = (calcValue computer paramModes 1L)
        let target = (calcValue computer paramModes 2L)
        // printfn "Checking param %d" param
        if param
           |> int
           |> comp
        then
            // printfn "Jumping to %d" target
            { computer with curIndex = target }
        else
            // printfn "Not jumping"
            { computer with curIndex = index + 3L }

    let private setIf comp paramModes computer =
        let { curIndex = index } = computer
        let input1 = (calcValue computer paramModes 1L)
        let input2 = (calcValue computer paramModes 2L)

        let value =
            if comp input1 input2 then 1L
            else 0L
        // printfn "setIf %d %d %d" input1 input2 value

        let newComputer = storeValue computer paramModes 3L value

        { newComputer with curIndex = index + 4L }

    let private getOpCode computer =
        let { memory = arr; curIndex = index } = computer
        arr.[index] % 100L

    let private getParamModes computer =
        let { memory = arr; curIndex = index } = computer
        arr.[index] / 100L

    let private halt computer = { computer with state = Halted }

    let private updateRelativeBase paramModes computer =
        let change = calcValue computer paramModes 1L
        // printfn "updateRelativeBase %d" change
        { computer with
              relativeBase = computer.relativeBase + change
              curIndex = computer.curIndex + 2L }

    let private getOpCodeMethod computer =
        let paramModes = getParamModes computer
        let opCode = getOpCode computer
        // printfn "opCode: %A" opCode
        match opCode with
        | 1L -> Some(twoParamOp (+) paramModes)
        | 2L -> Some(twoParamOp (*) paramModes)
        | 3L -> Some(store paramModes)
        | 4L -> Some(print paramModes)
        | 5L -> Some(jumpIf ((<>) 0) paramModes)
        | 6L -> Some(jumpIf ((=) 0) paramModes)
        | 7L -> Some(setIf (<) paramModes)
        | 8L -> Some(setIf (=) paramModes)
        | 9L -> Some(updateRelativeBase paramModes)
        | 99L -> Some halt
        | _ -> None

    let rec private processArray computer =
        match getOpCodeMethod computer with
        | Some fn ->
            let newComp = fn computer
            match newComp.state with
            | Running -> processArray newComp
            | _ -> newComp
        | None -> halt computer

    let execute64 (arr: int64 array) (inputData: int64 list) =
        let computer =
            { memory =
                  (arr
                   |> Array.mapi (fun i c -> (i |> int64, c))
                   |> Map.ofArray)
              input = inputData
              output = []
              curIndex = 0L
              state = Running
              relativeBase = 0L }
        // computer.memory
        // |> Map.toList
        // |> List.iter (printfn "%A")
        processArray computer

    let execute (arr: int array) (inputData: int list) =
        execute64 (arr |> Array.map int64) (inputData |> List.map int64)

    let executeComputer computer = processArray computer
