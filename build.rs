use vergen_gitcl::{Emitter, GitclBuilder};

fn main() {
    let gitcl = GitclBuilder::all_git().unwrap();

    Emitter::default()
        .add_instructions(&gitcl)
        .expect("Failed to add build instructions.")
        .emit()
        .expect("Failed to emit by vergen.");
}
