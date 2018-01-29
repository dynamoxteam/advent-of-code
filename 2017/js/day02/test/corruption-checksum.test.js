import { expect } from 'chai'
import { input } from './input-test'
import { 
  evenlyDivisible, 
  fetchDivisibleResult,
  minMaxBased, 
  parseInput 
} from '../src/corruption-checksum'

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

  describe('minMaxBased', () => {
    it('Should implement a minMaxBased method', () => {
      expect(minMaxBased).to.be.a('function')
    })

    it('Should return 10', () => {
      const ans = minMaxBased([[10, 20]])
      expect(ans).to.be.equal(10)
    })
  
    it('Should return 6', () => {
      const ans = minMaxBased([[1, 3], [5, 7], [9, 11]])
      expect(ans).to.be.equal(6)
    })

    it('Should return 6', () => {
      const ans = minMaxBased([[0, 2], [4, 6], [8, 10]])
      expect(ans).to.be.equal(6)
    })

    it('Should return 1549', () => {
      const ans = minMaxBased([
        [278, 1689, 250, 1512], 
        [160,	50,	55,	81]
      ])

      expect(ans).to.be.equal(1549)
    })

    it('Should return 0', () => {
      const ans = minMaxBased([])
      expect(ans).to.be.equal(0)
    })
  })

  describe('fetchDivisibleResult', () => {
    it('Should implement a fetchDivisibleResult method', () => {
      expect(fetchDivisibleResult).to.be.a('function')
    })

    it('Should return 4', () => {
      const ans = fetchDivisibleResult(8, [5, 9, 2, 8])
      expect(ans).to.be.equal(4)
    })
  })

  describe('evenlyDivisible', () => {
    it('Should implement a evenlyDivisible method', () => {
      expect(evenlyDivisible).to.be.a('function')
    })

    it('Should return 9', () => {
      const ans = evenlyDivisible([[5, 9, 2, 8], [9, 4, 7, 3], [3, 8, 6, 5]])
      expect(ans).to.be.equal(9)
    })
  
    it('Should return 0', () => {
      const ans = evenlyDivisible([])
      expect(ans).to.be.equal(0)
    })
  })
})