use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pyohwa", version, about = "Rust + Elm static site generator")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new Pyohwa project
    Init {
        /// Project name (directory will be created)
        name: String,
    },
    /// Build the static site
    Build {
        /// Project root directory (default: current directory)
        #[arg(short, long, default_value = ".")]
        root: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Init { name } => run_init(&name),
        Command::Build { root } => run_build(&root),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run_init(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let target = PathBuf::from(name);

    if target.exists() {
        return Err(format!("directory '{}' already exists", name).into());
    }

    // Create project directory structure
    std::fs::create_dir_all(target.join("content"))?;
    std::fs::create_dir_all(target.join("static"))?;

    // Write default index.md
    std::fs::write(
        target.join("content/index.md"),
        r#"---
title: "Welcome"
layout: home
---

# Welcome to Pyohwa

Your documentation starts here.
"#,
    )?;

    // Write default pyohwa.toml
    std::fs::write(
        target.join("pyohwa.toml"),
        r#"[site]
title = "My Documentation"
description = ""
"#,
    )?;

    // Write .gitignore
    std::fs::write(target.join(".gitignore"), "dist/\n")?;

    println!("Created new Pyohwa project in '{name}'");
    println!();
    println!("  cd {name}");
    println!("  pyohwa build");

    Ok(())
}

fn run_build(root: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let project_root = if root == &PathBuf::from(".") {
        std::env::current_dir()?
    } else {
        std::fs::canonicalize(root)?
    };

    pyohwa_core::build::pipeline::build(&project_root)?;

    println!("Build complete.");
    Ok(())
}
