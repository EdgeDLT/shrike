import Query from "~/components/Query"
import Stats from "~/components/Stats"
import About from "~/components/About"
export default function Home() {
    return (
        <main>
            <article>
                <header>
                    <hgroup>
                        <h2>Shrike</h2>
                        <h3>A data analysis tool for Neo</h3>
                    </hgroup>
                </header>
                    <Stats />
                <footer>
                    <Query />
                    <About />
                </footer>
            </article>
        </main>
    )
}
