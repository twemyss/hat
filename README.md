# Home Acuity Assessment Aid (HAT) 

Generate and serve PDFs containing random combinations of optotypes, for home monitoring of visual acuity.

_Please note that these are **not** validated tests of vision, but are assessment aids for use in clinical research._

## About

[![Build Status](https://travis-ci.org/twemyss/hat.svg?branch=master)](https://travis-ci.org/twemyss/hat) [![codecov](https://codecov.io/gh/twemyss/hat/branch/master/graph/badge.svg)](https://codecov.io/gh/twemyss/hat)

This code is a port (to Rust) of the code powering [homeacuitytest.org](https://homeacuitytest.org), which is a _research tool_ (not yet a validated test of vision) for at-home monitoring of patient visual acuity. The aim of this tool is to reduce digital exclusion of people without access to smartphones or computers, through producing a printable test which can be easily administered over the telephone.

This codebase generates and serves PDFs containing random arrangements of optotypes.

## Configuration

We suggest using the free web version of this tool, which we host at [homeacuitytest.org](https://homeacuitytest.org). However, you can also install and run this tool locally, although some Linux experience is required.

This codebase is a re-write of the code at homeacuitytest.org with a focus on extensibility (supporting new optotypes), speed, and backwards compatibility. **This codebase is not yet complete.** For the original code used for the publication, please email the authors via the contact form at [homeacuitytest.org](https://homeacuitytest.org).

## Running the HAT server

This server runs on Rust. To install Rust on a linux machine, follow the instructions [here](https://www.rust-lang.org/tools/install). Then, switch to Rust nightly by running:

```
rustup default nightly
```

To install and run the HAT server:

```
git clone https://github.com/twemyss/hat
cd hat
cargo run
```

You may wish to place the HAT server behind NGINX or Caddy reverse proxies in order to have SSL.

## References

Please note that the fonts included with this code are licenced under separate licences. To view the licences for the fonts, navigate to `/static/fonts`.
