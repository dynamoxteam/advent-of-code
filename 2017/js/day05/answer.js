import debug from 'debug'
import input from './src/input'
import { esc, parseInput, variantEsc } from './src/maze'

const answer = {
  step1: esc(parseInput(input)),
  step2: variantEsc(parseInput(input))
}

debug('answer')('%O', answer)

export default answer