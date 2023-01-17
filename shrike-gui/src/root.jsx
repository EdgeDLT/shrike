// @refresh reload
import { Suspense } from "solid-js"
import {
    A,
    Body,
    ErrorBoundary,
    FileRoutes,
    Head,
    Html,
    Meta,
    Routes,
    Scripts,
    Title
} from "solid-start"
import "./pico.min.css"
export default function Root() {
    return (
        <Html lang="en">
            <Head>
                <Title>Shrike</Title>
                <Meta charset="utf-8" />
                <Meta
                    name="viewport"
                    content="width=device-width, initial-scale=1"
                />
            </Head>
            <Body>
            <Suspense>
            <ErrorBoundary>
                <main class="container">
                    <article>
                            <hgroup>
                                <h2>Shrike</h2>
                                <h3>A data analysis tool for Neo</h3>
                            </hgroup>
                            <br />
                        <Routes>
                            <FileRoutes />
                        </Routes>
                    </article>
                </main>
                </ErrorBoundary>
            </Suspense>
            <Scripts />
            </Body>
        </Html>
    )
}
