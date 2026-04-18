debug:
    cargo build --profile dev
    cargo test

release:  
    cargo build --profile release

publish version:
    cargo fmt
    git add *
    git commit -m "Automatic formatting with cargo-fmt"
    cargo fix
    got add *
    git commit -m "Automatic fixes with cargo-fix"
    git tag {{version}}
    git push
    cargo publish
    
