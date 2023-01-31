import { Formatter } from "./formatter";
import { Checker } from "./checker";
import { API_PATH } from "../constants/index.js"

export class Fetcher {
    static async block() {
        let value = document.getElementById("getblock").value

        if (!Checker.isNeoTxidHash(value) && !Checker.isReasonableNumber(value)) {
            document.getElementById("getblock").ariaInvalid = "true"
            await new Promise(r => setTimeout(r, 2000));
            document.getElementById("getblock").ariaInvalid = ""
            return false
         }

        let res = await fetch(`${API_PATH}/block/${value}`)
        let block = await res.json()
        return Formatter.flattenObject(block)
    }

    static async transaction() {
        let value = document.getElementById("gettransaction").value

        if (!Checker.isNeoTxidHash(value)) {
            document.getElementById("gettransaction").ariaInvalid = "true"
            await new Promise(r => setTimeout(r, 2000));
            document.getElementById("gettransaction").ariaInvalid = ""
            return false
        }

        let res = await fetch(`${API_PATH}/transaction/${value}`)
        let tx = await res.json()
        return Formatter.flattenObject(tx)
    }

    static async blockTransactions() {
        let value = document.getElementById("getblocktransactions").value

        if (!Checker.isNeoTxidHash(value)) {
            document.getElementById("getblocktransactions").ariaInvalid = "true"
            await new Promise(r => setTimeout(r, 2000));
            document.getElementById("getblocktransactions").ariaInvalid = ""
            return false
        }

        let res = await fetch(`${API_PATH}/block/${value}/transactions`)
        let txList = await res.json()
        return Formatter.flattenObject(txList)
    }
}
