const fs = require("fs")

const input = fs.readFileSync(__dirname + "/../input").toString()
const split = input.split("\n\n")
const entries = split.map(entry => {
    return entry.split("\n").map(line => parseInt(line))
})
const totals = entries.map(entry => entry.reduce((sum, val) => sum + val, 0))
const topThree = totals.sort((a, b) => b - a).slice(0, 3)
console.log(topThree[0] + topThree[1] + topThree[2])