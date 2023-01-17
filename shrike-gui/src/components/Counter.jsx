import { createSignal } from "solid-js"
export default function Counter() {
    const [count, setCount] = createSignal(0)
    return (
        <main class="container">
            <section>
            <a
                href="#"
                role="button"
                class="outline"
                onClick={() => setCount(count() + 1)}
            >
                Clicks: {count()}
            </a>
            </section>
        </main>
    )
}
