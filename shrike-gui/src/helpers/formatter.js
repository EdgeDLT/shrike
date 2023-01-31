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
}
