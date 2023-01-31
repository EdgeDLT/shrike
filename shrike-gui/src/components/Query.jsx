import { useNavigate } from "solid-start"
import { Fetcher } from "../helpers/fetcher.js"

export default function Query() {

    const navigate = useNavigate()

    return (
        <main class="container">
            <section>
                <form onsubmit={async (e)=>{
                    e.preventDefault();
                    let result = await Fetcher.block();
                    if (result) {
                        navigate("/result", { state: result })
                    }
                }}>
                    <label for="getblock">
                        Get Block
                        <input
                            type="text"
                            id="getblock"
                            name="getblock"
                            placeholder="420"
                            required
                        >
                        </input>
                    </label>
                </form>

                <form onsubmit={async (e)=>{
                    e.preventDefault();
                    let result = await Fetcher.transaction();
                    if (result) {
                        navigate("/result", { state: result })
                    }
                }}>
                    <label for="gettransaction">
                        Get Transaction
                        <input
                            type="text"
                            id="gettransaction"
                            name="gettransaction"
                            placeholder="0x72a4073d926405d0d40ebea07a1438744e73cd8accb6a09efb1def5f19110145"
                            required
                        >
                        </input>
                    </label>
                </form>

                <form onsubmit={async (e)=>{
                    e.preventDefault();
                    let result = await Fetcher.blockTransactions();
                    if (result) {
                        navigate("/result", { state: result })
                    }
                }}>
                    <label for="getblocktransactions">
                        Get Block Transactions
                        <input
                            type="text"
                            id="getblocktransactions"
                            name="getblocktransactions"
                            placeholder="0xeaa8a71a27a5172743100ba71e3bccafe951d641f5fb4bc5fcfb4cf91fcb43b4"
                            required
                        >
                        </input>
                    </label>
                </form>
            </section>
        </main>
    )
}
