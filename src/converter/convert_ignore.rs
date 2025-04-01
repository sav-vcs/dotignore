// Conversor de formatos de archivos ignore
//
// Este módulo implementa la funcionalidad para convertir archivos .gitignore y .svnignore
// al formato .DotIgnore

use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use crate::ignore::{DotIgnore, ConversionResult};
use std::fs;

/// Convierte un archivo .gitignore o .svnignore al formato .DotIgnore
///
/// # Argumentos
///
/// * `source_path` - Ruta al archivo de origen (.gitignore o .svnignore)
/// * `dest_path` - Ruta opcional donde guardar el archivo .DotIgnore generado
///
/// # Retorna
///
/// Un `Result` que contiene el resultado de la conversión o un error si falla
pub fn convert_file<P: AsRef<Path>>(source_path: P, dest_path: Option<P>) -> Result<ConversionResult> {
    let source = source_path.as_ref();
    let dest = dest_path.map(|p| p.as_ref().to_path_buf())
        .unwrap_or_else(|| {
            let mut dest = source.to_path_buf();
            dest.set_file_name(".DotIgnore");
            dest
        });
    
    // Verificar que el archivo de origen existe
    if !source.exists() {
        return Err(anyhow::anyhow!("El archivo de origen no existe: {}", source.display()));
    }
    
    // Leer el archivo de origen
    let content = fs::read_to_string(source)
        .context(format!("No se pudo leer el archivo: {}", source.display()))?;
    
    // Determinar el formato de origen basado en la extensión
    let format = determine_format(source)?;
    
    // Convertir el contenido
    let result = convert_content(&content, format, source, &dest)?;
    
    // Ya no necesitamos guardar el resultado aquí ya que lo hacemos en convert_content
    
    Ok(result)
}

/// Convierte un archivo .gitignore o .svnignore al formato .DotIgnore sin guardar el resultado
///
/// # Argumentos
///
/// * `content` - Contenido del archivo a convertir
/// * `format` - Formato del archivo de origen ("git" o "svn")
/// * `source_path` - Ruta al archivo de origen
/// * `dest_path` - Ruta donde se guardaría el archivo de destino
///
/// # Retorna
///
/// Un `Result` que contiene el resultado de la conversión o un error si falla
pub fn convert_content(content: &str, format: &str, source_path: &Path, dest_path: &Path) -> Result<ConversionResult> {
    let mut result = ConversionResult {
        source_file: source_path.to_path_buf(),
        destination_file: dest_path.to_path_buf(),
        pattern_count: 0,
        standard_patterns: 0,
        platform_patterns: std::collections::HashMap::new(),
    };
    
    // String para el contenido de salida
    let mut output_content = String::new();
    
    // Añadir cabecera
    output_content.push_str(&format!("# .DotIgnore file converted from .{}\n", format));
    output_content.push_str(&format!("# Conversion date: {}\n\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
    
    // Vector para almacenar patrones por grupo
    let mut current_group_name = String::from("default");
    let mut current_group_patterns = Vec::new();
    let mut last_line_was_comment = false;
    let mut last_comment = String::new();
    
    // Procesar cada línea según el formato
    for line in content.lines() {
        let trimmed = line.trim();
        
        // Saltar líneas vacías pero finalizar grupo actual si es necesario
        if trimmed.is_empty() {
            last_line_was_comment = false;
            continue;
        }
        
        // Procesar comentarios
        if (format == "git" && trimmed.starts_with('#')) || 
           (format == "svn" && trimmed.starts_with('#')) {
            // Si hay un comentario, considerar crear un nuevo grupo
            let comment_text = trimmed.trim_start_matches('#').trim();
            last_comment = comment_text.to_string();
            last_line_was_comment = true;
            continue;
        }
        
        // Si hay un comentario previo y esta línea no es un comentario, crear un nuevo grupo
        if last_line_was_comment && !trimmed.starts_with('#') {
            // Finalizar grupo actual si tiene patrones
            if !current_group_patterns.is_empty() {
                output_content.push_str(&format!("[{}] {{\n", current_group_name));
                for pattern in &current_group_patterns {
                    output_content.push_str(&format!("    {}\n", pattern));
                    result.pattern_count += 1;
                    if current_group_name == "default" {
                        result.standard_patterns += 1;
                    } else {
                        *result.platform_patterns.entry(current_group_name.clone()).or_insert(0) += 1;
                    }
                }
                output_content.push_str("}\n\n");
                current_group_patterns.clear();
            }
            
            // Crear nuevo grupo basado en el comentario
            if !last_comment.is_empty() {
                current_group_name = slugify(&last_comment);
            } else {
                current_group_name = String::from("default");
            }
        }
        
        // Añadir patrón al grupo actual
        current_group_patterns.push(line.to_string());
        last_line_was_comment = false;
    }
    
    // Finalizar último grupo si tiene patrones
    if !current_group_patterns.is_empty() {
        output_content.push_str(&format!("[{}] {{\n", current_group_name));
        for pattern in &current_group_patterns {
            output_content.push_str(&format!("    {}\n", pattern));
            result.pattern_count += 1;
            if current_group_name == "default" {
                result.standard_patterns += 1;
            } else {
                *result.platform_patterns.entry(current_group_name.clone()).or_insert(0) += 1;
            }
        }
        output_content.push_str("}\n");
    }
    
    // Guardar el contenido en el archivo directamente
    fs::write(dest_path, &output_content)
        .context(format!("No se pudo escribir el archivo: {}", dest_path.display()))?;
    
    Ok(result)
}

/// Convierte un texto a formato slug (para nombres de grupo)
fn slugify(text: &str) -> String {
    let mut slug = String::new();
    
    // Eliminar caracteres no alfanuméricos y reemplazar espacios con guiones bajos
    for c in text.chars() {
        if c.is_alphanumeric() {
            slug.push(c.to_ascii_lowercase());
        } else if c.is_whitespace() {
            slug.push('_');
        }
    }
    
    // Si está vacío, usar "default"
    if slug.is_empty() {
        return String::from("default");
    }
    
    slug
}

/// Determina el formato del archivo de origen basado en su nombre o extensión
///
/// # Argumentos
///
/// * `path` - Ruta al archivo
///
/// # Retorna
///
/// Un `Result` que contiene el formato ("git" o "svn") o un error si no se puede determinar
fn determine_format(path: &Path) -> Result<&'static str> {
    let filename = path.file_name()
        .and_then(|f| f.to_str())
        .ok_or_else(|| anyhow::anyhow!("No se pudo determinar el nombre del archivo"))?;
    
    if filename.contains("gitignore") {
        Ok("git")
    } else if filename.contains("svnignore") {
        Ok("svn")
    } else {
        Err(anyhow::anyhow!("No se pudo determinar el formato del archivo. El nombre debe contener 'gitignore' o 'svnignore'"))
    }
} 
