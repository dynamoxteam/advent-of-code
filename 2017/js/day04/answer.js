import debug from 'debug'
import input from './src/input'
import { checkAnagram, checkDuplication, parseInput } from './src/entropy'

const step1 = parseInput(input)
  .map(password => checkDuplication(password))
  .filter(isValid => isValid)

const step2 = parseInput(input)
  .map(password => checkAnagram(password))
  .filter(isValid => isValid)

const answer = {
  step1: step1.length,
  step2: step2.length
}

debug('answer')('%O', answer)

export default answer