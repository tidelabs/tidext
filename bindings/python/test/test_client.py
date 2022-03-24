import pytest
import os
import tidext.tidext

# TODO: can we share the client instance for all tests? sorry I never used Python before
async def get_client():
  builder = tidext.Builder(os.getenv('SUBSTRATE_URL'), os.getenv('STRONGHOLD_PATH'), os.getenv('STRONGHOLD_PASSWORD'))
  client = await builder.build()
  return client

@pytest.mark.asyncio
async def test_wallet():
  client = await get_client()
  print(client.get_account_id())
  assets = await client.all_assets()
  print('Wallet')
  for asset in assets:
    balance = await client.balance(asset.token_id)
    print(f'id: {asset.token_id}, token: {asset.metadata.name}, balance: {balance.available}')

@pytest.mark.asyncio
async def test_fee():
  client = await get_client()
  print(await client.get_regular_swap_fee())
  print(await client.get_market_maker_swap_fee())

@pytest.mark.asyncio
async def test_stake():
  client = await get_client()
  # `2` here is the Bitcoin Token ID; 1 is the amount and 1000000 is the duration
  extrinsic = await client.stake_extrinsic(2, 1, 1000000)
  fee = await client.extrinsic_cost(extrinsic)
  print(f'staking 1 satoshi, gas fee: {fee} TIDE')
  await client.submit_signed_extrinsic(extrinsic)
  print('stake extrinsic submitted')
