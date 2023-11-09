import { A, Title } from "solid-start"
import { HttpStatusCode } from "solid-start/server"
export default function NotFound() {
    return (
        <article>
            <Title>Not Found</Title>
            <HttpStatusCode code={404} />
            <h1>Page Not Found</h1>
            <A href="/" role="button" class="outline">Home</A>
        </article>
    )
}
