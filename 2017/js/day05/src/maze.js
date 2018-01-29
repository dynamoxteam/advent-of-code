import debug from 'debug'

export const esc = input => {

  debug('aoc:day5:esc')(input)

  let maze = [...input]
  let position = 0
  let steps = 0

  while (position < maze.length) {
    const nextPosition = position + maze[position]

    maze = [
      ...maze.slice(0, position), 
      maze[position] + 1, 
      ...maze.slice(position + 1)
    ]

    debug('aoc:day5:esc')(steps + 1, position, maze)

    position = nextPosition
    steps++
  }

  return steps
}

export const parseInput = input => {
  return input.split('\n').map(entry => parseInt(entry.trim(), 10))
}