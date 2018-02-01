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

export const steps = position => {
  const [x, y] = coordinates(position)
  const steps = Math.abs(x) + Math.abs(y)

  debug('aoc:day3:steps')(position, [x, y], steps)

  return steps
}

export const sum = list => {
  return list.reduce((acc, i) => acc + i, 0)
}

export const last = list => {
  return list.slice(-1).pop()
}