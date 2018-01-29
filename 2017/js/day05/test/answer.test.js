import { expect } from 'chai'
import answer from '../answer'

/* global describe, it */

describe('Answer', () => {
  it('Should only exercise the answer module', () => {
    expect(answer).to.be.a('object')
    expect(answer).to.have.property('step1').to.be.equal(358131)
    expect(answer).to.have.property('step2').to.be.equal(25558839)
  })
})