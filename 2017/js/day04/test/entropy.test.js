import { expect } from 'chai'
import { check, parseInput, uniq } from '../src/entropy'

/* global describe, it */

describe('entropy', () => {
  describe('check', () => {
    it('Should implement a check method', () => {
      expect(check).to.be.a('function')
    })

    it('Should be evaluated as valid "aa bb cc dd ee"', () => {
      expect(check(['aa', 'bb', 'cc', 'dd', 'ee'])).to.be.true
    })

    it('Should be evaluated as invalid "aa bb cc dd aa"', () => {
      expect(check(['aa', 'bb', 'cc', 'dd', 'aa'])).to.be.false
    })

    it('Should be evaluated as invalid "aa bb cc dd aaa"', () => {
      expect(check(['aa', 'bb', 'cc', 'dd', 'aaa'])).to.be.true
    })
  })

  describe('uniq', () => {
    it('Should implement a uniq method', () => {
      expect(uniq).to.be.a('function')
    })

    it('Should return some expected values', () => {
      expect(uniq([1, 2, 3])).to.be.deep.equal([1, 2, 3])
      expect(uniq([1, 2, 3, 2])).to.be.deep.equal([1, 2, 3])
      expect(uniq([5, 5, 6, 7])).to.be.deep.equal([5, 6, 7])
    })
  })

  describe('parseInput', () => {

    it('Should implement a parseInput method', () => {
      expect(parseInput).to.be.a('function')
    })

    it('Should match some results', () => {
      const input = `nyot babgr babgr
        iix ewj rojvbkk
        isjur jppvano vctnpjp
        ojufqke gpd olzirc`
      
      parseInput(input)

      expect(parseInput(input)).to.be.deep.equal([
        ['nyot', 'babgr', 'babgr'],
        ['iix', 'ewj', 'rojvbkk'],
        ['isjur', 'jppvano', 'vctnpjp'],
        ['ojufqke', 'gpd', 'olzirc']
      ])
    })
  })
})