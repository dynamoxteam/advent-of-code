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
    
    let lastRound = rounds.slice(-1).pop()
    
    rounds.push(lastRound + 1)
    width = rounds.reduce((acc, i) => acc + i)
    sign = rounds.slice(-1).pop() % 2 ? 1 : -1
  }

  const coordinates = [
    x.reduce((acc, i) => acc + i, 0),
    y.reduce((acc, i) => acc + i, 0)
  ]

  debug('aoc:day3:coordinates')(position, coordinates)

  return coordinates
}

export const steps = position => {
  const [x, y] = coordinates(position)
  const steps = Math.abs(x) + Math.abs(y)

  debug('aoc:day3:steps')(position, [x, y], steps)

  return steps
}