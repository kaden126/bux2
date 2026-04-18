debug:
    cargo build --profile dev
    cargo test

release:  
    cargo build --profile release

publish version:
    cargo fmt
    cargo fix
    git add *
    git tag {{version}}
    git push tags
    cargo publish
    
