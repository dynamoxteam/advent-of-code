import debug from 'debug'

export const check = input => {
  const removeDuplicates = uniq(input)
  const isValid = input.length === removeDuplicates.length

  debug('aoc:day4:check')(input, isValid)

  return isValid
}

export const uniq = list => {

  const filtered = list.filter((entry, i) => {
    return list.indexOf(entry) === i
  })

  return filtered
}

export const parseInput = input => {
  return input.split('\n')
    .map(row => row.trim().split(' '))
}
