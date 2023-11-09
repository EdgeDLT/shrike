import Query from "~/components/Query"
import Stats from "~/components/Stats"
import About from "~/components/About"
export default function Home() {
    return (
        <article>
            <header>
                <Stats />
            </header>
                <Query />
            <footer>
                <About />
            </footer>
        </article>
    )
}
