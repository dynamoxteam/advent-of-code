export const minMaxBased = input => {
  if (input.length === 0) {
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

export const evenlyDivisible = input => {
  if (input.length === 0) {
    return 0
  }

  return input.map(row => {
    let divisible = 0

    row.forEach(cell => {
      const match = fetchDivisibleResult(cell, row)
      divisible = match ? match : divisible
    })

    return divisible
  })
  .reduce((acc, cell) => acc + cell, 0)
}

export const fetchDivisibleResult = (input, row) => {
  let match = false

  row.forEach(cell => {
    if (input !== cell && input % cell === 0) {
      match = input / cell
    }
  })

  return match
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
