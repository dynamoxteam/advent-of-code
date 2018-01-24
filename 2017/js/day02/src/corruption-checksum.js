export const corruptionChecksum = rawInput => {
  const input = rawInput

  if (rawInput.length === 0) {
    return 0
  }

  return input.map(row => {
    const sorted = row.slice()
      .sort((a, b) => a - b)

    const min = sorted.slice(0, 1)
    const max = sorted.slice(sorted.length - 1)

    return max - min
  })
  .reduce((acc, p) => acc + p, 0)
}

export const parseInput = rawInput => {
  const input = rawInput.split("\n")
    .map(row => (
      row.trim()
        .split("\t")
        .map(char => parseInt(char, 10))
    ))

  return input
}
