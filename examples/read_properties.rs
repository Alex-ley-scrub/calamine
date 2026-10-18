// SPDX-License-Identifier: MIT
//
// Copyright 2016-2026, Johann Tuffe.

//! Demonstrates reading workbook properties from an XLSX or XLSB file.
//!
//! This example reads the document properties (such as creator, application,
//! and company) as well as any custom document properties.
//!
//! Run the example like this:
//!
//! ```text
//! $ cargo run -q --example read_properties -- tests/workbook_custom_properties.xlsx
//!
//! Workbook properties:
//!   creator: Some("Test Creator")
//!   last_modified_by: None
//!   created: None
//!   modified: None
//!   title: None
//!   application: Some("Microsoft Excel")
//!   app_version: None
//!   company: None
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

use calamine::open_workbook_auto;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <xlsx/xlsb path>", args[0]);
        exit(1);
    }

    let path = &args[1];
    let mut excel = match open_workbook_auto(path) {
        Ok(excel) => excel,
        Err(e) => {
            eprintln!("Cannot open {path}: {e}");
            exit(1);
        }
    };

    let props = match excel.workbook_properties() {
        Ok(props) => props,
        Err(e) => {
            eprintln!("Cannot read workbook properties from {path}: {e}");
            exit(1);
        }
    };

    println!("Workbook properties:");
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
