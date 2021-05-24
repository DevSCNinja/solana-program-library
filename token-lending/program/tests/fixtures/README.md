# fixtures

### SOL / USDC / SRM Dex Accounts

On devnet, via [Mango Markets](https://github.com/blockworks-foundation/mango-client-ts/blob/main/src/ids.json)
```shell
solana config set --url https://devnet.solana.com
solana account 8RJA4WhY2Ei48c4xANSgPoqw7DU7mRgvg6eqJS3tvLEN --output-file sol_usdt_dex_market.bin
solana account 7r9frDKnGgEPLsYq8112N3Q8A3AQEyTANj6YheoW6i6x --output-file sol_usdt_dex_market_bids.bin
solana account 4LDcoKPrBMjDYUW2gyKBy6cfsoEqw5ESWMeJdyRGcpvG --output-file sol_usdt_dex_market_asks.bin
solana account CRLpSnSf7JkoJi9tUnz55R2FoTCrDDkWxQMU6uSVBQgc --output-file srm_usdt_dex_market.bin
solana account 2GKVz4GGxJaxS6kc9ziejcbyx85TyFtgbDppp2bkhrLS --output-file srm_usdt_dex_market_bids.bin
solana account AVfVvsPoUaW62wayj1EFzFHZATJVijEkpiTbirdNCZym --output-file srm_usdt_dex_market_asks.bin
```

On mainnet-beta:
```shell
solana config set --url https://api.mainnet-beta.solana.com
solana account 9wFFyRfZBsuAha4YcuxcXLKwMxJR43S7fPfQLusDBzvT --output-file sol_usdc_dex_market.bin
solana account 14ivtgssEBoBjuZJtSAPKYgpUK7DmnSwuPMqJoVTSgKJ --output-file sol_usdc_dex_market_bids.bin
solana account CEQdAFKdycHugujQg9k2wbmxjcpdYZyVLfV9WerTnafJ --output-file sol_usdc_dex_market_asks.bin
solana account ByRys5tuUWDgL73G8JBAEfkdFf8JWBzPBDHsBVQ5vbQA --output-file srm_usdc_dex_market.bin
solana account AuL9JzRJ55MdqzubK4EutJgAumtkuFcRVuPUvTX39pN8 --output-file srm_usdc_dex_market_bids.bin
solana account 8Lx9U9wdE3afdqih1mCAXy3unJDfzSaXFqAvoLMjhwoD --output-file srm_usdc_dex_market_asks.bin
```