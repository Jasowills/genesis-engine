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
use std::collections::BTreeSet;
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
        #[arg(long)]
        source_id: Option<String>,
        #[arg(long)]
        licence: Option<String>,
        #[arg(long)]
        attribution: Option<String>,
    },
    #[command(name = "features")]
    FeaturesImport {
        project: PathBuf,
        file: PathBuf,
        #[arg(long)]
        source_id: Option<String>,
        #[arg(long)]
        licence: Option<String>,
        #[arg(long)]
        attribution: Option<String>,
    },
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

/// Source descriptor supplied by the caller. Defaults are deliberately
/// neutral: never attribute arbitrary user data to OpenStreetMap implicitly.
fn source_from_args(
    source_id: Option<String>,
    licence: Option<String>,
    attribution: Option<String>,
) -> SourceDescriptor {
    SourceDescriptor {
        id: source_id.unwrap_or_else(|| "local:user-provided".to_string()),
        publisher: "user-provided".to_string(),
        url: "local-file".to_string(),
        version: "n/a".to_string(),
        licence: match licence.unwrap_or_default().as_str() {
            "odbl" | "ODbL-1.0" => LicenceElection::Odbl10,
            "cc-by-4.0" | "CC-BY-4.0" => LicenceElection::CcBy40,
            "apache-2.0" => LicenceElection::Apache20,
            other if !other.is_empty() => LicenceElection::Other(other.to_string()),
            _ => LicenceElection::Other("UNSPECIFIED".to_string()),
        },
        attribution: attribution
            .unwrap_or_else(|| "User-provided data; no licence elected".to_string()),
        ..Default::default()
    }
}

fn metadata_path(project: &Path) -> PathBuf {
    project.join("metadata/project.json")
}

fn load_crs(project: &Path) -> Result<CrsMetadata, String> {
    let raw: Value = read_json(&metadata_path(project))?;
    serde_json::from_value(
        raw.get("crs")
            .cloned()
            .ok_or("project manifest has no CRS")?,
    )
    .map_err(|e| e.to_string())
}

fn load_source(project: &Path) -> Result<SourceDescriptor, String> {
    read_json(&project.join("metadata/source.json"))
}

fn save_source(project: &Path, source: &SourceDescriptor) -> Result<(), String> {
    std::fs::create_dir_all(project.join("metadata")).map_err(|e| e.to_string())?;
    std::fs::write(
        project.join("metadata/source.json"),
        serde_json::to_vec_pretty(source).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
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
    json!({"schema_version": OUTPUT_SCHEMA_VERSION, "command": command, "status": "error", "error": {"kind": "cli_error", "message": message}})
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
            let c = CrsMetadata {
                wkt2: Some(wkt2),
                vertical_datum: Some(datum),
                epoch: Some(epoch),
                ..CrsMetadata::default()
            };
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
            source_id,
            licence,
            attribution,
        } => {
            let source = source_from_args(source_id, licence, attribution);
            save_source(&project, &source)?;
            let crs = load_crs(&project)?;
            let dest = copy_source(&project, &file, "boundary.geojson")?;
            let bytes = std::fs::read(&dest).map_err(|e| e.to_string())?;
            let boundary =
                import_geojson(&bytes, &name, &source, &crs).map_err(|e| e.to_string())?;
            let manifest = json!({"schema_version": CURRENT_SCHEMA_VERSION, "name": boundary.name, "bbox": boundary.bbox, "id": boundary.id.to_string(), "checksum": boundary.source_checksum});
            std::fs::create_dir_all(project.join("worlds")).map_err(|e| e.to_string())?;
            std::fs::write(
                project.join("worlds/boundary.json"),
                serde_json::to_vec_pretty(&manifest).unwrap(),
            )
            .map_err(|e| e.to_string())?;
            Ok(ok("boundary_import", manifest))
        }
        Command::FeaturesImport {
            project,
            file,
            source_id,
            licence,
            attribution,
        } => {
            let source = source_from_args(source_id, licence, attribution);
            save_source(&project, &source)?;
            let crs = load_crs(&project)?;
            let dest = copy_source(&project, &file, "features.geojson")?;
            let bytes = std::fs::read(&dest).map_err(|e| e.to_string())?;
            let features = import_osm_geojson(&bytes, &source, &crs).map_err(|e| e.to_string())?;
            let objects = to_world_objects(&features, &source).map_err(|e| e.to_string())?;
            let world = WorldIr {
                schema_version: CURRENT_SCHEMA_VERSION,
                world_id: derive_id("world", "cli-project"),
                crs,
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
            let source = load_source(&project)?;
            let crs = load_crs(&project)?;
            let path = project.join("sources/features.geojson");
            let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
            let features = import_osm_geojson(&bytes, &source, &crs).map_err(|e| e.to_string())?;
            let mut meshes =
                generate_building_volumes(&features, &source, seed).map_err(|e| e.to_string())?;
            meshes.extend(
                generate_road_surfaces(&features, &source, seed, road_width)
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
            let selected_profile = profile(&profile_name)?;
            let world = load_ir(&project)?;
            let meshes = load_meshes(&project)?;
            let report = validate_world(&world, selected_profile, &meshes, false, "cli-1.0");
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
            let exported = export_glb(&meshes, &world, &[load_source(&project)?], &report)
                .map_err(|e| e.to_string())?;
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
                json!({
                    "schema_version": 1,
                    "world_id": world.world_id.to_string(),
                    "mesh_count": meshes.len(),
                    "ids": meshes.iter().map(|m| m.asset_id.to_string()).collect::<Vec<_>>(),
                    "reproducibility": {
                        "generator_version": meshes.first().map(|m| m.generator_version.clone()).unwrap_or_default(),
                        "seeds": meshes.iter().map(|m| m.generation_seed).collect::<Vec<_>>(),
                        "source_ids": meshes.iter().map(|m| m.provenance.source_id.clone()).collect::<BTreeSet<_>>().into_iter().collect::<Vec<_>>(),
                        "subject_hash": genesis_core::mesh_subject_hash(&meshes)
                    }
                }),
            ))
        }
        Command::JobStatus { project: _, id } => Ok(json!({
            "schema_version": OUTPUT_SCHEMA_VERSION,
            "command": "job_status",
            "status": "unsupported",
            "data": {"id": id, "note": "job orchestration is not implemented in Core yet (GE-0023 follow-up)"}
        })),
        Command::JobCancel { project: _, id } => Ok(json!({
            "schema_version": OUTPUT_SCHEMA_VERSION,
            "command": "job_cancel",
            "status": "unsupported",
            "data": {"id": id, "note": "job orchestration is not implemented in Core yet (GE-0023 follow-up)"}
        })),
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
