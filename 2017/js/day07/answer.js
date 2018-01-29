import debug from 'debug'
import input from './src/input'
import { parse } from './src/tower'

const parsedInput = input.split('\n')
  .map(cell => cell.trim())
  .map(program => parse(program))
  .filter(i => i.sub.length > 0)

let sub = []
parsedInput.forEach(p => p.sub.forEach(s => sub.push(s)))

/** 
 * @todo Move this logic into isDown method (for example)
 */
const step1 = parsedInput.filter(p => sub.indexOf(p.name) === -1)
  .pop()
  .name

const answer = {
  step1,
  step2: 0
}

debug('answer')('%O', answer)

export default answer