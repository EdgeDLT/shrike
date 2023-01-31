import { A, useLocation } from "solid-start"

export default function Results() {

    const result = useLocation().state

    return (
        <main class="container">
            <section>
                <table role="">
                    <thead>
                        <tr>
                            <th style="text-indent: 2em" colspan="2" scope="col"><b>Field</b></th>
                            <th colspan="0" scope="col"><b>Value</b></th>
                        </tr>
                    </thead>
                    <tbody>
                        <For each={result}>{(result) =>
                            <tr>
                                <th scope="row"></th>
                                <td>{result[0]}</td>
                                <td style="word-break: break-all"><code>{result[1]}</code></td>
                            </tr>
                        }</For>
                    </tbody>
                </table>
                <A href="/" role="button" class="outline">Back</A>
            </section>
        </main>
    )
}
