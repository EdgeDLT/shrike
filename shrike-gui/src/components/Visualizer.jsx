import { For } from "solid-js"
import { A, useLocation } from "solid-start"

export default function Visualizer() {

    const result = useLocation().state

    return (
        <main class="container">
            <section>

                <div>
                    <h1>Visualizer</h1>
                    <p>{JSON.stringify(result)}</p>
                </div>

                <A href="/" role="button" class="outline">Back</A>
            </section>
        </main>
    )
}
