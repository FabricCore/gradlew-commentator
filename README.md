# Gradlew Commentator

This tool is used to generate [**Yarnwrap**](https://github.com/FabricCore/yarnwrap).

Working principle:

- Comment out lines containing errors.
- Delete files where line number 2 is contain errors, as there are issues with the class definition.

## Usage

1. Install gradlew-commentator
```sh
cargo install --git https://github.com/FabricCore/gradlew-commentator
```
2. CD into the directory where `./gradlew` is at, and run command.
```sh
gradlew-commentator
```
3. Wait until process completes.

> Note you might have to specify JAVA_HOME (version >= 21) for it to work.
