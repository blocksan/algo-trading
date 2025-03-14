use std::fs;
use std::path::{Path, PathBuf};
use syn::{parse_file, Item, ItemFn, Attribute};
use quote::ToTokens;

fn extract_api_docs_from_file(file_path: &Path) -> Vec<String> {
    let content = fs::read_to_string(file_path).expect("Failed to read file");
    let file = parse_file(&content).expect("Failed to parse file");
    
    let mut docs = Vec::new();
    
    for item in file.items {
        if let Item::Fn(func) = item {
            // Extract from any function with doc comments containing @path
            let doc = extract_doc_from_function(&func);
            if doc.contains("@path") {
                docs.push(extract_doc_from_function(&func));
            }
        }
    }
    
    docs
}

fn extract_doc_from_function(func: &ItemFn) -> String {
    // Extract documentation from function attributes
    let mut doc_string = String::new();
    
    for attr in &func.attrs {
        if attr.path.segments.iter().any(|segment| segment.ident == "doc") {
            if let Ok(meta) = attr.parse_meta() {
                if let syn::Meta::NameValue(name_value) = meta {
                    if let syn::Lit::Str(lit_str) = name_value.lit {
                        doc_string.push_str(&lit_str.value());
                        doc_string.push('\n');
                    }
                }
            }
        }
    }
    
    doc_string
}

fn main() {
    let src_dir = PathBuf::from("src");
    let mut all_docs = Vec::new();
    
    // Recursively scan all Rust files
    visit_dirs(&src_dir, &mut all_docs).expect("Failed to scan directories");
    
    // Generate OpenAPI spec from collected docs
    generate_openapi_spec(&all_docs);
}

fn visit_dirs(dir: &Path, docs: &mut Vec<String>) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                visit_dirs(&path, docs)?;
            } else if let Some(extension) = path.extension() {
                if extension == "rs" {
                    docs.extend(extract_api_docs_from_file(&path));
                }
            }
        }
    }
    
    Ok(())
}

fn generate_openapi_spec(docs: &[String]) {
    // Convert collected docs to OpenAPI spec
    // This would involve parsing the documentation strings and
    // constructing the OpenAPI structure
    
    // Write the OpenAPI spec to a file
    fs::write("algo-trading-api.yaml", "# Generated OpenAPI spec").expect("Failed to write spec");
} 