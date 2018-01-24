import debug from 'debug'
import { steps } from './src/memory'

const input = 361527

const answer = {
  step1: steps(input)
}

debug('answer')('%O', answer)

export default answer