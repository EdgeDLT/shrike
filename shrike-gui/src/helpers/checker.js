export class Checker {
    static isNeoAddress(s) {
        return (s.length === 34 && s[0] === "N" && /^[A-HJ-NP-Za-km-z1-9]*$/.test(s)) ? true : false
    }

    static isNeoScriptHash(s) {
        return (s.length === 42 && s.slice(0, 2) === "0x" === /^[A-Fa-f0-9]*$/.test(s.slice(2, 42))) ? true : false
    }

    static isNeoTxidHash(s) {
        return (s.length === 66 && s.slice(0, 2) === "0x" && /^[A-Fa-f0-9]*$/.test(s.slice(2, 66))) ? true : false
    }

    static isReasonableNumber(s) {
        return (s.length < 9 && /^[0-9]*$/.test(s)) ? true : false
    }
}
