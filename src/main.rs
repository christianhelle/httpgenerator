use std::fs;
use std::path::Path;
use std::process::ExitCode;
use std::time::Instant;

use anyhow::Context;
use clap::CommandFactory;
use httpgenerator::cli::Cli;
use httpgenerator::generator::{generate, GeneratorSettings};
use httpgenerator::openapi::{load_document, statistics};

fn main() -> ExitCode {
    if std::env::args_os().len() == 1 {
        let mut command = Cli::command();
        let _ = command.print_help();
        println!();
        return ExitCode::SUCCESS;
    }

    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error:\n{error:#}");
            ExitCode::from(1)
        }
    }
}

fn run() -> anyhow::Result<()> {
    let started = Instant::now();
    let mut cli = Cli::parse_args();

    display_header(cli.no_logging);

    println!("🔍 Loading OpenAPI specification...");
    let document = load_document(&cli.openapi_path).with_context(|| {
        format!(
            "Failed to load OpenAPI specification from {}",
            cli.openapi_path
        )
    })?;

    if !cli.skip_validation {
        println!("🔍 Validating OpenAPI specification...");
        let stats = statistics(&document);
        println!("✅ OpenAPI specification parsed successfully");
        println!(
            "📊 OpenAPI Statistics: path items={}, operations={}, parameters={}, request bodies={}, responses={}, links={}, callbacks={}, schemas={}",
            stats.path_items,
            stats.operations,
            stats.parameters,
            stats.request_bodies,
            stats.responses,
            stats.links,
            stats.callbacks,
            stats.schemas
        );
    }

    acquire_azure_token_if_requested(&mut cli);

    let settings = GeneratorSettings {
        openapi_path: cli.openapi_path.clone(),
        authorization_header: cli.authorization_header.clone(),
        authorization_header_from_environment_variable: cli
            .load_authorization_header_from_environment,
        authorization_header_variable_name: cli.authorization_header_variable_name.clone(),
        content_type: cli.content_type.clone(),
        base_url: cli.base_url.clone(),
        output_type: cli.output_type,
        timeout: cli.timeout,
        generate_intellij_tests: cli.generate_intellij_tests,
        custom_headers: cli.custom_header.clone(),
        skip_headers: cli.skip_headers,
    };

    let files = generate(&settings, &document)?;
    write_files(&cli.output, &files)?;

    println!("🎉 Generation completed successfully!");
    println!("⏱️  Duration: {:.3}s", started.elapsed().as_secs_f64());
    Ok(())
}

fn display_header(no_logging: bool) {
    println!("🚀 HTTP File Generator v{}", env!("CARGO_PKG_VERSION"));
    if no_logging {
        println!("Support key: ⚠️  Unavailable when logging is disabled");
    } else {
        println!("Support key: 🔑 {}", support_key());
    }
    println!();
}

fn support_key() -> String {
    use sha2::{Digest, Sha256};

    let user = std::env::var("USERNAME")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "unknown".to_string());
    let host = std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| "unknown".to_string());
    let hash = Sha256::digest(format!("{user}@{host}"));
    format!("{hash:x}").chars().take(7).collect()
}

fn acquire_azure_token_if_requested(cli: &mut Cli) {
    if cli.authorization_header.is_some()
        || (cli.azure_scope.is_none() && cli.azure_tenant_id.is_none())
    {
        return;
    }

    println!("🔐 Acquiring authorization header from Azure Entra ID...");
    let Some(scope) = cli.azure_scope.as_deref() else {
        eprintln!("Error:\n--azure-scope is required when acquiring an Azure Entra ID token");
        return;
    };

    let mut command = std::process::Command::new("az");
    command.args([
        "account",
        "get-access-token",
        "--scope",
        scope,
        "--query",
        "accessToken",
        "-o",
        "tsv",
    ]);

    if let Some(tenant) = cli.azure_tenant_id.as_deref() {
        command.args(["--tenant", tenant]);
    }

    match command.output() {
        Ok(output) if output.status.success() => {
            let token = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !token.is_empty() {
                cli.authorization_header = Some(format!("Bearer {token}"));
                println!("✅ Successfully acquired access token\n");
            }
        }
        Ok(output) => {
            let message = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error:\n{}", message.trim());
        }
        Err(error) => eprintln!("Error:\n{error}"),
    }
}

fn write_files(output: &str, files: &[httpgenerator::generator::HttpFile]) -> anyhow::Result<()> {
    println!("📁 Writing {} file(s)", files.len());
    fs::create_dir_all(output)
        .with_context(|| format!("Failed to create output directory {output}"))?;

    for file in files {
        let path = Path::new(output).join(&file.filename);
        fs::write(&path, &file.content)
            .with_context(|| format!("Failed to write {}", path.display()))?;
        println!("   📄 {}", path.display());
    }
    println!("✅ Files written successfully:\n");
    Ok(())
}
