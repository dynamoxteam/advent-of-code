import debug from 'debug'
import input from './src/input'
import { bulk } from './src/memory'

const parsedInput = input.split('\t').map(cell => parseInt(cell, 10))

const { steps: step1 } = bulk(parsedInput)

const answer = {
  step1
}

debug('answer')('%O', answer)

export default answer