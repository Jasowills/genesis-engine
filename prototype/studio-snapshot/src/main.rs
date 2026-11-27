//! Throwaway QA probe: turn a CLI-generated project into a Studio snapshot JSON
//! file that can be injected into the running React shell for product review.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage: studio-snapshot <world.json> <meshes.json> <out.json>".into());
    }
    let world = std::fs::read(&args[1])?;
    let meshes = std::fs::read(&args[2])?;
    let snapshot = genesis_studio::scene_from_documents(&world, &meshes)?;
    std::fs::write(&args[3], serde_json::to_vec_pretty(&snapshot)?)?;
    println!("{}", args[3]);
    Ok(())
}
