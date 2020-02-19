# BtAppViewParams

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_id** | Option<**String**> |  | [optional]
**parent_change_id** | Option<**String**> |  | [optional]
**display_state_id** | Option<**String**> |  | [optional]
**model_reference_id** | Option<**String**> |  | [optional]
**view_matrix** | Option<**Vec<f64>**> |  | [optional]
**view_direction** | Option<**Vec<f64>**> |  | [optional]
**cut_point** | Option<**Vec<f64>**> |  | [optional]
**offset_section_points** | Option<**Vec<f64>**> |  | [optional]
**broken_out_section** | Option<**bool**> |  | [optional]
**crop_view** | Option<**bool**> |  | [optional]
**bom_reference_id** | Option<**String**> |  | [optional]
**include_hidden_instances** | Option<**bool**> |  | [optional]
**view_scale** | Option<**f64**> |  | [optional]
**show_tangent_lines** | Option<**bool**> |  | [optional]
**compute_intersection** | Option<**bool**> |  | [optional]
**quality_option** | Option<**i32**> |  | [optional]
**simplification_option** | Option<**i32**> |  | [optional]
**simplification_threshold** | Option<**f64**> |  | [optional]
**is_broken_out_section** | Option<**bool**> |  | [optional]
**is_crop_view** | Option<**bool**> |  | [optional]
**show_cut_geom_only** | Option<**bool**> |  | [optional]
**hidden_lines** | Option<**String**> |  | [optional]
**modification_id** | Option<**String**> |  | [optional]
**perspective** | Option<**bool**> |  | [optional]
**projection_angle** | Option<**String**> |  | [optional]
**show_threads** | Option<**bool**> |  | [optional]
**quality_option_type** | Option<**String**> |  | [optional]
**simplification_option_type** | Option<**String**> |  | [optional]
**is_partial_section** | Option<**bool**> |  | [optional]
**broken_out_point_numbers** | Option<**Vec<i32>**> |  | [optional]
**broken_out_end_conditions** | Option<[**::std::collections::HashMap<String, crate::models::BtBrokenOutEndCondition>**](BTBrokenOutEndCondition.md)> |  | [optional]
**broken_out_b_boxes** | Option<**Vec<f64>**> |  | [optional]
**broken_out_b_boxes_map** | Option<[**::std::collections::HashMap<String, crate::models::BtBoundingBox>**](BTBoundingBox.md)> |  | [optional]
**include_surfaces** | Option<**bool**> |  | [optional]
**is_surface** | Option<**bool**> |  | [optional]
**depth_section_end_condition** | Option<[**crate::models::BtBrokenOutEndCondition**](BTBrokenOutEndCondition.md)> |  | [optional]
**exploded_view_id** | Option<**String**> |  | [optional]
**occurrence_or_part_id_to_geometry_properties** | Option<[**::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>**](map.md)> |  | [optional]
**parameters** | Option<**Vec<f64>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


