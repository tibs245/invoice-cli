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
- [x] Auto-generate pdf from template
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

> Invoice ref is optionnal
>

```
cargo run -- --root-path ~/invoices/ -dddd customer get {invoice_ref}
```

### Create invoice

```
cargo run -- --root-path ~/invoices/ -dddd invoice create
```

### Get invoice

> Invoice ref is optionnal

```
cargo run -- --root-path ~/invoices/ -dddd invoice get {invoice_ref}
```

### Generate specific invoice

> Invoice ref is optionnal

```
cargo run -- --root-path ~/invoices/ -dddd generate {invoice_ref}
```

### Generate all invoices

> Generate and replace all invoice

```
cargo run -- --root-path ~/invoices/ -dddd generate-all
```

