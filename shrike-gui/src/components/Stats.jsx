import { createEffect, createSignal } from "solid-js"
import { API_PATH } from "../constants/index.js"

export default function Stats() {

    const [blocks, setBlocks] = createSignal(0)
    const [transactions, setTransactions] = createSignal(0)
    const [senders, setSenders] = createSignal(0)
    const [burned, setBurned] = createSignal(0)
    const [deployed, setDeployed] = createSignal(0)
    const [transfers, setTransfers] = createSignal(0)

    createEffect(async () => {

        let stats = [
            "total_blocks",
            "total_transactions",
            "total_senders",
            "total_sysfee",
            "total_contracts",
            "total_transfers"
        ]

        const requests = stats.map(s => fetch(`${API_PATH}/stat/${s}`))
        const unprocessedResponses = await Promise.all(requests)
        const process = unprocessedResponses.map(r => r.json())
        const responses = await Promise.all(process)

        setBlocks(responses[0].total_blocks)
        setTransactions(responses[1].total_transactions)
        setSenders(responses[2].total_senders)
        setBurned(responses[3].total_sysfee)
        setDeployed(responses[4].total_contracts)
        setTransfers(responses[5].total_transfers)
    })

    return (
        <main class="container">
                <div style="grid-template-columns: 1fr 1fr 1fr" class="grid">
                    <hgroup>
                        <Switch>
                            <Match when={blocks() == 0}>
                                <h4 aria-busy="true"> </h4>
                                <h5>Current Height</h5>
                            </Match>
                            <Match when={blocks() !== 0}>
                                <h4>{ blocks() }</h4>
                                <h5>Current Height</h5>
                            </Match>
                        </Switch>
                    </hgroup>
                    <hgroup>
                        <Switch>
                            <Match when={transactions() == 0}>
                                <h4 aria-busy="true"> </h4>
                                <h5>Total Transactions</h5>
                            </Match>
                            <Match when={transactions() !== 0}>
                                <h4>{ transactions() }</h4>
                                <h5>Total Transactions</h5>
                            </Match>
                        </Switch>
                    </hgroup>
                    <hgroup>
                        <Switch>
                            <Match when={senders() == 0}>
                                <h4 aria-busy="true"> </h4>
                                <h5>Unique Senders</h5>
                            </Match>
                            <Match when={senders() !== 0}>
                                <h4>{ senders() }</h4>
                                <h5>Unique Senders</h5>
                            </Match>
                        </Switch>
                    </hgroup>
                    <hgroup>
                    <Switch>
                            <Match when={burned() == 0}>
                                <h4 aria-busy="true"> </h4>
                                <h5>Total GAS Burned</h5>
                            </Match>
                            <Match when={burned() !== 0}>
                                <h4>{ burned() }</h4>
                                <h5>Total GAS Burned</h5>
                            </Match>
                        </Switch>
                    </hgroup>
                    <hgroup>
                    <Switch>
                            <Match when={deployed() == 0}>
                                <h4 aria-busy="true"> </h4>
                                <h5>Total Contracts</h5>
                            </Match>
                            <Match when={deployed() !== 0}>
                                <h4>{ deployed() }</h4>
                                <h5>Total Contracts</h5>
                            </Match>
                        </Switch>
                    </hgroup>
                    <hgroup>
                    <Switch>
                            <Match when={transfers() == 0}>
                                <h4 aria-busy="true"> </h4>
                                <h5>Total Transfers</h5>
                            </Match>
                            <Match when={transfers() !== 0}>
                                <h4>{ transfers() }</h4>
                                <h5>Total Transfers</h5>
                            </Match>
                        </Switch>
                    </hgroup>
                </div>
                <br />
        </main>

    )
}
