# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2]
### Changed
- Criterion.rs is now stable-compatible!
- Criterion.rs now includes its own stable-compatible `black_box` function.
  Some benchmarks may now be affected by dead-code-elimination where they
  previously weren't and may have to be updated.
- Criterion.rs now uses `serde` to save results. Existing results files will
  be automatically removed when benchmarks are run.
- Redesigned the command-line output to highlight the important information
  and reduce noise.

### Added
- Running benchmarks with the variable "CRITERION_DEBUG" in the environment will
  cause Criterion.rs to generate extra debug output and save the gnuplot scripts
  alongside the generated plots.

### Fixed
- Don't panic on IO errors or gnuplot failures
- Fix generation of invalid gnuplot scripts when benchmarking over inputs and inputs include values <= 0.
- Bug where benchmarks would run one sample fewer than was configured.

### Removed
- Generated plots will no longer use log-scale.

## [0.1.1]
### Added
- A changelog file.
- Added a chapter to the book on how Criterion.rs collects and analyzes data.
- Added macro rules to generate a test harness for use with `cargo bench`.
  Benchmarks defined without these macros should continue to work.
- New contribution guidelines
- Criterion.rs can selectively run benchmarks. See the Command-line page for
more details

## 0.1.0 - 2017-12-02
### Added
- Initial release on Crates.io.


[Unreleased]: https://github.com/japaric/criterion.rs/compare/0.1.2...HEAD
[0.1.1]: https://github.com/japaric/criterion.rs/compare/0.1.0...0.1.1
[0.1.2]: https://github.com/japaric/criterion.rs/compare/0.1.1...0.1.2