import { onMount, createSignal } from "solid-js"
import { A, useLocation } from "solid-start"
import { Grapher } from "../helpers/grapher.js"
import { Formatter } from "../helpers/formatter.js"

const loadGraph = (nodes, edges) => {
    Grapher.draw(nodes, edges)
}

export default function Visualizer() {

    const result = useLocation().state
    const [address, setAddress] = createSignal(result.address)
    const [activeGraph, setActiveGraph] = createSignal("Sender")
    const [senderData, setSenderData] = createSignal([])
    const [participantData, setParticipantData] = createSignal([])

    if (typeof result === "undefined" || result === null) {
        return (
            <section>
                <p>No address loaded for visualization.</p>
                <A href="/" role="button" class="outline">Back</A>
            </section>
        )
    } else {

        const senderNodes = Grapher.getNodesFromAddresses(Formatter.getAddresses(result.as_sender))
        const senderEdges = Grapher.getEdgesFromTransfers(Formatter.getTransfers(result.as_sender))
        setSenderData([senderNodes, senderEdges])

        const participantNodes = Grapher.getNodesFromAddresses(Formatter.getAddresses(result.as_participant))
        const participantEdges = Grapher.getEdgesFromTransfers(Formatter.getTransfers(result.as_participant))
        setParticipantData([participantNodes, participantEdges])

        onMount(() => {
            loadGraph(senderData()[0], senderData()[1])
        })
    }

    return (
        <section>
            <div>
                <hgroup>
                    <h3>Visualizer</h3>
                    <br />
                    <h5>{activeGraph()}</h5>
                    <h6>{address()}</h6>
                </hgroup>
            </div>
            <div style="border: 2px solid; margin-bottom: 1rem" class="container-fluid">
                <div style="height: 55vh" id="cyto"></div>
            </div>
            <div class="container">
                <a
                    href="#" role="button"  class="outline contrast" onclick={
                        (e) => {
                            e.preventDefault()
                            if (activeGraph() === "Sender") {
                                setActiveGraph("Participant")
                                loadGraph(participantData()[0], participantData()[1])
                            } else {
                                setActiveGraph("Sender")
                                loadGraph(senderData()[0], senderData()[1])
                            }
                        }
                    }>Switch
                </a>
                <A href="/" role="button" style="margin-left: 10px" class="outline contrast">Back</A>
            </div>
        </section>
    )
}
