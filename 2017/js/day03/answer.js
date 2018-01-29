import debug from 'debug'
import { 
  steps,
  stress
} from './src/memory'

const input = 361527

const answer = {
  step1: steps(input),
  step2: stress(input)
}

debug('answer')('%O', answer)

export default answer