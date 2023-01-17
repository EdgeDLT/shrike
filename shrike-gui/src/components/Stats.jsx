import { createEffect, createSignal } from "solid-js"

const path = "http://127.0.0.1:8080/v1"

export default function Stats() {

    const [blocks, setBlocks] = createSignal(0)
    const [transactions, setTransactions] = createSignal(0)
    const [senders, setSenders] = createSignal(0)
    const [burned, setBurned] = createSignal(0)
    const [deployed, setDeployed] = createSignal(0)

    createEffect(async () => {
        const blockRequest = await fetch("http://127.0.0.1:8080/v1/stat/total_blocks")
        const blockResponse = await blockRequest.json()
        setBlocks(blockResponse.total_blocks)

        const transactionRequest = await fetch("http://127.0.0.1:8080/v1/stat/total_transactions")
        const transactionResponse = await transactionRequest.json()
        setTransactions(transactionResponse.total_transactions)


        const senderRequest = await fetch("http://127.0.0.1:8080/v1/stat/total_senders")
        const senderResponse = await senderRequest.json()
        setSenders(senderResponse.total_senders)

        const burnedRequest = await fetch("http://127.0.0.1:8080/v1/stat/total_sysfee")
        const burnedResponse = await burnedRequest.json()
        setBurned(burnedResponse.total_sysfee)

        const deployedRequest = await fetch("http://127.0.0.1:8080/v1/stat/total_deploys")
        const deployedResponse = await deployedRequest.json()
        setDeployed(deployedResponse.total_deploys)
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
                                <h5>Total Contracts Deployed</h5>
                            </Match>
                            <Match when={deployed() !== 0}>
                                <h4>{ deployed() }</h4>
                                <h5>Total Contracts Deployed</h5>
                            </Match>
                        </Switch>
                    </hgroup>
                </div>
                <br />
        </main>

    )
}
