import { expect } from 'chai'
import answer from '../answer'

describe('Answer', () => {
  it('Should only exercise the answer module', () => {
    expect(answer).to.be.a('object')
    expect(answer).to.have.property('step1').to.be.equal(42378)
    expect(answer).to.have.property('step2').to.be.equal(246)
  })
})