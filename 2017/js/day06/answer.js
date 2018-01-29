import debug from 'debug'
import input from './src/input'
import { bulk } from './src/memory'

const parsedInput = input.split('\t').map(cell => parseInt(cell, 10))

const { steps: step1 } = bulk(parsedInput)
const { steps: step2 } = bulk(parsedInput, 2)

const answer = {
  step1,
  step2: step2 - step1
}

debug('answer')('%O', answer)

export default answer