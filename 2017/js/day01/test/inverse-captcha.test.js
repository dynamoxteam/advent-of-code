import { expect } from 'chai'
import { 
  inverseCaptchaNext, 
  inverseCaptchaHalfwayAround, 
  parseInput 
} from '../src/inverse-captcha'

describe('Day1', () => {
  describe('parseInput', () => {
    it('Should implement a parseInput method', () => {
      expect(parseInput).to.be.a('function')
    })
  
    it('Should retrieve a list from a string', () => {
      expect(parseInput('1234')).to.be.deep.equal([1, 2, 3, 4])
      expect(parseInput('5678')).to.be.deep.equal([5, 6, 7, 8])
    })

    it('Should throw errors if input is not a String', () => {
      expect(() => parseInput([1, 2, 3])).to.throw
      expect(() => parseInput({ a: 1, b: 1 })).to.throw
    })
  })

  describe('inverseCaptchaNext', () => {
    it('Should implement a inverseCaptchaNext method', () => {
      expect(inverseCaptchaNext).to.be.a('function')
    })
  
    it('Should return 3', () => {
      const ans = inverseCaptchaNext('1122')
      expect(ans).to.be.equal(3)
    })
  
    it('Should return 4', () => {
      const ans = inverseCaptchaNext('1111')
      expect(ans).to.be.equal(4)
    })
  
    it('Should return 0', () => {
      const ans = inverseCaptchaNext('1234')
      expect(ans).to.be.equal(0)
    })
  
    it('Should return 9', () => {
      const ans = inverseCaptchaNext('91212129')
      expect(ans).to.be.equal(9)
    })
  
    it('Should return 0', () => {
      const ans = inverseCaptchaNext('')
      expect(ans).to.be.equal(0)
    })
  
    it('Should return 18', () => {
      const ans = inverseCaptchaNext('82393664534558127269567731851345949')
      expect(ans).to.be.equal(18)
    })
  })

  describe('inverseCaptchaHalfwayAround', () => {
    it('Should implement a inverseCaptchaHalfwayAround method', () => {
      expect(inverseCaptchaHalfwayAround).to.be.a('function')
    })
  
    it('Should return 6', () => {
      const ans = inverseCaptchaHalfwayAround('1212')
      expect(ans).to.be.equal(6)
    })

    it('Should return 0', () => {
      const ans = inverseCaptchaHalfwayAround('1221')
      expect(ans).to.be.equal(0)
    })

    it('Should return 4', () => {
      const ans = inverseCaptchaHalfwayAround('123425')
      expect(ans).to.be.equal(4)
    })

    it('Should return 12', () => {
      const ans = inverseCaptchaHalfwayAround('123123')
      expect(ans).to.be.equal(12)
    })

    it('Should return 4', () => {
      const ans = inverseCaptchaHalfwayAround('12131415')
      expect(ans).to.be.equal(4)
    })

    it('Should return 0', () => {
      const ans = inverseCaptchaHalfwayAround('')
      expect(ans).to.be.equal(0)
    })
  })
})