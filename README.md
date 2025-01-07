# mol_parser

Parses mol2 files

Probably also mol and xyz with minimal changes?

Cargo.toml:

```
parsemol2 = { git = "https://github.com/ivnsch/mol_parser" }
```

```rust
let file = "assets/benzene.mol2";
let mol_res = parse_mol2(&file, 1).await;
assert!(mol_res.is_ok());

let mol = mol_res.unwrap();
assert_eq!(12, mol.atoms.len());
assert_eq!(12, mol.bonds.len());
```
