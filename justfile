_default:
    @just --list

# Runs the CLI with the provided arguments
run:
    cargo run -- $(ARGS)

# Bump version
bump VERSION:
    git cliff --bump
    git add .
    git commit -m "release: version {{VERSION}}"
    git push
    git tag v{{VERSION}}
    git push --tags

# Generate changelog for next version number
changelog:
    git cliff --bump -o CHANGELOG.md

# Get next version
next-version:
    git cliff --bumped-version

# Generate a new release
release VERSION:
    @just changelog
    @just bump {{VERSION}}
