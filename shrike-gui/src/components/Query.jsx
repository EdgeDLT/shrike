import { createSignal, Match, Show } from "solid-js"
const path = "http://127.0.0.1:8080/v1"

const [activeResult, setActiveResult] = createSignal(0)
const [queryResult, setQueryResult] = createSignal({})

function flatten(obj) {
    let arr = Object.entries(obj)

    return arr.map(el => {
        if (
            el[0] === "witnesses" ||
            el[0] === "signers" ||
            el[0] === "stack_result" ||
            el[0] === "notifications"
            ) {
            return [el[0], JSON.stringify(el[1])]
        } else {
            return [el[0], el[1]]
        }
    })
}

function resetView() {
    setActiveResult(0)
    setQueryResult({})
}

async function fetchBlock() {
    let value = document.getElementById("getblock").value
    let res = await fetch(`${path}/block/${value}`)
    let block = await res.json()
    setQueryResult(flatten(block))
    setActiveResult(1)
}

async function fetchTransaction() {
    let value = document.getElementById("gettransaction").value
    let res = await fetch(`${path}/transaction/${value}`)
    let tx = await res.json()
    setQueryResult(flatten(tx))
    setActiveResult(1)
}

// async function fetchInvocations() {
//     let value = document.getElementById("getinvocations").value
//     let res = await fetch(`${path}/stat/total_invocations/${value}`)
//     let invokes = await res.json()
//     setQueryResult(flatten(invokes))
//     setActiveResult(1)
// }

export default function Query() {
    return (
        <main class="container">
            <section>
                <Switch>
                    <Match when={activeResult() === 0}>
                        <form onsubmit={(e)=>{e.preventDefault(); fetchBlock()}}>
                            <label for="getblock">
                                Get Block
                                <input
                                    type="text"
                                    id="getblock"
                                    name="getblock"
                                    placeholder="420"
                                    required
                                >
                                </input>
                            </label>
                        </form>

                        <form onsubmit={(e)=>{e.preventDefault(); fetchTransaction()}}>
                            <label for="gettransaction">
                                Get Transaction
                                <input
                                    type="text"
                                    id="gettransaction"
                                    name="gettransaction"
                                    placeholder="0x72a4073d926405d0d40ebea07a1438744e73cd8accb6a09efb1def5f19110145"
                                    required
                                >
                                </input>
                            </label>
                        </form>

                        {/* <form onsubmit={(e)=>{e.preventDefault(); fetchInvocations()}}>
                            <label for="getinvocations">
                                Get Total Contract Invocations
                                <input
                                    type="text"
                                    id="getinvocations"
                                    name="getinvocations"
                                    placeholder="0x24fc2b1bd77778dbbefb7c9c4fdb020a7aba6986"
                                    required
                                >
                                </input>
                            </label>
                        </form> */}
                    </Match>
                    <Match when={activeResult() === 1}>
                        <table role="grid">
                            <thead style="">
                                <tr>
                                    <th style="text-indent: 2em" colspan="2" scope="col"><b>Field</b></th>
                                    <th colspan="0" scope="col"><b>Value</b></th>
                                </tr>
                            </thead>
                            <tbody>
                                <For each={queryResult()}>{(a) =>
                                    <tr>
                                        <th scope="row"></th>
                                        <td>{a[0]}</td>
                                        <td style="word-break: break-all">{a[1]}</td>
                                    </tr>
                                }</For>
                            </tbody>
                        </table>

                        <a href="#" role="button" class="outline" onClick={() => resetView()}>Back</a>
                    </Match>
                </Switch>
            </section>
        </main>
    )
}
