//! Variable font support for font variations
//!
//! Handles OpenType variable fonts with design space axes.

use super::Font;

/// A variable font axis
#[derive(Debug, Clone)]
pub struct VariationAxis {
    /// Axis tag (e.g., "wght" for weight)
    pub tag: String,
    
    /// Axis name
    pub name: String,
    
    /// Minimum value
    pub min_value: f32,
    
    /// Default value
    pub default_value: f32,
    
    /// Maximum value
    pub max_value: f32,
}

/// Variable font variation settings
#[derive(Debug, Clone)]
pub struct Variation {
    /// Axis tag
    pub tag: String,
    
    /// Value for this axis
    pub value: f32,
}

/// Variable font manager
pub struct VariableFontManager;

impl VariableFontManager {
    /// Check if font is a variable font
    pub fn is_variable(font: &Font) -> bool {
        let face = font.face();
        face.tables().fvar.is_some()
    }
    
    /// Get available variation axes
    pub fn axes(font: &Font) -> Vec<VariationAxis> {
        let face = font.face();
        
        let fvar = match face.tables().fvar {
            Some(fvar) => fvar,
            None => return Vec::new(),
        };
        
        let mut axes = Vec::new();
        
        for axis in fvar.axes {
            // Get axis name from name table
            let name = face.names()
                .into_iter()
                .find(|n| n.name_id == axis.name_id)
                .and_then(|n| n.to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            
            axes.push(VariationAxis {
                tag: String::from_utf8_lossy(&axis.tag.to_bytes()).to_string(),
                name,
                min_value: axis.min_value,
                default_value: axis.def_value,
                max_value: axis.max_value,
            });
        }
        
        axes
    }
    
    /// Get named instances (predefined variations)
    /// Note: Requires instance iteration support in ttf-parser
    pub fn instances(_font: &Font) -> Vec<String> {
        // Stub - ttf-parser's fvar doesn't expose instances iterator
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_variation_axis() {
        let axis = VariationAxis {
            tag: "wght".to_string(),
            name: "Weight".to_string(),
            min_value: 100.0,
            default_value: 400.0,
            max_value: 900.0,
        };
        
        assert_eq!(axis.tag, "wght");
        assert_eq!(axis.default_value, 400.0);
    }
}
