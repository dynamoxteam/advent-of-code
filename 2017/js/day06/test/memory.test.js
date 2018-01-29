import { expect } from 'chai'
import { bulk, distribute } from '../src/memory'

/* global describe, it */

describe('memory', () => {
  describe('distribute', () => {
    it('Should implement a distribute method', () => {
      expect(distribute).to.be.a('function')
    })

    it('Should exercise distribute method [0, 2, 7, 0]', () => {
      expect(distribute([0, 2, 7, 0])).to.be.deep.equal([2, 4, 1, 2])
    })

    it('Should exercise distribute method [2, 4, 1, 2]', () => {
      expect(distribute([2, 4, 1, 2])).to.be.deep.equal([3, 1, 2, 3])
    })

    it('Should exercise distribute method [3, 1, 2, 3]', () => {
      expect(distribute([3, 1, 2, 3])).to.be.deep.equal([0, 2, 3, 4])
    })

    it('Should exercise distribute method [0, 2, 3, 4]', () => {
      expect(distribute([0, 2, 3, 4])).to.be.deep.equal([1, 3, 4, 1])
    })

    it('Should exercise distribute method [1, 3, 4, 1]', () => {
      expect(distribute([1, 3, 4, 1])).to.be.deep.equal([2, 4, 1, 2])
    })
  })

  describe('bulk', () => {
    it('Should implement a bulk method', () => {
      expect(bulk).to.be.a('function')
    })

    it('Should exercise bulk method', () => {
      const ans = bulk([0, 2, 7, 0])
      expect(ans.steps).to.be.equal(5)
    })

    it('Should exercise bulk method with second parameter', () => {
      const { steps: firstDuplication } = bulk([0, 2, 7, 0], 1)
      const { steps: secondDuplication } = bulk([0, 2, 7, 0], 2)

      expect(secondDuplication - firstDuplication).to.be.equal(4)
    })
  })
})