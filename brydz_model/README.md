# brydz_model

Minimal implementation of bridge game simulator. 
This is very early stage of development.


At current moment crate lacks features and documentation,
it is published as use case example for 
[`amfiteatr`](https://github.com/moroviintaas/amfiteatr)

## Example run

1. Prepare directory
```bash
mkdir -p ./run/brydz_model 
```

2. Generate biased distributions

```bash 
cargo run --release --bin generate \
  --format yaml --number 10000 \
  --output ./run/brydz_model/distributions_10k.yaml
```

3. Generate test games:
```bash 
cargo run --all-features --release  \
  --bin generate contract --format yaml  \
  --output ./run/brydz_model/test_contracts_100.yaml \
  --game-count 100 --upper-bound 4 --force-declarer force-north \
  --method biased \
  --probability-file ./run/brydz_model/distributions_1k.yaml
```
4. Generate model configuration:
```bash
cargo run --all-features --release  --bin model default
```
