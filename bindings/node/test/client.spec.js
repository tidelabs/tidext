const { Builder } = require('..')

async function getClient() {
  const builder = new Builder(process.env.SUBSTRATE_URL, process.env.STRONGHOLD_PATH, process.env.STRONGHOLD_PASSWORD)
  const client = await builder.build()
  return client
}

describe('Client', function () {
  this.timeout(0)

  describe('client APIs', function () {
    it('runs', async function () {
      const client = await getClient()
      console.log(await client.allAssets())
      console.log(await client.getAccountId())
      console.log(await client.getAccountIdSs58())
      console.log(await client.getRegularSwapFee())
      console.log(await client.getMarketMakerSwapFee())
      console.log(await client.balance())
    })
  })
})
