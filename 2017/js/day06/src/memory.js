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

export const bulk = (input, allowedDuplications = 1) => {
  debug('aoc:day6:bulk')(input)

  let memoryLoop = {}
  let duplicationAllowed = true
  let incremementalInput = input

  while (duplicationAllowed) {
    const data = distribute(incremementalInput)

    debug('aoc:day6:bulk')(data)
    
    if (!memoryLoop.hasOwnProperty(data.toString())) {
      memoryLoop[data.toString()] = {
        data: null,
        count: 0
      }
    }

    const { count } = memoryLoop[data.toString()]

    memoryLoop[data.toString()] = {
      data,
      count: count + 1
    }
    
    duplicationAllowed = count <= allowedDuplications - 1
    incremementalInput = data
  }

  let steps = 0

  for (let key in memoryLoop) {
    steps = steps + memoryLoop[key]['count']
  }

  return {
    memoryLoop,
    steps
  }
}