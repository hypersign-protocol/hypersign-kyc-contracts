# How to run Localnet of nibirun chain

## 1. Setup Nibiru chain

1. Clone Nibiru chain

```
git clone https://github.com/NibiruChain/nibiru.git
cd nibiru
git checkout v1.0.3-rc2
```

2. Run a localnet 

```
sh ./contrib/scripts/localnet.sh
```

3. Stop the chain and run the following config commands:

```
sed -i 's/output = "json"/output = "text"/' $HOME/.nibid/config/client.toml
sed -i -E 's|cors_allowed_origins = \[\]|cors_allowed_origins = \[\"\*\"\]|g' $HOME/.nibid/config/config.toml
```

4. Start the chain:

```
nibid start
```

5. Add the following mnemonic to your Keplr Wallet (This is taken from `localnet.sh` script from Nibiru chain repo)

```
guard cream sadness conduct invite crumble clock pudding hole grit liar hotel maid produce squeeze return argue turtle know drive eight casino maze host
```