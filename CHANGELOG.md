# egui_alignments changelog

All notable changes to this crate will be documented in this file.

## Unreleased

- Update egui to 0.35.0

## 0.3.8 - 2026-04-12

- Update egui to 0.34.0

## 0.3.4 - 2025-04-10

- Fixed: Wrapped layouts with stretch never unwrapping. [#7](https://github.com/a-littlebit/egui_alignments/issues/7)

## 0.3.3 - 2025-04-09

- Fixed: a wrapping row can take up full available height. [#6](https://github.com/a-littlebit/egui_alignments/issues/6)

## 0.3.2 - 2025-03-19

- Added `stretch` feature to align items in a container more flexibly.

- P.S. Where is 0.3.0? Actually 0.3.0 has been published on crates.io, but it seems I failed to push the related commits to GitHub. The has led to a version confusion. Anyway, 0.3.2 will be the latest version before 0.3.3 or 0.4.0 with interesting new features compared to 0.2.x and is compatible with later version of `egui` compared to 0.3.0. I'm sorry for the inconvenience. Just try 0.3.2!

## 0.2.7

- Update egui to 0.31.1

## 0.2.6

- Update egui to 0.31.0

## 0.2.5

- Update egui to 0.30.0

## 0.2.4

- Fix: column containers appear at incorrect position when overflowing

## 0.2.3 - 2024-09-30

- Add `container` module with container `Row` and `Column`

## 0.2.2 - 2024-09-28

- Update egui to 0.29.0

## 0.2.1 - 2024-09-27

- Fix: Nested alignments result in endless repaint and high CPU usage

## 0.2.0 - 2024-09-26

- Support alignments releative to the whole Ui
- Support allocate no space & content row & content column

## 0.1.0 - 2024-09-25

- First release
