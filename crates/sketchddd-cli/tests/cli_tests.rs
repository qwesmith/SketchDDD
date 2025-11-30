//! CLI integration tests for SketchDDD.

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use std::fs;

/// Get the sketchddd binary command
fn sketchddd() -> Command {
    Command::cargo_bin("sketchddd").unwrap()
}

// =============================================================
// Help and Version Tests
// =============================================================

#[test]
fn test_help() {
    let mut cmd = sketchddd();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Build Domain Models Visually or with Code"));
}

#[test]
fn test_version() {
    let mut cmd = sketchddd();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("sketchddd"));
}

#[test]
fn test_check_help() {
    let mut cmd = sketchddd();
    cmd.args(["check", "--help"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Validate a SketchDDD model file"));
}

#[test]
fn test_init_help() {
    let mut cmd = sketchddd();
    cmd.args(["init", "--help"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Initialize a new SketchDDD project"));
}

// =============================================================
// Check Command Tests
// =============================================================

#[test]
fn test_check_valid_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.sddd");

    fs::write(&file_path, r#"
        context Commerce {
            objects { Customer, Order }
        }
    "#).unwrap();

    let mut cmd = sketchddd();
    cmd.args(["check", file_path.to_str().unwrap()]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No issues found"));
}

#[test]
fn test_check_empty_context() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("empty.sddd");

    fs::write(&file_path, r#"
        context Empty {
        }
    "#).unwrap();

    let mut cmd = sketchddd();
    cmd.args(["check", file_path.to_str().unwrap()]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No issues found"));
}

#[test]
fn test_check_nonexistent_file() {
    let mut cmd = sketchddd();
    cmd.args(["check", "nonexistent.sddd"]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read file"));
}

#[test]
fn test_check_invalid_syntax() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("invalid.sddd");

    fs::write(&file_path, r#"
        this is not valid syntax {{{
    "#).unwrap();

    let mut cmd = sketchddd();
    cmd.args(["check", file_path.to_str().unwrap()]);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Parse error"));
}

#[test]
fn test_check_with_entities() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("entities.sddd");

    fs::write(&file_path, r#"
        context Commerce {
            entity Customer {
                id: UUID
                name: String
            }

            entity Order {
                id: UUID
                status: OrderStatus
            }

            enum OrderStatus = Pending | Confirmed | Shipped
        }
    "#).unwrap();

    let mut cmd = sketchddd();
    cmd.args(["check", file_path.to_str().unwrap()]);
    cmd.assert()
        .success();
}

#[test]
fn test_check_with_morphisms() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("morphisms.sddd");

    fs::write(&file_path, r#"
        context Commerce {
            objects { Customer, Order }

            morphisms {
                placedBy: Order -> Customer
            }
        }
    "#).unwrap();

    let mut cmd = sketchddd();
    cmd.args(["check", file_path.to_str().unwrap()]);
    cmd.assert()
        .success();
}

#[test]
fn test_check_quiet_mode() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("quiet.sddd");

    fs::write(&file_path, r#"
        context Test {
        }
    "#).unwrap();

    let mut cmd = sketchddd();
    cmd.args(["-v", "quiet", "check", file_path.to_str().unwrap()]);
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn test_check_verbose_mode() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("verbose.sddd");

    fs::write(&file_path, r#"
        context Test {
            objects { A, B }
        }
    "#).unwrap();

    let mut cmd = sketchddd();
    cmd.args(["-v", "verbose", "check", file_path.to_str().unwrap()]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Parsed"));
}

#[test]
fn test_check_json_format() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("json.sddd");

    fs::write(&file_path, r#"
        context Test {
        }
    "#).unwrap();

    let mut cmd = sketchddd();
    cmd.args(["check", "--format", "json", file_path.to_str().unwrap()]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[]")); // Empty issues array
}

// =============================================================
// Init Command Tests
// =============================================================

#[test]
fn test_init_minimal() {
    let temp_dir = tempfile::tempdir().unwrap();
    let project_name = "TestProject";
    let project_dir = temp_dir.path().join(project_name);

    let mut cmd = sketchddd();
    cmd.current_dir(temp_dir.path());
    cmd.args(["init", project_name]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Created"))
        .stdout(predicate::str::contains(".sddd"));

    // Check that files were created
    assert!(project_dir.exists());
    assert!(project_dir.join("testproject.sddd").exists());
    assert!(project_dir.join(".gitignore").exists());
}

#[test]
fn test_init_ecommerce_template() {
    let temp_dir = tempfile::tempdir().unwrap();
    let project_name = "EcommerceProject";

    let mut cmd = sketchddd();
    cmd.current_dir(temp_dir.path());
    cmd.args(["init", project_name, "--template", "ecommerce"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("e-commerce"));

    // Verify the template content
    let file_path = temp_dir.path().join(project_name).join("ecommerceproject.sddd");
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("entity Customer"));
    assert!(content.contains("entity Order"));
    assert!(content.contains("value Money"));
}

#[test]
fn test_init_microservices_template() {
    let temp_dir = tempfile::tempdir().unwrap();
    let project_name = "MicroProject";

    let mut cmd = sketchddd();
    cmd.current_dir(temp_dir.path());
    cmd.args(["init", project_name, "--template", "microservices"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("microservices"));

    // Verify the template content
    let file_path = temp_dir.path().join(project_name).join("microproject.sddd");
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("context Orders"));
    assert!(content.contains("context Inventory"));
    assert!(content.contains("context Shipping"));
    assert!(content.contains("map OrdersToInventory"));
}

#[test]
fn test_init_quiet_mode() {
    let temp_dir = tempfile::tempdir().unwrap();
    let project_name = "QuietProject";

    let mut cmd = sketchddd();
    cmd.current_dir(temp_dir.path());
    cmd.args(["-v", "quiet", "init", project_name]);
    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty());

    // Files should still be created
    assert!(temp_dir.path().join(project_name).exists());
}

// =============================================================
// Export Command Tests
// =============================================================

#[test]
fn test_export_to_stdout() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("export.sddd");

    fs::write(&file_path, r#"
        context Commerce {
            entity Order {
                id: UUID
            }
        }
    "#).unwrap();

    let mut cmd = sketchddd();
    cmd.args(["export", file_path.to_str().unwrap()]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"contexts\""))
        .stdout(predicate::str::contains("Commerce"));
}

#[test]
fn test_export_to_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let input_path = temp_dir.path().join("input.sddd");
    let output_path = temp_dir.path().join("output.json");

    fs::write(&input_path, r#"
        context Test {
            objects { A, B }
        }
    "#).unwrap();

    let mut cmd = sketchddd();
    cmd.args([
        "export",
        input_path.to_str().unwrap(),
        "--output",
        output_path.to_str().unwrap()
    ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Exported to"));

    // Verify output file
    assert!(output_path.exists());
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("Test"));
}

// =============================================================
// Context Map Tests
// =============================================================

#[test]
fn test_check_with_context_map() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("context_map.sddd");

    fs::write(&file_path, r#"
        context Commerce {
            objects { Order, Customer }
        }

        context Shipping {
            objects { Shipment, Recipient }
        }

        map CommerceToShipping: Commerce -> Shipping {
            pattern: CustomerSupplier
            mappings {
                Order -> Shipment
                Customer -> Recipient
            }
        }
    "#).unwrap();

    let mut cmd = sketchddd();
    cmd.args(["check", file_path.to_str().unwrap()]);
    cmd.assert()
        .success();
}

// =============================================================
// Stub Command Tests (ensure they don't crash)
// =============================================================

#[test]
fn test_codegen_stub() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("codegen.sddd");

    fs::write(&file_path, r#"
        context Test {
        }
    "#).unwrap();

    let mut cmd = sketchddd();
    cmd.args(["codegen", file_path.to_str().unwrap()]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("not yet implemented"));
}

#[test]
fn test_viz_stub() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("viz.sddd");

    fs::write(&file_path, r#"
        context Test {
        }
    "#).unwrap();

    let mut cmd = sketchddd();
    cmd.args(["viz", file_path.to_str().unwrap()]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("not yet implemented"));
}

#[test]
fn test_serve_stub() {
    let mut cmd = sketchddd();
    cmd.args(["serve"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("not yet implemented"));
}

#[test]
fn test_import_stub() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("import.json");

    fs::write(&file_path, "{}").unwrap();

    let mut cmd = sketchddd();
    cmd.args(["import", file_path.to_str().unwrap()]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("not yet implemented"));
}

#[test]
fn test_diff_stub() {
    let temp_dir = tempfile::tempdir().unwrap();
    let file1 = temp_dir.path().join("old.sddd");
    let file2 = temp_dir.path().join("new.sddd");

    fs::write(&file1, "context A {}").unwrap();
    fs::write(&file2, "context B {}").unwrap();

    let mut cmd = sketchddd();
    cmd.args(["diff", file1.to_str().unwrap(), file2.to_str().unwrap()]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("not yet implemented"));
}
