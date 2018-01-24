import { expect } from 'chai'
import { corruptionChecksum, parseInput } from '../src/corruption-checksum'
import { input } from './input-test'

describe('Day2', () => {
  describe('parseInput', () => {
    it('Should implement a parseInput method', () => {
      expect(parseInput).to.be.a('function')
    })
  
    it('Should retrieve a list from a string', () => {
      const ml1 = `1	2
        3	4`
      const ml2 = `5	6
        7	8`

      expect(parseInput(ml1)).to.be.deep.equal([[1, 2], [3, 4]])
      expect(parseInput(ml2)).to.be.deep.equal([[5, 6], [7, 8]])
    })

    it('Should transform "input" string in a list', () => {
      const ans = parseInput(input)

      expect(ans).to.have.length(3)
      expect(ans.map(sub => sub.length)).to.be.deep.equal([16, 16, 16])
    })
  })

  describe('corruptionChecksum', () => {
    it('Should implement a corruptionChecksum method', () => {
      expect(corruptionChecksum).to.be.a('function')
    })

    it('Should return 10', () => {
      const ans = corruptionChecksum([[10, 20]])
      expect(ans).to.be.equal(10)
    })
  
    it('Should return 6', () => {
      const ans = corruptionChecksum([[1, 3], [5, 7], [9, 11]])
      expect(ans).to.be.equal(6)
    })

    it('Should return 6', () => {
      const ans = corruptionChecksum([[0, 2], [4, 6], [8, 10]])
      expect(ans).to.be.equal(6)
    })

    it('Should return 1549', () => {
      const ans = corruptionChecksum([
        [278, 1689, 250, 1512], 
        [160,	50,	55,	81]
      ])

      expect(ans).to.be.equal(1549)
    })

    it('Should return 0', () => {
      const ans = corruptionChecksum([])
      expect(ans).to.be.equal(0)
    })
  })
})