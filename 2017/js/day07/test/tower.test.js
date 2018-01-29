import { expect } from 'chai'
import { parse } from '../src/tower'

/* global describe, it */

describe('tower', () => {
  describe('parse', () => {
    it('Should implement a parse method', () => {
      expect(parse).to.be.a('function')
    })

    it('Should parse "rxivjo (206) -> mewof, hrncqs, qgfstpq"', () => {
      const programData = 'rxivjo (206) -> mewof, hrncqs, qgfstpq'

      expect(parse(programData)).to.be.deep.equal({
        name: 'rxivjo',
        weight: 206,
        sub: ['mewof', 'hrncqs', 'qgfstpq']
      })
    })

    it('Should parse "jlbcwrl (93)"', () => {
      const programData = 'jlbcwrl (93)'

      expect(parse(programData)).to.be.deep.equal({
        name: 'jlbcwrl',
        weight: 93,
        sub: []
      })
    })
  })
})