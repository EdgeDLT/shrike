// @refresh reload
import { Suspense, createSignal } from "solid-js"
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

import Stats from "./components/Stats"

import "./pico.min.css"
export default function Root() {

    const moon = <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path></svg>
    const sun = <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="5"></circle><line x1="12" y1="1" x2="12" y2="3"></line><line x1="12" y1="21" x2="12" y2="23"></line><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line><line x1="1" y1="12" x2="3" y2="12"></line><line x1="21" y1="12" x2="23" y2="12"></line><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line></svg>

    const [theme, setTheme] = createSignal("dark")
    const [themeIcon, setThemeIcon] = createSignal(sun)

    const toggleDarkMode = () => {
        toggleTheme()
        toggleThemeIcon()
    }

    const toggleTheme = () => { theme() === "dark" ? setTheme("light") : setTheme("dark") }
    const toggleThemeIcon = () => { themeIcon() === moon ? setThemeIcon(sun) : setThemeIcon(moon) }

    return (
        <Html lang="en" data-theme={theme()}>
            <Head>
                <Title>Shrike</Title>
                <Meta charset="utf-8" />
                <Meta name="viewport" content="width=device-width, initial-scale=1" />
                <Meta name="description" content="Blockchain data analysis infrastructure for Neo" />
            </Head>
            <Body>
            <Suspense>
            <ErrorBoundary>
                <main class="container">
                    <div style="padding-bottom: 1rem">
                        <a href="#" role="button" class="outline" style="float: right" onClick={() => toggleDarkMode()}>{themeIcon()}</a>
                    </div>
                    <article>
                        <hgroup>
                            <h2>Shrike</h2>
                            <h3>A data analysis tool for Neo</h3>
                        </hgroup>
                        <br />
                        <Stats />
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
