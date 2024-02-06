# Invoice CLI

## Requirement

- Invoice-CLI package
- [Typst](https://github.com/typst/typst?tab=readme-ov-file#installation)

## Roadmap

- [x] Add main entities
- [ ] Add cli interface
  - [x] Initiate folder
  - [ ] Create customer
  - [ ] Create invoice
  - [ ] Cancel invoice
  - [ ] Show enterprise stats
  - Edit enterprise settings
- [ ] Split modules
- [ ] Add Swift module for Mac UI interface

## Quickstart

### First launch
```
cargo run -- --root-path ~/invoices/ -dddd init
```