import { expect } from 'chai'
import { esc, parseInput } from '../src/maze'

/* global describe, it */

describe('maze', () => {
  describe('esc', () => {
    it('Should implement an "esc" method', () => {
      expect(esc).to.be.a('function')
    })

    it('Should retrieve 5', () => {
      expect(esc([0, 3, 0, 1, -3])).to.be.equal(5)
    })

    it('Should retrieve 5', () => {
      expect(esc([0, 0, 2])).to.be.equal(5)
    })

    it('Should retrieve 4', () => {
      expect(esc([0, -1])).to.be.equal(4)
    })

    it('Should retrieve 8', () => {
      expect(esc([0, 0, -2])).to.be.equal(8)
    })

    it('Should retrieve 11', () => {
      expect(esc([0, 0, 2, 0, -3])).to.be.equal(11)
    })
  })

  describe('parseInput', () => {
    it('Should implement a parseInput method', () => {
      expect(parseInput).to.be.a('function')
    })

    it('Should parse as a list', () => {
      const tpl = `1
        2
        3`

      expect(parseInput(tpl)).to.be.deep.equal([1, 2, 3])
    })
  })
})