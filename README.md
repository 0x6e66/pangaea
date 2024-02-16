# A rust module to to access [PANGAEA](https://www.pangaea.de/) (meta)data
[![Build](https://github.com/0x6e66/pangaea/actions/workflows/rust.yml/badge.svg)](https://github.com/0x6e66/pangaea/actions/workflows/rust.yml)
[![Crate](https://img.shields.io/crates/v/pangaea.svg)](https://crates.io/crates/pangaea)
[![Documentation](https://img.shields.io/docsrs/pangaea?label=docs.rs)](https://docs.rs/pangaea)
![License](https://img.shields.io/crates/l/pangaea)

## Description
This crate aims to provide interoperability between rust and the data publisher [PANGAEA](https://www.pangaea.de/). This crate provides mainly two type to get metadata from [PANGAEA](https://www.pangaea.de/)-datasets:
1. [Metadata](./src/metadata/metadatatype.rs) => A type that contains every information that is provided by [PANGAEA](https://www.pangaea.de/) and matches their [MetaData.xsd](https://ws.pangaea.de/schemas/pangaea/MetaData.xsd)-schema
2. [Dataset](./src/dataset/datasettype.rs) => A striped down version of [Metadata](./src/metadata/metadatatype.rs). This type only contains a selected amount of information of [Metadata](./src/metadata/metadatatype.rs).

This crate also provides the opportunity to download the actual data associated with a dataset.

Examples can be found in [examples](./examples/)