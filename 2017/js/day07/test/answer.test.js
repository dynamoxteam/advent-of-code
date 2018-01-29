import { expect } from 'chai'
import { parse } from '../src/tower'
import answer from '../answer'

/* global describe, it */

describe('Answer', () => {
  it('Should exercise the anwer acquirement', () => {
    const input = `pbga (66)
      xhth (57)
      ebii (61)
      havc (66)
      ktlj (57)
      fwft (72) -> ktlj, cntj, xhth
      qoyq (66)
      padx (45) -> pbga, havc, qoyq
      tknk (41) -> ugml, padx, fwft
      jptl (61)
      ugml (68) -> gyxo, ebii, jptl
      gyxo (61)
      cntj (57)`

    const parsedInput = input.split('\n')
      .map(cell => cell.trim())
      .map(program => parse(program))
      .filter(i => i.sub.length > 0)

    let sub = []
    parsedInput.forEach(p => p.sub.forEach(s => sub.push(s)))

    const name = parsedInput.filter(p => sub.indexOf(p.name) === -1)
      .pop()
      .name

    expect(name).to.be.equal('tknk')
  })

  it('Should only exercise the answer module', () => {
    expect(answer).to.be.a('object')
    expect(answer).to.have.property('step1').to.be.equal('gynfwly')
    expect(answer).to.have.property('step2').to.be.equal(0)
  })
})