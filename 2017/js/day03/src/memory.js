import debug from 'debug'

export const coordinates = positionOneIndexed => {
  let rounds = [0], width = 0, sign = 1, x = [], y = []
  const position = positionOneIndexed - 1
  
  while ((x.length + y.length) < position) {
    if (x.length < width) {
      x.push(sign)
      continue
    } 
    
    if (y.length < width) {
      y.push(sign)
      continue
    } 
    
    let lastRound = last(rounds)
    rounds.push(lastRound + 1)
    
    width = sum(rounds)
    sign = last(rounds) % 2 ? 1 : -1
  }

  const coordinates = [sum(x), sum(y)]

  debug('aoc:day3:coordinates')(position, coordinates)

  return coordinates
}

export const steps = (position, origin = [0, 0]) => {
  const [x0, y0] = origin
  const [x1, y1] = coordinates(position)
  const steps = Math.abs(x1 - x0) + Math.abs(y1 - y0)

  debug('aoc:day3:steps')(position, [x1, y1], steps)

  return steps
}

export const sum = list => {
  return list.reduce((acc, i) => acc + i, 0)
}

export const last = list => {
  return list.slice(-1).pop()
}

export const circleGenerator = ([x, y]) => {
  const adjacent = [
    [x - 1, y + 1],
    [x + 0, y + 1],
    [x + 1, y + 1],
    [x - 1, y + 0],
    [x + 1, y + 0],
    [x - 1, y - 1],
    [x + 0, y - 1],
    [x + 1, y - 1]
  ]

  return adjacent.map(a => a.toString())
}

export const intersection = (a, b) => {
  let result = []

  a.forEach(cell => {
    if (b.indexOf(cell) >= 0) {
      result.push(cell)
    }
  })

  return result
}

export const stress = input => {
  let reverseIndex = {
    '0,0': 1
  }
  
  let stress = [1]
  let i = 2
  
  while (last(stress) <= input) {
    const point = coordinates(i)
    
    const key = point.toString()
    reverseIndex[key] = i
  
    const circle = circleGenerator(point)
    const adjacentKeys = intersection(circle, Object.keys(reverseIndex))
  
    const adjacentPositions = adjacentKeys.map(i => reverseIndex[i])
    const adjacents = adjacentPositions.map(a => stress[a - 1])
    const sumAdjacents = sum(adjacents)
    
    stress.push(sumAdjacents)
    i++
  }

  return last(stress)
}
