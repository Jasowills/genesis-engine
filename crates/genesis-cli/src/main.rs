//! Genesis headless CLI — thin adapter over Genesis Core (GE-0022).
//! No domain logic lives here; all CRS, provenance, validation and geometry
//! work is performed by `genesis-core`.

use clap::{Parser, Subcommand};
use genesis_core::{
    CURRENT_SCHEMA_VERSION, CrsMetadata, GeneratedGeometry, LicenceElection, LocalFrame,
    QualityProfile, SourceDescriptor, WorldIr, WorldLayer, check_export_ready, create_project,
    derive_id, export_glb, generate_building_volumes, generate_road_surfaces, import_geojson,
    import_glb, import_osm_geojson, to_world_objects, validate_world,
};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

const OUTPUT_SCHEMA_VERSION: u32 = 1;

#[derive(Parser)]
#[command(name = "genesis", about = "Genesis Engine headless command surface")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(name = "project")]
    ProjectCreate {
        path: PathBuf,
        name: String,
        #[arg(long, default_value = "GEOGCRS[\"WGS 84\"]")]
        wkt2: String,
        #[arg(long, default_value = "EGM2008")]
        datum: String,
        #[arg(long, default_value_t = 2024.5)]
        epoch: f64,
    },
    #[command(name = "boundary")]
    BoundaryImport {
        project: PathBuf,
        file: PathBuf,
        name: String,
    },
    #[command(name = "features")]
    FeaturesImport { project: PathBuf, file: PathBuf },
    #[command(name = "generate")]
    Generate {
        project: PathBuf,
        seed: u64,
        #[arg(long, default_value_t = 8.0)]
        road_width: f64,
    },
    #[command(name = "validate")]
    Validate {
        project: PathBuf,
        #[arg(long, default_value = "preview")]
        profile: String,
    },
    #[command(name = "export")]
    Export { project: PathBuf, output: PathBuf },
    #[command(name = "scene")]
    Scene { project: PathBuf },
    #[command(name = "job")]
    JobStatus { project: PathBuf, id: String },
    #[command(name = "job-cancel")]
    JobCancel { project: PathBuf, id: String },
}

fn source() -> SourceDescriptor {
    SourceDescriptor {
        id: "osm:fixture:ikeja".to_string(),
        publisher: "OpenStreetMap contributors".to_string(),
        url: "https://www.openstreetmap.org/copyright".to_string(),
        version: "2026-11-27".to_string(),
        licence: LicenceElection::Odbl10,
        attribution: "© OpenStreetMap contributors".to_string(),
        ..Default::default()
    }
}

fn crs() -> CrsMetadata {
    CrsMetadata {
        wkt2: Some("GEOGCRS[\"WGS 84\"]".to_string()),
        epoch: Some(2024.5),
        vertical_datum: Some("EGM2008".to_string()),
        ..CrsMetadata::default()
    }
}

fn profile(name: &str) -> Result<QualityProfile, String> {
    match name {
        "preview" => Ok(QualityProfile::Preview),
        "game_ready" => Ok(QualityProfile::GameReady),
        "high_fidelity" => Ok(QualityProfile::HighFidelity),
        "survey_reference" => Ok(QualityProfile::SurveyReference),
        other => Err(format!("unknown profile '{other}'")),
    }
}

fn ok(command: &str, data: Value) -> Value {
    json!({"schema_version": OUTPUT_SCHEMA_VERSION, "command": command, "status": "ok", "data": data})
}

fn fail(command: &str, message: String) -> Value {
    json!({"schema_version": OUTPUT_SCHEMA_VERSION, "command": command, "status": "error", "error": {"kind": "core_error", "message": message}})
}

fn copy_source(project: &Path, file: &Path, name: &str) -> Result<PathBuf, String> {
    std::fs::create_dir_all(project.join("sources")).map_err(|e| e.to_string())?;
    let dest = project.join("sources").join(name);
    std::fs::copy(file, &dest).map_err(|e| e.to_string())?;
    Ok(dest)
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
}

fn load_ir(project: &Path) -> Result<WorldIr, String> {
    read_json(&project.join("worlds/world.json"))
}

fn load_meshes(project: &Path) -> Result<Vec<GeneratedGeometry>, String> {
    read_json(&project.join("payloads/meshes.json"))
}

fn run(cli: Cli) -> Result<Value, String> {
    match cli.command {
        Command::ProjectCreate {
            path,
            name,
            wkt2,
            datum,
            epoch,
        } => {
            let mut c = crs();
            c.wkt2 = Some(wkt2);
            c.vertical_datum = Some(datum);
            c.epoch = Some(epoch);
            let project = create_project(&path, &name, c).map_err(|e| e.to_string())?;
            Ok(ok(
                "project_create",
                json!({"path": path, "id": project.manifest().id, "schema_version": project.manifest().schema_version}),
            ))
        }
        Command::BoundaryImport {
            project,
            file,
            name,
        } => {
            let dest = copy_source(&project, &file, "boundary.geojson")?;
            let bytes = std::fs::read(&dest).map_err(|e| e.to_string())?;
            let boundary =
                import_geojson(&bytes, &name, &source(), &crs()).map_err(|e| e.to_string())?;
            let manifest = json!({"schema_version": 1, "name": boundary.name, "bbox": boundary.bbox, "id": boundary.id.to_string(), "checksum": boundary.source_checksum});
            std::fs::create_dir_all(project.join("worlds")).map_err(|e| e.to_string())?;
            std::fs::write(
                project.join("worlds/boundary.json"),
                serde_json::to_vec_pretty(&manifest).unwrap(),
            )
            .map_err(|e| e.to_string())?;
            Ok(ok("boundary_import", manifest))
        }
        Command::FeaturesImport { project, file } => {
            let dest = copy_source(&project, &file, "features.geojson")?;
            let bytes = std::fs::read(&dest).map_err(|e| e.to_string())?;
            let features =
                import_osm_geojson(&bytes, &source(), &crs()).map_err(|e| e.to_string())?;
            let objects = to_world_objects(&features, &source()).map_err(|e| e.to_string())?;
            let world = WorldIr {
                schema_version: CURRENT_SCHEMA_VERSION,
                world_id: derive_id("world", "cli-project"),
                crs: crs(),
                local_frame: LocalFrame::default(),
                layers: vec![WorldLayer {
                    id: derive_id("layer", "osm-features"),
                    objects,
                }],
            };
            std::fs::create_dir_all(project.join("worlds")).map_err(|e| e.to_string())?;
            std::fs::write(
                project.join("worlds/world.json"),
                world.to_bytes().map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
            Ok(ok(
                "features_import",
                json!({"feature_count": features.len(), "world_id": world.world_id.to_string()}),
            ))
        }
        Command::Generate {
            project,
            seed,
            road_width,
        } => {
            let path = project.join("sources/features.geojson");
            let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
            let features =
                import_osm_geojson(&bytes, &source(), &crs()).map_err(|e| e.to_string())?;
            let mut meshes =
                generate_building_volumes(&features, &source(), seed).map_err(|e| e.to_string())?;
            meshes.extend(
                generate_road_surfaces(&features, &source(), seed, road_width)
                    .map_err(|e| e.to_string())?,
            );
            std::fs::create_dir_all(project.join("payloads")).map_err(|e| e.to_string())?;
            std::fs::write(
                project.join("payloads/meshes.json"),
                serde_json::to_vec_pretty(&meshes).unwrap(),
            )
            .map_err(|e| e.to_string())?;
            Ok(ok(
                "generate",
                json!({"mesh_count": meshes.len(), "seed": seed, "generator_version": "procedural-v0.1"}),
            ))
        }
        Command::Validate {
            project,
            profile: profile_name,
        } => {
            let world = load_ir(&project)?;
            let meshes = load_meshes(&project)?;
            let report = validate_world(&world, profile(&profile_name)?, &meshes, false, "cli-1.0");
            std::fs::create_dir_all(project.join("validation")).map_err(|e| e.to_string())?;
            std::fs::write(
                project.join("validation/report.json"),
                serde_json::to_vec_pretty(&report).unwrap(),
            )
            .map_err(|e| e.to_string())?;
            Ok(ok(
                "validate",
                json!({"passed": report.passed, "accuracy_claim": report.geographic_accuracy_claim, "warnings": report.warnings, "subject_hash": report.subject_hash}),
            ))
        }
        Command::Export { project, output } => {
            let world = load_ir(&project)?;
            let meshes = load_meshes(&project)?;
            let report: genesis_core::ValidationReport =
                read_json(&project.join("validation/report.json"))?;
            check_export_ready(&report).map_err(|e| e.to_string())?;
            let exported =
                export_glb(&meshes, &world, &[source()], &report).map_err(|e| e.to_string())?;
            std::fs::write(&output, &exported.bytes).map_err(|e| e.to_string())?;
            std::fs::write(
                output.with_extension("attribution.json"),
                &exported.attribution_sidecar,
            )
            .map_err(|e| e.to_string())?;
            let imported = import_glb(&exported.bytes).map_err(|e| e.to_string())?;
            Ok(ok(
                "export",
                json!({"output": output, "gltf_version": imported.version, "mesh_count": imported.mesh_count, "triangle_count": imported.triangle_count}),
            ))
        }
        Command::Scene { project } => {
            let world = load_ir(&project)?;
            let meshes = load_meshes(&project)?;
            Ok(ok(
                "scene",
                json!({"schema_version": 1, "world_id": world.world_id.to_string(), "mesh_count": meshes.len(), "ids": meshes.iter().map(|m| m.asset_id.to_string()).collect::<Vec<_>>()}),
            ))
        }
        Command::JobStatus { project: _, id } => Ok(ok(
            "job_status",
            json!({"id": id, "status": "unsupported", "note": "job orchestration is not implemented in Core yet (GE-0023 follow-up)"}),
        )),
        Command::JobCancel { project: _, id } => Ok(ok(
            "job_cancel",
            json!({"id": id, "status": "unsupported", "note": "job orchestration is not implemented in Core yet (GE-0023 follow-up)"}),
        )),
    }
}

fn main() {
    let command_name = std::env::args().nth(1).unwrap_or_default();
    match run(Cli::parse()) {
        Ok(value) => println!("{}", serde_json::to_string_pretty(&value).unwrap()),
        Err(message) => {
            eprintln!(
                "{}",
                serde_json::to_string_pretty(&fail(&command_name, message)).unwrap()
            );
            std::process::exit(1);
        }
    }
}
