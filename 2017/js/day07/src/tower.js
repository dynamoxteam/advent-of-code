import debug from 'debug'

export const parse = metadata => {
  const re = /([a-z]+) \(([0-9]+)\)(?:[ ->]*)*([a-z, ]+)*/
  const [input, name, weight, subStr] = re.exec(metadata)
  const sub = subStr ? subStr.split(',').map(s => s.trim()) : []

  debug('aoc:day7:parse')(input, [name, weight, sub])

  return {
    name,
    weight: parseInt(weight),
    sub
  }
}