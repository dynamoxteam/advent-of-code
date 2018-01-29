import { expect } from 'chai'
import { 
  checkAnagram, 
  checkDuplication, 
  parseInput, 
  uniq 
} from '../src/entropy'

/* global describe, it */

describe('entropy', () => {
  describe('checkDuplication', () => {
    it('Should implement a checkDuplication method', () => {
      expect(checkDuplication).to.be.a('function')
    })

    it('Should be evaluated as valid "aa bb cc dd ee"', () => {
      expect(checkDuplication(['aa', 'bb', 'cc', 'dd', 'ee'])).to.be.true
    })

    it('Should be evaluated as invalid "aa bb cc dd aa"', () => {
      expect(checkDuplication(['aa', 'bb', 'cc', 'dd', 'aa'])).to.be.false
    })

    it('Should be evaluated as invalid "aa bb cc dd aaa"', () => {
      expect(checkDuplication(['aa', 'bb', 'cc', 'dd', 'aaa'])).to.be.true
    })
  })

  describe('checkAnagram', () => {
    it('Should implement a checkAnagram method', () => {
      expect(checkAnagram).to.be.a('function')
    })

    it('Should be evaluated as valid "abcde fghij"', () => {
      expect(checkAnagram(['abcde', 'fghij'])).to.be.true
    })

    it('Should be evaluated as invalid "abcde xyz ecdab"', () => {
      expect(checkAnagram(['abcde', 'xyz', 'ecdab'])).to.be.false
    })

    it('Should be evaluated as valid "a ab abc abd abf abj"', () => {
      expect(checkAnagram(['a', 'ab', 'abc', 'abd', 'abf', 'abj'])).to.be.true
    })

    it('Should be evaluated as valid "iiii oiii ooii oooi oooo"', () => {
      expect(checkAnagram(['iiii', 'oiii', 'ooii', 'oooi', 'oooo'])).to.be.true
    })

    it('Should be evaluated as invalid "oiii ioii iioi iiio"', () => {
      expect(checkAnagram(['oiii', 'ioii', 'iioi', 'iiio'])).to.be.false
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