import { createSignal, onMount } from "solid-js"
import { API_PATH } from "../constants/index.js"

export default function Stats() {

    const fetchStats = async () => {
        let stat_request = fetch(`${API_PATH}/stat/stats`)
        let stats = await (await stat_request).json()

        setBlocks(stats.total_blocks)
        setTransactions(stats.total_transactions)
        setSenders(stats.total_senders)
        setBurned(stats.total_sysfee)
        setDeployed(stats.total_contracts)
        setTransfers(stats.total_transfers)
    }

    const [blocks, setBlocks] = createSignal(0)
    const [transactions, setTransactions] = createSignal(0)
    const [senders, setSenders] = createSignal(0)
    const [burned, setBurned] = createSignal(0)
    const [deployed, setDeployed] = createSignal(0)
    const [transfers, setTransfers] = createSignal(0)

    onMount(async () => {

        await fetchStats()
        setInterval(async () => {
            await fetchStats(), 5000
        })
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
