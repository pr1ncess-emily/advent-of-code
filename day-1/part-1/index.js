const fs = require("fs")

const input = fs.readFileSync(__dirname + "/../input").toString()
const split = input.split("\n\n")
const entries = split.map(entry => {
    return entry.split("\n").map(line => parseInt(line))
})
const totals = entries.map(entry => entry.reduce((sum, val) => sum + val, 0))
console.log(totals.reduce((max, val) => (val > max) ? val : max, 0))