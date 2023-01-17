import About from "~/components/About"
import Query from "~/components/Query"
import Stats from "~/components/Stats"
export default function Home() {
    return (
        <main>
            <Stats />
            <Query />
            <About />
        </main>
    )
}
