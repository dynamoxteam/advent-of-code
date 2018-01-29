import debug from 'debug'

export const distribute = input => {
  const max = input.slice()
    .sort((a, b) => a - b)
    .pop()

  const maxSlot = input.indexOf(max)
  const blocks = input[maxSlot]

  let base = [
    ...input.slice(0, maxSlot),
    0,
    ...input.slice(maxSlot + 1)
  ]

  debug('aoc:day6:distribute')(input, { maxSlot, blocks })

  for (let i = 1; i <= blocks; i++) {
    const slot = (maxSlot + i) % input.length
    base[slot]++
  }

  return base
}

export const bulk = input => {
  debug('aoc:day6:bulk')(input)

  let memoryLoop = {}
  let duplicationNotDetected = true
  let incremementalInput = input

  while (duplicationNotDetected) {
    const ans = distribute(incremementalInput)    
    duplicationNotDetected = !memoryLoop.hasOwnProperty(ans.toString())

    debug('aoc:day6:bulk')(ans)
    
    memoryLoop[ans.toString()] = ans
    incremementalInput = ans
  }

  return {
    memoryLoop,
    steps: Object.keys(memoryLoop).length + 1
  }
}