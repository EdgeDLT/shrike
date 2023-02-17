export class Formatter {

    static formatJson(json) {
        return JSON.stringify(json, null, 0)
    }

    static flattenObject(obj) {
        let arr = Object.entries(obj)

        return arr.map(el => {
            if (
                el[0] === "witnesses" ||
                el[0] === "signers" ||
                el[0] === "stack_result" ||
                el[0] === "notifications" ||
                el[0] === "transactions"
                ) {
                return [el[0], Formatter.formatJson(el[1])]
            } else {
                return [el[0], el[1]]
            }
        })
    }

    static getAddresses(transferList) {
        let addresses = []
        for (const transfer of transferList) {
            for (const nep17 of transfer.nep17_transfers) {

              if (!addresses.includes(nep17.from.slice(0,5))) {
                addresses.push(nep17.from.slice(0,5))
              }

              if (!addresses.includes(nep17.to.slice(0,5))) {
                addresses.push(nep17.to.slice(0,5))
              }
            }
        }
        return addresses
    }

    static getTransfers(transferList) {
        let transfers = []
        for (const transfer of transferList) {
            for (const nep17 of transfer.nep17_transfers) {
                transfers.push({to: nep17.to.slice(0, 5), from: nep17.from.slice(0, 5), amount: nep17.amount, asset: nep17.contract})
            }
        }
        return transfers
    }

    static parseIfHsl(cssString) {
        if (cssString.includes("hsl")) {
            return Formatter.hslToHex(cssString)
        } else {
            return cssString
        }
    }

    static parseHsl(rawHsl) {
        let hslArr = rawHsl.split("(")[1].split(")")[0].split(",")
        let h = Number(hslArr[0].split("deg")[0].trim())
        let s = Number(hslArr[1].split("%")[0].trim())
        let l = Number(hslArr[2].split("%")[0].trim())
        return [h, s, l]
    }

    static hslToHex(rawHsl) {
        let hsl = Formatter.parseHsl(rawHsl)
        let h = hsl[0]
        let s = hsl[1]
        let l = hsl[2]

        l /= 100
        const a = s * Math.min(l, 1 - l) / 100
        const f = n => {
            const k = (n + h / 30) % 12
            const color = l - a * Math.max(Math.min(k - 3, 9 - k, 1), -1)
            return Math.round(255 * color).toString(16).padStart(2, '0')
        }
        return `#${f(0)}${f(8)}${f(4)}`
    }
}
