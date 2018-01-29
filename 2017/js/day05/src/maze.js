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

    debug('aoc:day5:esc:verbose')(steps + 1, position, maze)

    position = nextPosition
    steps++
  }

  return steps
}

export const variantEsc = input => {
  debug('aoc:day5:variantEsc')(input)

  let maze = [...input]
  let position = 0
  let steps = 0

  let isLockedInWaze = true

  while (isLockedInWaze) {
    const nextPosition = position + maze[position]
    const offset = maze[position]
    const backward = offset >= 3

    maze[position] = offset + 1

    if (backward) {
      maze[position] = offset - 1
    }

    position = nextPosition
    steps++

    isLockedInWaze = position >= 0 && position < maze.length
  }

  return steps
}

export const parseInput = input => {
  return input.split('\n').map(entry => parseInt(entry.trim(), 10))
}