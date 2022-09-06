# Arcalog

> **⚠ Note**  
> This repository does not yet include the full feature set of Arcalog as we are incrementally uploading code segments. A scientific paper describing the project is available as a [preprint](https://www.techrxiv.org/articles/preprint/Arcalog_Enhancing_Continuous_Integration_Systems_with_Assisted_Root_Cause_Analysis/20689594).

Arcalog can assist you with or automate your root cause analysis in CI or other log systems either as a standalone tool or by embedding it into your applications. It also provides additional tooling to download jobs from various log systems or add your own log files for analysis.

<p align="center">
  <img alt="Arcalog logo depicting an ark floating on a scroll" src="https://github.com/arcalot/.github/raw/main/branding/arcalog.png">
</p>

## Getting Started

The goal of Arcalog is to get started quickly and easily without a lengthy setup. If you have Rust installed, you can start gathering data locally from the Kubernetes CI by simply running `cargo run -- --collect prow` from the root folder of this repository. More gathering options will be supported soon. Please note that if you start collecting data without specifying a data folder, Arcalog will save the data in the current directory. You can change this by either adding `data: "yourdir"` to the config file or by passing the `--data yourdir` flag when running Arcalog.

```bash
git clone https://github.com/arcalot/arcalog.git
cd arcalog
cargo run -- --collect prow
```

 Type `cargo run -- --help` for further commands. You can also use the provided Dockerfile, as long as you provide a default `config.yaml`. If you would like to use a custom path to a YAML-formatted file via the `--config` flag, e.g.: `arcalog --config /path/to/config.yaml`, you will have to mount the config file into the container as well.

```bash
docker build -t arcalog .
docker run -v /path/to/datafolder:/data arcalog
```

### Start collecting data from Prow

If you want to contribute to development of Arcalog, the best way to get started is to have Rust installed and then iteratively check and test what you are doing. One example command if you want to collect some data including artifacts as you develop is `cargo run -- --bin cli --config config-prod.yaml --collect prow -a`.

### Run web server with UI

If you are looking to run the web server you need to add the `--http` flag. If you cloned this repository and have Rust installed, you could do so by executing `cargo run -- --http` from the root folder of this repository.

## Hints

If you specify a data folder in your configuration file, but want to test data collection to a temporary folder, you can run `arcalog --collect yoursource --data /your/tmp/path/`, which will take precedence over the data path in the configuration file. This is useful for testing data collection or an additional data source while keeping your production data intact.

---

By contributing to this project, you agree to follow our simple [code of conduct](https://github.com/arcalot/.github/blob/main/CODE_OF_CONDUCT.md) and [contribution guidelines](CONTRIBUTING.md).

If you have any questions, suggestions, or would like to report a bug, please use the [issues](https://github.com/arcalot/arcalog/issues) or contribute to the [discussions](https://github.com/arcalot/.github/discussions).