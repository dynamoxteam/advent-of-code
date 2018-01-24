import { expect } from 'chai'
import { coordinates, steps } from '../src/memory'

describe('Day3', () => {
  describe('coordinates', () => {
    it('Should implement a coordinates method', () => {
      expect(coordinates).to.be.a('function')
    })

    it('Should retrieve (0, 0)', () => {
      expect(coordinates(1)).to.be.deep.equal([0, 0])
    })

    it('Should retrieve many expected points', () => {
      expect(coordinates(2)).to.be.deep.equal([1, 0])
      expect(coordinates(3)).to.be.deep.equal([1, 1])
      expect(coordinates(4)).to.be.deep.equal([0, 1])
      expect(coordinates(5)).to.be.deep.equal([-1, 1])
      expect(coordinates(10)).to.be.deep.equal([2, -1])
      expect(coordinates(16)).to.be.deep.equal([-1, 2])
    })
  })
  
  describe('steps', () => {
    it('Should implement a steps method', () => {
      expect(steps).to.be.a('function')
    })

    it('Should retrieve 0', () => {
      expect(steps(1)).to.be.equal(0)
    })

    it('Should retrieve 12', () => {
      expect(steps(12)).to.be.equal(3)
    })

    it('Should retrieve 13', () => {
      expect(steps(13)).to.be.equal(4)
    })

    it('Should retrieve 23', () => {
      expect(steps(23)).to.be.equal(2)
    })

    it('Should retrieve 1024', () => {
      expect(steps(1024)).to.be.equal(31)
    })
  })
})