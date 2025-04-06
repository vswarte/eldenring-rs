use clap::Parser;
use quick_xml::de::from_str;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

/// Command line arguments.
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Folder containing XML files.
    #[arg(short, long)]
    input: String,
    /// Output Rust file.
    #[arg(short, long)]
    output: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let input_path = Path::new(&args.input);
    let mut output = String::new();

    output.push_str("//! THIS FILE IS GENERATED FROM THE PARAM DEFS, DO NOT EDIT IT DIRECTLY\n\n");

    output.push_str("/// Trait to perform safe param lookups.\n");
    output.push_str("pub trait ParamDef {\n");
    output.push_str("    const NAME: &str;\n");
    output.push_str("}\n\n");

    for entry in fs::read_dir(input_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("xml") {
            let content = fs::read_to_string(&path)?;
            match from_str::<ParamDef>(&content) {
                Ok(param) => {
                    let structure = build_definition(&param);
                    output.push_str(&generate_code(&structure));
                }
                Err(e) => eprintln!("Failed to parse {}: {}", path.display(), e),
            }
        }
    }
    File::create(&args.output)?.write_all(output.as_bytes())?;
    Ok(())
}

fn generate_code(def: &StructDef) -> String {
    let mut code = String::new();

    // Group the bitfields
    let mut bitfield_groups: HashMap<usize, Vec<(&LayoutUnit, u8)>> = HashMap::new();
    for unit in def.layout.iter() {
        if let FieldType::Bitfield(_) = unit.field_type {
            bitfield_groups
                .entry(unit.offset)
                .or_default()
                .push((unit, unit.size as u8));
        }
    }
    let mut grouped_names = HashMap::new();

    code.push_str("#[derive(Debug, Clone)]\n");
    code.push_str("#[allow(non_camel_case_types)]\n");
    code.push_str("#[repr(C)]\n");

    code.push_str(&format!("pub struct {} {{\n", def.name));
    for unit in def.layout.iter() {
        match &unit.field_type {
            FieldType::Bitfield(_) => {
                if grouped_names.contains_key(&unit.offset) {
                    continue;
                }

                let group_name = format!("bits_{:x}", &unit.offset);

                code.push_str(&format!("    {}: u8,\n", &group_name));

                grouped_names.insert(unit.offset, group_name);
            }
            FieldType::Standard(_) => {
                code.push_str(&format!(
                    "    {}: {},\n",
                    normalize_name(&unit.name),
                    unit.field_type.native_type()
                ));
            }
            FieldType::Array(inner_type, count) => {
                code.push_str(&format!(
                    "    {}: [{}; {}],\n",
                    normalize_name(&unit.name),
                    inner_type.native_type(),
                    count,
                ));
            }
        }
    }
    code.push_str("}\n\n");

    code.push_str(&format!("impl ParamDef for {} {{\n", def.name));
    code.push_str(&format!("    const NAME: &str = \"{}\";\n", def.name));
    code.push_str("}\n\n");

    code.push_str(&format!("impl {} {{\n", def.name));
    for unit in def.layout.iter() {
        if unit.hidden() {
            continue;
        }

        let normalized_name = normalize_name(&unit.name);
        match &unit.field_type {
            FieldType::Bitfield(bits) => {
                let group_name = &grouped_names[&unit.offset];
                let used_bits: u8 = bitfield_groups[&unit.offset]
                    .iter()
                    .take_while(|(u, _)| u.name != unit.name)
                    .map(|(_, b)| *b)
                    .sum();

                let mask = (1 << bits) - 1;

                code.push_str("    #[allow(clippy::identity_op)]\n");
                code.push_str(&format!(
                    "    pub fn {}(&self) -> u8 {{\n        (self.{} >> {}) & 0b{:08b}\n    }}\n\n",
                    normalized_name, group_name, used_bits, mask
                ));

                code.push_str("    #[allow(clippy::identity_op)]\n");
                code.push_str(&format!(
                    "    pub fn set_{}(&mut self, value: u8) {{\n        self.{} = (self.{} & !(0b{:08b} << {})) | ((value & 0b{:08b}) << {});\n    }}\n\n",
                    normalized_name, group_name, group_name, mask, used_bits, mask, used_bits
                ));
            }
            FieldType::Standard(_) => {
                code.push_str(&format!(
                    "    pub fn {}(&self) -> {} {{\n        self.{}\n    }}\n\n",
                    normalized_name,
                    unit.field_type.native_type(),
                    normalized_name
                ));
                code.push_str(&format!(
                    "    pub fn set_{}(&mut self, value: {}) {{\n        self.{} = value;\n    }}\n\n",
                    normalized_name, unit.field_type.native_type(), normalized_name
                ));
            }
            FieldType::Array(inner, count) => {
                code.push_str(&format!(
                    "    pub fn {}(&self) -> &[{}; {}] {{\n        &self.{}\n    }}\n\n",
                    normalized_name,
                    inner.native_type(),
                    count,
                    normalized_name
                ));
                code.push_str(&format!(
                    "    pub fn set_{}(&mut self, value: [{}; {}]) {{\n        self.{} = value;\n    }}\n\n",
                    normalized_name, inner.native_type(), count, normalized_name
                ));
            }
        }
    }
    code.push_str("}\n\n");

    code
}

fn build_definition(parsed: &ParamDef) -> StructDef {
    let fields = parsed
        .fields
        .fields
        .iter()
        .filter(|e| e.removed_version.is_none())
        .map(|f| parse_field(f).unwrap())
        .collect::<Vec<_>>();

    let layout = layout_struct(&fields);

    StructDef {
        name: parsed.param_type.clone(),
        layout,
    }
}

fn layout_struct(fields: &[LayoutField]) -> Vec<LayoutUnit> {
    let mut offset = 0;
    let mut layout = Vec::new();
    let mut bit_cursor: Option<(usize, u8)> = None;

    for field in fields {
        let (alignment, size) = field.field_type.alignment_and_size();

        match &field.field_type {
            FieldType::Bitfield(bits) => {
                // Either get the current bit cursor or start a new one
                let (byte_offset, used_bits) = bit_cursor.unwrap_or((offset, 0));

                // Wrap around the 8 bit boundaries.
                if used_bits + bits > 8 {
                    offset += 1;
                    bit_cursor = Some((offset, *bits));
                } else {
                    bit_cursor = Some((byte_offset, used_bits + bits));
                }

                layout.push(LayoutUnit {
                    name: field.name.clone(),
                    offset,
                    size,
                    field_type: field.field_type.clone(),
                });

                // Yeet bit cursor and advance offset if we get to the end of a byte.
                if let Some((_, used)) = bit_cursor {
                    if used == 8 {
                        offset += 1;
                        bit_cursor = None;
                    }
                }
            }
            FieldType::Standard(_) | FieldType::Array(_, _) => {
                // Clean bit cursor if we're in the middle of a byte and need to enforce alignment.
                if bit_cursor.is_some() {
                    offset += 1;
                    bit_cursor = None;
                }

                // Align to current types alignment.
                offset = align_offset(offset, alignment);
                layout.push(LayoutUnit {
                    name: field.name.clone(),
                    offset,
                    size,
                    field_type: field.field_type.clone(),
                });
                offset += size;
            }
        }
    }

    layout
}

fn parse_field(field: &FieldDef) -> Option<LayoutField> {
    let (main, _) = if let Some(pos) = field.def.find('=') {
        (
            field.def[..pos].trim(),
            Some(field.def[pos + 1..].trim().to_string()),
        )
    } else {
        (field.def.as_str(), None)
    };

    let mut parts = main.split_whitespace();
    let orig_type = parts.next()?.to_string();
    let remainder = parts.next()?.to_string();
    let mut name = remainder.clone();
    let mut bit_width = None;
    let mut array_size = None;

    if let Some(colon) = remainder.find(':') {
        name = remainder[..colon].to_string();
        let after = &remainder[colon + 1..];
        let (bw_str, rest) = if let Some(bracket) = after.find('[') {
            (&after[..bracket], &after[bracket..])
        } else {
            (after, "")
        };
        bit_width = bw_str.parse::<u32>().ok();
        if rest.starts_with('[') && rest.ends_with(']') {
            array_size = rest[1..rest.len() - 1].parse().ok();
        }
    } else if let Some(bracket) = remainder.find('[') {
        name = remainder[..bracket].to_string();
        let closing = remainder.find(']').unwrap_or(remainder.len());
        array_size = remainder[bracket + 1..closing].parse().ok();
    }

    let field_type = match (bit_width, array_size) {
        (None, None) => FieldType::Standard(orig_type),
        (None, Some(array_size)) => {
            FieldType::Array(Box::new(FieldType::Standard(orig_type)), array_size)
        }
        (Some(bit_width), None) => FieldType::Bitfield(bit_width as u8),
        (Some(_), Some(_)) => unimplemented!(),
    };

    Some(LayoutField { name, field_type })
}

fn normalize_name(mut name: &str) -> String {
    // type is a reserved keyword, so we cannot name properties that.
    if name == "type" {
        name = "typ";
    }

    let mut result = String::with_capacity(name.len());
    let mut prev_char: Option<char> = None;

    for (i, ch) in name.chars().enumerate() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_uppercase() {
                if let Some(prev) = prev_char {
                    if prev.is_lowercase() || (prev.is_numeric() && i > 0) {
                        result.push('_');
                    } else if prev.is_uppercase() {
                        // Look ahead to avoid splitting acronyms prematurely
                        if let Some(next) = name.chars().nth(i + 1) {
                            if next.is_lowercase() {
                                result.push('_');
                            }
                        }
                    }
                }
                result.push(ch.to_ascii_lowercase());
            } else {
                result.push(ch);
            }
        } else {
            // Replace symbols (non-alphanumerics) with a single underscore
            if !result.ends_with('_') {
                result.push('_');
            }
        }
        prev_char = Some(ch);
    }

    // Remove leading/trailing/multiple underscores
    let cleaned = result
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_");

    cleaned
}

fn align_offset(offset: usize, align: usize) -> usize {
    (offset + align - 1) & !(align - 1)
}

#[derive(Debug, Deserialize)]
#[serde(rename = "PARAMDEF")]
struct ParamDef {
    #[serde(rename = "ParamType")]
    param_type: String,
    #[serde(rename = "Fields")]
    fields: Fields,
}

#[derive(Debug, Deserialize)]
struct Fields {
    #[serde(rename = "Field")]
    fields: Vec<FieldDef>,
}

#[derive(Debug, Deserialize)]
struct FieldDef {
    #[serde(rename = "@Def")]
    def: String,
    #[serde(rename = "@RemovedVersion")]
    removed_version: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
enum FieldType {
    Bitfield(u8),
    Standard(String),
    Array(Box<FieldType>, usize),
}

impl FieldType {
    fn alignment_and_size(&self) -> (usize, usize) {
        match self {
            FieldType::Bitfield(_) => (1, 1),
            FieldType::Standard(ty) => match ty.as_str() {
                "u8" | "s8" | "dummy8" | "fixstr" => (1, 1),
                "u16" | "s16" | "fixstrW" => (2, 2),
                "u32" | "s32" | "f32" => (4, 4),
                _ => panic!("Unknown type: {ty}"),
            },
            FieldType::Array(inner_type, repetitions) => (
                inner_type.alignment_and_size().0,
                inner_type.alignment_and_size().1 * repetitions,
            ),
        }
    }

    fn native_type(&self) -> &str {
        match self {
            FieldType::Standard(ty) => match ty.as_str() {
                "u8" | "dummy8" | "fixstr" => "u8",
                "s8" => "i8",
                "u16" | "fixstrW" => "u16",
                "s16" => "i16",
                "u32" => "u32",
                "f32" => "f32",
                "s32" => "i32",
                _ => panic!("Unknown type: {ty}"),
            },
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
struct StructDef {
    name: String,
    layout: Vec<LayoutUnit>,
}

#[derive(Debug)]
struct LayoutField {
    name: String,
    field_type: FieldType,
}

#[derive(Debug)]
struct LayoutUnit {
    name: String,
    offset: usize,
    size: usize,
    field_type: FieldType,
}

impl LayoutUnit {
    pub fn hidden(&self) -> bool {
        if FieldType::Standard("dummy8".to_string()) == self.field_type {
            return true;
        }

        if let FieldType::Array(inner, _) = &self.field_type {
            if FieldType::Standard("dummy8".to_string()) == **inner {
                return true;
            }
        }

        let lower = self.name.to_lowercase();
        lower.contains("reserve") || lower.starts_with("pad") || lower.starts_with("unk")
    }
}
