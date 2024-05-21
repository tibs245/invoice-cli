# Invoice CLI

## Description

Simple project to manage invoice with Rust modules.

- Be simple and readable to be ideal to learn and improve skill RUST
- Simple file management to be simple to use and reusable
  - Can be synchronizable with Cloud file system (Nextcloud, iCloud, Drive)
  - Can be versionnable with Git
- First interface with simple CLI

## Requirement

- Invoice-CLI packages
- [Typst](https://github.com/typst/typst?tab=readme-ov-file#installation)

## Roadmap

- [x] Add main entities
- [x] Add cli interface
  - [x] Initiate folder
  - [x] Create customer
  - [x] Create invoice
  - [x] Cancel invoice
  - [x] Show enterprise stats
  - [x] Edit enterprise settings
- [ ] Auto-generate pdf from template
- [ ] Split modules
- [ ] Add Swift module for Mac UI interface

## Quickstart

### First launch

```
cargo run -- --root-path ~/invoices/ -dddd init
```

### Create customer

```
cargo run -- --root-path ~/invoices/ -dddd customer create
```

### Get customer

```
cargo run -- --root-path ~/invoices/ -dddd customer get
```


### Create invoice

```
cargo run -- --root-path ~/invoices/ -dddd invoice create
```

### Get invoice

```
cargo run -- --root-path ~/invoices/ -dddd invoice get
```

