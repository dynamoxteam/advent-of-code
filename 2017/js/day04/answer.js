import debug from 'debug'
import input from './src/input'
import { check, parseInput } from './src/entropy'

const checking = parseInput(input)
  .map(password => check(password))
  .filter(isValid => isValid)

const answer = {
  step1: checking.length
}

debug('answer')('%O', answer)

export default answer