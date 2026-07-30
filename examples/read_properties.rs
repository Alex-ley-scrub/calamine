// SPDX-License-Identifier: MIT
//
// Copyright 2016-2026, Johann Tuffe.

//! Demonstrates reading workbook properties from an XLSX or XLSB file.
//!
//! This example reads the core and extended properties (such as creator,
//! application, and company) as well as any custom document properties stored
//! in `docProps/custom.xml`.
//!
//! Run the example like this:
//!
//! ```text
//! $ cargo run -q --example read_properties -- tests/workbook_custom_properties.xlsx
//!
//! Core / Extended properties:
//!   creator: Some("Test Creator")
//!   last_modified_by: Some("Last Modifier")
//!   created: Some("2024-01-15T08:30:00Z")
//!   modified: Some("2024-06-20T14:22:00Z")
//!   title: Some("Workbook Title")
//!   application: Some("Microsoft Excel")
//!   app_version: Some("16.0300")
//!   company: Some("Contoso")
//!   template: None
//!   manager: None
//!
//! Custom properties:
//!   MyInt: 4 (vt:i4)
//!   MyFloat: 2.5 (vt:r8)
//!   MyBool: true (vt:bool)
//!   MyDateTime: 2020-08-24T20:19:22Z (vt:filetime)
//!   MyString: hello (vt:lpwstr)
//!   MyLink: SomeName (vt:lpwstr)
//! ```

use calamine::{open_workbook_auto, Reader};
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <xlsx/xlsb path>", args[0]);
        exit(1);
    }

    let path = &args[1];
    let excel = match open_workbook_auto(path) {
        Ok(excel) => excel,
        Err(e) => {
            eprintln!("Cannot open {path}: {e}");
            exit(1);
        }
    };

    let props = excel.metadata().workbook_properties();

    println!("Core / Extended properties:");
    println!("  creator: {:?}", props.creator);
    println!("  last_modified_by: {:?}", props.last_modified_by);
    println!("  created: {:?}", props.created);
    println!("  modified: {:?}", props.modified);
    println!("  title: {:?}", props.title);
    println!("  application: {:?}", props.application);
    println!("  app_version: {:?}", props.app_version);
    println!("  company: {:?}", props.company);
    println!("  template: {:?}", props.template);
    println!("  manager: {:?}", props.manager);

    if props.custom_properties.is_empty() {
        println!("\nNo custom properties.");
    } else {
        println!("\nCustom properties:");
        for (name, value) in &props.custom_properties {
            println!("  {name}: {value} ({})", value.vt_type());
        }
    }
}
