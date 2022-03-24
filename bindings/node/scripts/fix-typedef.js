const fs = require('fs')
const path = require('path')

const CurrencyId = 'export type CurrencyId = number | undefined | null'
const BalanceInfo = 'export type BalanceInfo = string'

const typeDefPath = path.resolve(__dirname, '../index.d.ts')
const typeDef = fs.readFileSync(typeDefPath).toString()
  .replace('export interface', `${CurrencyId}\n\n${BalanceInfo}\n\nexport interface`)
fs.writeFileSync(typeDefPath, typeDef)
