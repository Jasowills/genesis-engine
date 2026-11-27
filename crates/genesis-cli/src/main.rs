//! Genesis headless CLI — thin adapter over Genesis Core (GE-0022).
//! No domain logic lives here; all CRS, provenance, validation and geometry
//! work is performed by `genesis-core`.

use clap::{Parser, Subcommand};
use genesis_core::{
    CURRENT_SCHEMA_VERSION, CrsMetadata, GeneratedGeometry, LicenceElection, LocalFrame,
    Provenance, QualityProfile, SourceDescriptor, WorldIr, WorldLayer, WorldObject,
    apply_run_metadata, check_export_ready, create_project, derive_id, export_glb,
    generate_building_volumes_with_origin, generate_markings, generate_road_surfaces_with_origin,
    generate_sidewalks, generate_terrain, generate_vegetation, import_geojson, import_glb,
    import_osm_geojson, to_world_objects, validate_world,
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
        /// Directory containing dem.json, markings.geojson, sidewalks.geojson and vegetation.geojson
        #[arg(long)]
        layers: Option<PathBuf>,
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

/// Build a source descriptor for a bundled layer fixture. The fixture's own
/// attribution and licence text travel with the generated mesh.
fn layer_source(id: &str, attribution: &str, licence: Option<&str>) -> SourceDescriptor {
    SourceDescriptor {
        id: id.to_string(),
        publisher: "Genesis layer fixture".to_string(),
        url: "local-fixture".to_string(),
        version: "2026-11-28".to_string(),
        licence: licence
            .map(str::to_string)
            .map(LicenceElection::Other)
            .unwrap_or_default(),
        attribution: attribution.to_string(),
        ..Default::default()
    }
}

fn fixture_attribution(bytes: &[u8]) -> Result<String, String> {
    let value: Value = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    value
        .get("attribution")
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| "layer fixture has no attribution".to_string())
}

/// Generate secondary layers from an explicit fixture directory. Every source
/// passes the lawful-source gate inside Core before any geometry is emitted.
fn load_layers(
    dir: &Path,
    project: &Path,
    project_source: &SourceDescriptor,
    seed: u64,
) -> Result<(Vec<GeneratedGeometry>, Vec<SourceDescriptor>), String> {
    let mut registered_sources = vec![project_source.clone()];

    let dem_bytes = std::fs::read(dir.join("dem.json")).map_err(|e| e.to_string())?;
    let dem = genesis_core::DemGrid::from_bytes(&dem_bytes).map_err(|e| e.to_string())?;
    let dem_source = layer_source(
        "local:fixture:dem",
        &dem.attribution,
        Some("synthetic-demo"),
    );
    registered_sources.push(dem_source.clone());
    let mut meshes = vec![generate_terrain(&dem, &dem_source).map_err(|e| e.to_string())?];

    let markings_bytes = std::fs::read(dir.join("markings.geojson")).map_err(|e| e.to_string())?;
    let markings_source = layer_source(
        "local:fixture:markings",
        &fixture_attribution(&markings_bytes)?,
        Some("synthetic-demo"),
    );
    registered_sources.push(markings_source.clone());
    meshes.extend(generate_markings(&markings_bytes, &markings_source).map_err(|e| e.to_string())?);

    let sidewalks_bytes =
        std::fs::read(dir.join("sidewalks.geojson")).map_err(|e| e.to_string())?;
    meshes.extend(generate_sidewalks(&sidewalks_bytes, project_source).map_err(|e| e.to_string())?);

    let vegetation_bytes =
        std::fs::read(dir.join("vegetation.geojson")).map_err(|e| e.to_string())?;
    let vegetation_source = layer_source(
        "local:fixture:vegetation",
        &fixture_attribution(&vegetation_bytes)?,
        Some("synthetic-demo"),
    );
    registered_sources.push(vegetation_source.clone());
    meshes.push(
        generate_vegetation(&vegetation_bytes, &vegetation_source).map_err(|e| e.to_string())?,
    );

    let metadata = project.join("metadata");
    std::fs::create_dir_all(&metadata).map_err(|e| e.to_string())?;
    std::fs::write(
        metadata.join("layer_sources.json"),
        serde_json::to_vec_pretty(&registered_sources).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    apply_run_metadata(&mut meshes, seed, "layers-v0.1");
    Ok((meshes, registered_sources))
}

/// Register layer meshes as World IR objects so Studio can select them and
/// export can verify their provenance against registered sources.
fn register_layer_objects(
    project: &Path,
    meshes: &mut [GeneratedGeometry],
    sources: &[SourceDescriptor],
) -> Result<(), String> {
    let path = project.join("worlds/world.json");
    let mut world = load_ir(project)?;
    let existing: BTreeSet<String> = world
        .layers
        .iter()
        .flat_map(|layer| layer.objects.iter())
        .map(|object| object.id.to_string())
        .collect();
    let by_id: std::collections::BTreeMap<String, &SourceDescriptor> = sources
        .iter()
        .map(|source| (source.id.clone(), source))
        .collect();
    let mut added = 0;
    for mesh in meshes.iter_mut() {
        let id = mesh.asset_id.to_string();
        if existing.contains(&id) {
            mesh.source_object_ids = vec![mesh.asset_id.clone()];
            continue;
        }
        let source = by_id
            .get(mesh.provenance.source_id.as_str())
            .ok_or_else(|| {
                format!(
                    "layer mesh {id} has unregistered source {}",
                    mesh.provenance.source_id
                )
            })?;
        let mut provenance = Provenance::from(*source);
        provenance.source_feature_ids = mesh.provenance.source_feature_ids.clone();
        mesh.source_object_ids = vec![mesh.asset_id.clone()];
        let object = WorldObject {
            id: mesh.asset_id.clone(),
            object_type: format!("layer:{:?}", mesh.kind),
            geographic_position: [mesh.origin_lon_lat[0], mesh.origin_lon_lat[1], 0.0],
            local_position: [0.0, 0.0, 0.0],
            evidence: mesh.evidence,
            provenance,
            dimensions: Default::default(),
            generation_seed: Some(mesh.generation_seed),
            dependencies: Vec::new(),
            validation_state: Default::default(),
            payloads: Vec::new(),
        };
        if let Some(layer) = world.layers.first_mut() {
            layer.objects.push(object);
            added += 1;
        }
    }
    if added > 0 {
        std::fs::write(
            &path,
            serde_json::to_vec_pretty(&world).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
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
    let dir = project.join("metadata");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let target = dir.join("source.json");
    assert_no_symlink(project, &target)?;
    let temporary = dir.join("source.json.tmp");
    std::fs::write(
        &temporary,
        serde_json::to_vec_pretty(source).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    std::fs::rename(&temporary, &target).map_err(|e| e.to_string())
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

/// Refuse to write through symlinks: an untrusted project bundle must not be
/// able to redirect source copies outside the project root (SECURITY_MODEL.md).
fn assert_no_symlink(root: &Path, target: &Path) -> Result<(), String> {
    let mut current = root.to_path_buf();
    for component in target
        .strip_prefix(root)
        .map_err(|e| e.to_string())?
        .components()
    {
        current.push(component);
        let is_symlink = std::fs::symlink_metadata(&current)
            .map(|metadata| metadata.file_type().is_symlink())
            .unwrap_or(false);
        if is_symlink {
            return Err(format!(
                "refusing to write through symlink: {}",
                current.display()
            ));
        }
    }
    Ok(())
}

fn copy_source(project: &Path, file: &Path, name: &str) -> Result<PathBuf, String> {
    let dest = project.join("sources").join(name);
    assert_no_symlink(project, &dest)?;
    std::fs::create_dir_all(project.join("sources")).map_err(|e| e.to_string())?;
    assert_no_symlink(project, &dest)?;
    std::fs::copy(file, &dest).map_err(|e| e.to_string())?;
    Ok(dest)
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
}

fn load_ir(project: &Path) -> Result<WorldIr, String> {
    // Use Core's version-gated parser, never a raw deserialise: a hand-edited
    // or legacy IR document must be rejected with a structured error.
    let bytes = std::fs::read(project.join("worlds/world.json")).map_err(|e| e.to_string())?;
    WorldIr::from_bytes(&bytes).map_err(|e| e.to_string())
}

/// Build the product local frame from the imported boundary bbox centre
/// (ADR-0008). Without a boundary, derive the origin from the imported features.
fn load_frame(project: &Path, features: &[genesis_core::OsmFeature]) -> Result<LocalFrame, String> {
    let path = project.join("worlds/boundary.json");
    if !path.exists() {
        let [lon, lat] = genesis_core::frame_origin_for_features(features);
        return Ok(if lon == 0.0 && lat == 0.0 {
            LocalFrame::default()
        } else {
            LocalFrame::from_geodetic(lon, lat, 0.0, "features-local")
        });
    }
    let raw: Value = read_json(&path)?;
    let bbox = raw
        .get("bbox")
        .and_then(Value::as_array)
        .ok_or("boundary manifest has no bbox")?;
    let lon = (bbox.first().and_then(Value::as_f64).unwrap_or(0.0)
        + bbox.get(2).and_then(Value::as_f64).unwrap_or(0.0))
        / 2.0;
    let lat = (bbox.get(1).and_then(Value::as_f64).unwrap_or(0.0)
        + bbox.get(3).and_then(Value::as_f64).unwrap_or(0.0))
        / 2.0;
    Ok(LocalFrame::from_geodetic(lon, lat, 0.0, "ikeja-local"))
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
            let crs = load_crs(&project)?;
            let bytes = std::fs::read(&file).map_err(|e| e.to_string())?;
            let boundary =
                import_geojson(&bytes, &name, &source, &crs).map_err(|e| e.to_string())?;
            copy_source(&project, &file, "boundary.geojson")?;
            save_source(&project, &source)?;
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
            let crs = load_crs(&project)?;
            let bytes = std::fs::read(&file).map_err(|e| e.to_string())?;
            let features = import_osm_geojson(&bytes, &source, &crs).map_err(|e| e.to_string())?;
            let objects = to_world_objects(&features, &source).map_err(|e| e.to_string())?;
            let world = WorldIr {
                schema_version: CURRENT_SCHEMA_VERSION,
                world_id: derive_id("world", "cli-project"),
                crs,
                local_frame: load_frame(&project, &features)?,
                layers: vec![WorldLayer {
                    id: derive_id("layer", "osm-features"),
                    objects,
                }],
            };
            copy_source(&project, &file, "features.geojson")?;
            save_source(&project, &source)?;
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
            layers,
        } => {
            let source = load_source(&project)?;
            let crs = load_crs(&project)?;
            let path = project.join("sources/features.geojson");
            let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
            let features = import_osm_geojson(&bytes, &source, &crs).map_err(|e| e.to_string())?;
            let frame = load_frame(&project, &features)?;
            let origin_lon_lat = frame.origin_lon_lat;
            let mut meshes =
                generate_building_volumes_with_origin(&features, &source, seed, origin_lon_lat)
                    .map_err(|e| e.to_string())?;
            meshes.extend(
                generate_road_surfaces_with_origin(
                    &features,
                    &source,
                    seed,
                    road_width,
                    origin_lon_lat,
                )
                .map_err(|e| e.to_string())?,
            );
            let layers_enabled = layers.is_some();
            if let Some(directory) = layers {
                let (mut layer_meshes, layer_sources) =
                    load_layers(&directory, &project, &source, seed)?;
                register_layer_objects(&project, &mut layer_meshes, &layer_sources)?;
                meshes.extend(layer_meshes);
            }
            std::fs::create_dir_all(project.join("payloads")).map_err(|e| e.to_string())?;
            std::fs::write(
                project.join("payloads/meshes.json"),
                serde_json::to_vec_pretty(&meshes).unwrap(),
            )
            .map_err(|e| e.to_string())?;
            Ok(ok(
                "generate",
                json!({"mesh_count": meshes.len(), "seed": seed, "generator_version": "procedural-v0.1", "layer_generators": layers_enabled}),
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
            let mut sources = vec![load_source(&project)?];
            let layer_sources_path = project.join("metadata/layer_sources.json");
            if layer_sources_path.exists() {
                let extra: Vec<SourceDescriptor> = read_json(&layer_sources_path)?;
                sources.extend(extra);
            }
            let exported =
                export_glb(&meshes, &world, &sources, &report).map_err(|e| e.to_string())?;
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
                        "generator_versions": meshes.iter().map(|m| m.generator_version.clone()).collect::<BTreeSet<_>>().into_iter().collect::<Vec<_>>(),
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
    let arguments: Vec<_> = std::env::args_os().collect();
    let command_name = arguments
        .get(1)
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_default();
    match run(Cli::parse_from(arguments)) {
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
