export const inverseCaptchaNext = (rawInput) => {
  const input = parseInput(rawInput)

  if (input.length === 0) {
    return 0
  }

  let matches = []

  for (let i = 0; i < input.length; i++) {
    const current = input[i]
    const nextIndex = i + 1
    const nextIndexCircle = nextIndex % input.length
    const next = input[nextIndexCircle]

    if (current === next) {
      matches.push(current)
    }
  }

  return matches.reduce((sum, a) => sum + a, 0)
}

export const inverseCaptchaHalfwayAround = (rawInput) => {
  const input = parseInput(rawInput)

  if (input.length === 0) {
    return 0
  }

  let matches = []

  for (let i = 0; i < input.length; i++) {
    const current = input[i]
    const nextIndex = i + input.length / 2
    const nextIndexCircle = nextIndex % input.length
    const next = input[nextIndexCircle]

    if (current === next) {
      matches.push(current)
    }
  }

  return matches.reduce((sum, a) => sum + a, 0)
}

/**
 * @todo Remove \n characters to allow multiline strings
 * @param string input 
 */
export const parseInput = (input) => {
  if (typeof input !== 'string') {
    throw new Error('A string was expected as input')
  }

  return input.split("").map(entry => parseInt(entry, 10))
}

export default inverseCaptchaNext