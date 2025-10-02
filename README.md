# dz-pay-fees

Petit utilitaire en ligne de commande pour interagir avec le compte dépôt (PDA) d’un programme de distribution de revenus sur Solana.

## Installation
Dans le dépôt :

```bash
cargo build --release
```

Le binaire se trouve ensuite dans `target/release/dz-pay-fees`.

## Mettre le binaire dans le PATH
```bash
cargo install --path .
```

## Utilisation
La CLI expose trois sous-commandes : `pda`, `balance` et `send`.

Arguments globaux obligatoires :
- `--rpc-url` : URL du RPC (ex : `https://api.mainnet-beta.solana.com`)
- `--validator-identity` : clé publique (base58) de l’identité du validateur

### Afficher l’adresse du Deposit PDA
```bash
dz-pay-fees \
  --rpc-url https://api.mainnet-beta.solana.com \
  --validator-identity 2t53LvZfskcpXkdwLaBnfZLbNgyVHPu2BNFpcRBaEBhM \
  pda
```

### Consulter le solde du Deposit PDA
```bash
dz-pay-fees \
  --rpc-url https://api.mainnet-beta.solana.com \
  --validator-identity 2t53LvZfskcpXkdwLaBnfZLbNgyVHPu2BNFpcRBaEBhM \
  balance
```

### Envoyer des SOL vers le Deposit PDA
`send` prend un montant en SOL (positionnel) et requiert `--payer` (chemin vers le keypair JSON).

```bash
dz-pay-fees \
  --rpc-url https://api.mainnet-beta.solana.com \
  --validator-identity 2t53LvZfskcpXkdwLaBnfZLbNgyVHPu2BNFpcRBaEBhM \
  --payer /chemin/vers/payer.json \
  send 0.5
```

## Détails techniques
- Le Deposit PDA est dérivé via la seed "solana_validator_deposit" et l’identité du validateur, avec le `program_id` défini dans `REVENUE_DISTRIBUTION_PROGRAM_ID`.
- Les transactions sont envoyées via `RpcClient::send_and_confirm_transaction`.
- Les instructions `transfer` proviennent du crate `solana-system-interface`.

## Dépannage
- Solde insuffisant : approvisionnez le compte payeur.
- URL RPC invalide / indisponible : vérifiez `--rpc-url`.
- Erreurs de versions de crates : exécutez `cargo update` ou supprimez/regenérez `Cargo.lock` si nécessaire (pour un binaire d’application, `Cargo.lock` est normalement committé).

## Sécurité
- Ne commitez jamais de secrets/clé privée. Conservez le fichier keypair (`--payer`) hors du dépôt et protégez les permissions.
