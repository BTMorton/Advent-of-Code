namespace IntCode

module IntCode =
    type ParamMode =
        | Immediate
        | Position

    type State =
        | Running
        | Paused
        | Halted

    type Computer =
        { memory: int array
          curIndex: int
          input: int list
          output: int list
          state: State }

    let private getParamMode paramModes paramNo =
        let mode =
            match paramNo with
            | 1 -> paramModes
            | _ -> paramModes / (10 * (paramNo - 1))
            |> (%)
            <| 10
        // printfn "Param mode %d no %d mode %d" paramModes paramNo mode
        match mode with
        | 1 -> Immediate
        | _ -> Position

    let private calcValue computer paramModes paramNo =
        let { memory = arr; curIndex = startIndex } = computer
        let mode = (getParamMode paramModes paramNo)
        let index = startIndex + paramNo
        let param = arr.[index]
        // printfn "Reading mode %A from param %d" mode param
        match mode with
        | Immediate -> param
        | Position -> arr.[param]

    let private storeValue computer paramModes paramNo value =
        let { memory = arr; curIndex = startIndex } = computer
        // printfn "Doing store %d" startIndex
        let mode = (getParamMode paramModes paramNo)
        let index = startIndex + paramNo

        let storeIndex =
            match mode with
            | Immediate -> index
            | Position -> arr.[index]
        // printfn "setting %d to %d mode %A index %d" value storeIndex mode index
        Array.set arr storeIndex value

    let private twoParamOp (op: int -> int -> int) paramModes computer =
        let { memory = arr; curIndex = index } = computer
        let input1 = (calcValue computer paramModes 1)
        let input2 = (calcValue computer paramModes 2)
        let result = (op input1 input2)

        storeValue computer paramModes 3 result
        { computer with curIndex = index + 4 }

    let private store paramModes computer =
        let { memory = arr; curIndex = index; input = inputData } = computer

        match inputData with
        | input :: remaining ->
            storeValue computer paramModes 1 input

            { computer with
                  curIndex = index + 2
                  input = remaining }
        | [] -> { computer with state = Paused }

    let private print paramModes computer =
        let { memory = arr; curIndex = index } = computer
        let output = (calcValue computer paramModes 1)
        printfn "Output: %d" output
        { computer with
              curIndex = index + 2
              output = output :: computer.output }

    let private jumpIf comp paramModes computer =
        let { memory = arr; curIndex = index } = computer
        let param = (calcValue computer paramModes 1)
        let target = (calcValue computer paramModes 2)
        // printfn "Checking param %d" param
        if comp param then
            // printfn "Jumping to %d" target
            { computer with curIndex = target }
        else
            // printfn "Not jumping"
            { computer with curIndex = index + 3 }

    let private setIf comp paramModes computer =
        let { memory = arr; curIndex = index } = computer
        let input1 = (calcValue computer paramModes 1)
        let input2 = (calcValue computer paramModes 2)

        let value =
            if comp input1 input2 then 1
            else 0

        storeValue computer paramModes 3 value

        { computer with curIndex = index + 4 }

    let private getOpCode computer =
        let { memory = arr; curIndex = index } = computer
        arr.[index] % 100

    let private getParamModes computer =
        let { memory = arr; curIndex = index } = computer
        arr.[index] / 100

    let private halt computer = { computer with state = Halted }

    let private getOpCodeMethod computer =
        let paramModes = getParamModes computer
        match getOpCode computer with
        | 1 -> Some(twoParamOp (+) paramModes)
        | 2 -> Some(twoParamOp (*) paramModes)
        | 3 -> Some(store paramModes)
        | 4 -> Some(print paramModes)
        | 5 -> Some(jumpIf ((<>) 0) paramModes)
        | 6 -> Some(jumpIf ((=) 0) paramModes)
        | 7 -> Some(setIf (<) paramModes)
        | 8 -> Some(setIf (=) paramModes)
        | 99 -> Some halt
        | _ -> None

    let rec private processArray computer =
        match getOpCodeMethod computer with
        | Some fn ->
            let newComp = fn computer
            match newComp.state with
            | Running -> processArray newComp
            | _ -> newComp
        | None -> halt computer

    let execute (arr: int array) (inputData: int list) =
        let computer =
            { memory = arr
              input = inputData
              output = []
              curIndex = 0
              state = Running }

        processArray computer

    let executeComputer computer = processArray computer
