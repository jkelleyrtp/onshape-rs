/*
 * Onshape REST API
 *
 * The Onshape REST API consumed by all clients.
 *
 * The version of the OpenAPI document: 1.104
 * Contact: api-support@onshape.zendesk.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BtDrawingParams {
    #[serde(rename = "workspaceId", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(rename = "elementId", skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    #[serde(rename = "documentId", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(rename = "partNumber", skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(rename = "partQuery", skip_serializing_if = "Option::is_none")]
    pub part_query: Option<String>,
    #[serde(rename = "partId", skip_serializing_if = "Option::is_none")]
    pub part_id: Option<String>,
    #[serde(rename = "elementMicroversionId", skip_serializing_if = "Option::is_none")]
    pub element_microversion_id: Option<String>,
    #[serde(rename = "sketchIds", skip_serializing_if = "Option::is_none")]
    pub sketch_ids: Option<Vec<String>>,
    #[serde(rename = "qualityOption", skip_serializing_if = "Option::is_none")]
    pub quality_option: Option<String>,
    #[serde(rename = "views", skip_serializing_if = "Option::is_none")]
    pub views: Option<String>,
    #[serde(rename = "computeIntersection", skip_serializing_if = "Option::is_none")]
    pub compute_intersection: Option<bool>,
    #[serde(rename = "simplificationOption", skip_serializing_if = "Option::is_none")]
    pub simplification_option: Option<String>,
    #[serde(rename = "simplificationThreshold", skip_serializing_if = "Option::is_none")]
    pub simplification_threshold: Option<f64>,
    #[serde(rename = "showCutGeomOnly", skip_serializing_if = "Option::is_none")]
    pub show_cut_geom_only: Option<bool>,
    #[serde(rename = "documentMicroversionId", skip_serializing_if = "Option::is_none")]
    pub document_microversion_id: Option<String>,
    #[serde(rename = "hiddenLines", skip_serializing_if = "Option::is_none")]
    pub hidden_lines: Option<HiddenLines>,
    #[serde(rename = "includeSurfaces", skip_serializing_if = "Option::is_none")]
    pub include_surfaces: Option<bool>,
    #[serde(rename = "isSurface", skip_serializing_if = "Option::is_none")]
    pub is_surface: Option<bool>,
    #[serde(rename = "pureSketch", skip_serializing_if = "Option::is_none")]
    pub pure_sketch: Option<bool>,
    #[serde(rename = "isSketchOnly", skip_serializing_if = "Option::is_none")]
    pub is_sketch_only: Option<bool>,
    #[serde(rename = "templateVersionId", skip_serializing_if = "Option::is_none")]
    pub template_version_id: Option<String>,
    #[serde(rename = "modelType", skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    #[serde(rename = "border", skip_serializing_if = "Option::is_none")]
    pub border: Option<bool>,
    #[serde(rename = "externalDocumentId", skip_serializing_if = "Option::is_none")]
    pub external_document_id: Option<String>,
    #[serde(rename = "externalDocumentVersionId", skip_serializing_if = "Option::is_none")]
    pub external_document_version_id: Option<String>,
    #[serde(rename = "elementConfiguration", skip_serializing_if = "Option::is_none")]
    pub element_configuration: Option<String>,
    #[serde(rename = "templateDocumentId", skip_serializing_if = "Option::is_none")]
    pub template_document_id: Option<String>,
    #[serde(rename = "templateWorkspaceId", skip_serializing_if = "Option::is_none")]
    pub template_workspace_id: Option<String>,
    #[serde(rename = "templateName", skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "templateArgs", skip_serializing_if = "Option::is_none")]
    pub template_args: Option<Vec<String>>,
    #[serde(rename = "templateElementId", skip_serializing_if = "Option::is_none")]
    pub template_element_id: Option<String>,
    #[serde(rename = "drawingName", skip_serializing_if = "Option::is_none")]
    pub drawing_name: Option<String>,
    #[serde(rename = "projection", skip_serializing_if = "Option::is_none")]
    pub projection: Option<String>,
    #[serde(rename = "standard", skip_serializing_if = "Option::is_none")]
    pub standard: Option<String>,
    #[serde(rename = "titleblock", skip_serializing_if = "Option::is_none")]
    pub titleblock: Option<bool>,
    #[serde(rename = "numberHorizontalZones", skip_serializing_if = "Option::is_none")]
    pub number_horizontal_zones: Option<i32>,
    #[serde(rename = "numberVerticalZones", skip_serializing_if = "Option::is_none")]
    pub number_vertical_zones: Option<i32>,
    #[serde(rename = "startZones", skip_serializing_if = "Option::is_none")]
    pub start_zones: Option<String>,
    #[serde(rename = "isFlattenedPart", skip_serializing_if = "Option::is_none")]
    pub is_flattened_part: Option<bool>,
    #[serde(rename = "referenceTypeEnum", skip_serializing_if = "Option::is_none")]
    pub reference_type_enum: Option<ReferenceTypeEnum>,
    #[serde(rename = "referenceType", skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<i32>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<crate::models::BtElementLocationParams>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
    #[serde(rename = "decimalSeparator", skip_serializing_if = "Option::is_none")]
    pub decimal_separator: Option<String>,
}

impl BtDrawingParams {
    pub fn new() -> BtDrawingParams {
        BtDrawingParams {
            workspace_id: None,
            element_id: None,
            document_id: None,
            part_number: None,
            revision: None,
            part_query: None,
            part_id: None,
            element_microversion_id: None,
            sketch_ids: None,
            quality_option: None,
            views: None,
            compute_intersection: None,
            simplification_option: None,
            simplification_threshold: None,
            show_cut_geom_only: None,
            document_microversion_id: None,
            hidden_lines: None,
            include_surfaces: None,
            is_surface: None,
            pure_sketch: None,
            is_sketch_only: None,
            template_version_id: None,
            model_type: None,
            border: None,
            external_document_id: None,
            external_document_version_id: None,
            element_configuration: None,
            template_document_id: None,
            template_workspace_id: None,
            template_name: None,
            template_args: None,
            template_element_id: None,
            drawing_name: None,
            projection: None,
            standard: None,
            titleblock: None,
            number_horizontal_zones: None,
            number_vertical_zones: None,
            start_zones: None,
            is_flattened_part: None,
            reference_type_enum: None,
            reference_type: None,
            location: None,
            size: None,
            language: None,
            units: None,
            decimal_separator: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HiddenLines {
    #[serde(rename = "DRAFTING")]
    DRAFTING,
    #[serde(rename = "EXCLUDED")]
    EXCLUDED,
    #[serde(rename = "MARKED")]
    MARKED,
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReferenceTypeEnum {
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
    #[serde(rename = "PARTSTUDIO")]
    PARTSTUDIO,
    #[serde(rename = "ASSEMBLY")]
    ASSEMBLY,
    #[serde(rename = "PART")]
    PART,
    #[serde(rename = "FLATTENED_PART")]
    FLATTENEDPART,
    #[serde(rename = "COMPOSITE_PART")]
    COMPOSITEPART,
    #[serde(rename = "MESH_PART")]
    MESHPART,
    #[serde(rename = "SURFACE")]
    SURFACE,
    #[serde(rename = "SKETCH")]
    SKETCH,
    #[serde(rename = "CURVE")]
    CURVE,
}

