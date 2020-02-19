# \PartStudiosApi

All URIs are relative to *https://cad.onshape.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_feature1**](PartStudiosApi.md#add_feature1) | **Post** /api/partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/features | Add Feature
[**compare_part_studios**](PartStudiosApi.md#compare_part_studios) | **Get** /api/partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/compare | Compare Part Studios
[**create_part_studio**](PartStudiosApi.md#create_part_studio) | **Post** /api/partstudios/d/{did}/w/{wid} | Create Part Studio
[**delete_feature1**](PartStudiosApi.md#delete_feature1) | **Delete** /api/partstudios/d/{did}/w/{wid}/e/{eid}/features/featureid/{fid} | Delete Feature
[**eval_feature_script**](PartStudiosApi.md#eval_feature_script) | **Post** /api/partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/featurescript | Evaluate FeatureScript
[**export_ps1**](PartStudiosApi.md#export_ps1) | **Get** /api/partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/parasolid | Export Part Studio to Parasolid
[**export_stl1**](PartStudiosApi.md#export_stl1) | **Get** /api/partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/stl | Export Part Studio to STL
[**get_body_details2**](PartStudiosApi.md#get_body_details2) | **Get** /api/partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/bodydetails | Array of body information
[**get_bounding_boxes2**](PartStudiosApi.md#get_bounding_boxes2) | **Get** /api/partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/boundingboxes | Mass properties of parts or a PartStudio.
[**get_configuration1**](PartStudiosApi.md#get_configuration1) | **Get** /api/partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/configuration | Get Configuration
[**get_edges2**](PartStudiosApi.md#get_edges2) | **Get** /api/partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/tessellatededges | Tesselated edges from a PartStudio.
[**get_faces2**](PartStudiosApi.md#get_faces2) | **Get** /api/partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/tessellatedfaces | Tesselated faces of the parts in the Part Studio.
[**get_feature_specs1**](PartStudiosApi.md#get_feature_specs1) | **Get** /api/partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/featurespecs | Get Feature Specs
[**get_features1**](PartStudiosApi.md#get_features1) | **Get** /api/partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/features | Get Feature List
[**get_mass_properties1**](PartStudiosApi.md#get_mass_properties1) | **Get** /api/partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/massproperties | Mass properties of parts or a PartStudio.
[**get_named_views1**](PartStudiosApi.md#get_named_views1) | **Get** /api/partstudios/d/{did}/e/{eid}/namedViews | Get Named Views
[**get_shaded_views2**](PartStudiosApi.md#get_shaded_views2) | **Get** /api/partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/shadedviews | Get Shaded Views
[**get_translator_formats4**](PartStudiosApi.md#get_translator_formats4) | **Get** /api/partstudios/d/{did}/w/{wid}/e/{eid}/translationformats | Get Translation Formats
[**translate_format5**](PartStudiosApi.md#translate_format5) | **Post** /api/partstudios/d/{did}/{wv}/{wvid}/e/{eid}/translations | Create Part Studio translation
[**translate_ids**](PartStudiosApi.md#translate_ids) | **Post** /api/partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/idtranslations | Id Translations
[**update_configuration1**](PartStudiosApi.md#update_configuration1) | **Post** /api/partstudios/d/{did}/{wvm}/{wvmid}/e/{eid}/configuration | Update Configuration
[**update_feature1**](PartStudiosApi.md#update_feature1) | **Post** /api/partstudios/d/{did}/w/{wid}/e/{eid}/features/featureid/{fid} | Update Feature
[**update_features**](PartStudiosApi.md#update_features) | **Post** /api/partstudios/d/{did}/w/{wid}/e/{eid}/features/updates | Update Features
[**update_rollback**](PartStudiosApi.md#update_rollback) | **Post** /api/partstudios/d/{did}/w/{wid}/e/{eid}/features/rollback | Update Feature Rollback



## add_feature1

> crate::models::BtFeatureDefinitionResponse add_feature1(did, wvm, wvmid, eid, body)
Add Feature

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**body** | Option<**String**> | feature The serialized feature definition |  |

### Return type

[**crate::models::BtFeatureDefinitionResponse**](BTFeatureDefinitionResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compare_part_studios

> crate::models::BtRootDiffInfo compare_part_studios(did, wvm, wvmid, eid, workspace_id, version_id, microversion_id, source_configuration, target_configuration, link_document_id)
Compare Part Studios

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**workspace_id** | Option<**String**> |  |  |
**version_id** | Option<**String**> |  |  |
**microversion_id** | Option<**String**> |  |  |
**source_configuration** | Option<**String**> |  |  |
**target_configuration** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> |  |  |

### Return type

[**crate::models::BtRootDiffInfo**](BTRootDiffInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_part_studio

> crate::models::BtDocumentElementInfo create_part_studio(did, wid, bt_model_element_params)
Create Part Studio

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wid** | **String** | Workspace ID. | [required] |
**bt_model_element_params** | [**BtModelElementParams**](BtModelElementParams.md) |  | [required] |

### Return type

[**crate::models::BtDocumentElementInfo**](BTDocumentElementInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_feature1

> delete_feature1(did, wid, eid, fid)
Delete Feature

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wid** | **String** | Workspace ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**fid** | **String** | The id of the feature being updated. This id should be URL encoded and must match the featureId found in the serialized structure | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## eval_feature_script

> crate::models::BtFeatureScriptEvalResponse eval_feature_script(did, wvm, wvmid, eid, configuration, bt_feature_script_eval_call)
Evaluate FeatureScript

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**configuration** | Option<**String**> | Configuration string. |  |
**bt_feature_script_eval_call** | Option<[**BtFeatureScriptEvalCall**](BtFeatureScriptEvalCall.md)> |  |  |

### Return type

[**crate::models::BtFeatureScriptEvalResponse**](BTFeatureScriptEvalResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_ps1

> export_ps1(did, wvm, wvmid, eid, part_ids, version, include_export_ids, configuration, link_document_id)
Export Part Studio to Parasolid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**part_ids** | Option<**String**> | IDs of the parts to retrieve. Repeat query param to add more than one (i.e. partId=JHK&partId=JHD). May not be combined with other ID filters |  |
**version** | Option<**String**> | Parasolid version |  |[default to 0]
**include_export_ids** | Option<**bool**> | Whether topolgy ids should be exported as parasolid attributes |  |[default to false]
**configuration** | Option<**String**> | Configuration string. |  |
**link_document_id** | Option<**String**> | Id of document that links to the document being accessed. This may provide additional access rights to the document. Allowed only with version (v) path parameter. |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_stl1

> export_stl1(did, wvm, wvmid, eid, part_ids, mode, grouping, scale, units, angle_tolerance, chord_tolerance, max_facet_width, min_facet_width, configuration, link_document_id)
Export Part Studio to STL

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**part_ids** | Option<**String**> | IDs of the parts to retrieve. Repeat query param to add more than one (i.e. partId=JHK&partId=JHD). May not be combined with other ID filters |  |
**mode** | Option<**String**> | Type of file: text, binary |  |[default to text]
**grouping** | Option<**bool**> | Whether parts should be exported as a group or individually in a .zip file |  |[default to true]
**scale** | Option<**f64**> | Scale for measurements. |  |[default to 1.0]
**units** | Option<**String**> | Name of base unit (meter, centimeter, millimeter, inch, foot, or yard) |  |[default to inch]
**angle_tolerance** | Option<**f64**> | Angle tolerance (in radians). This specifies the limit on the sum of the angular deviations of a tessellation chord from the tangent vectors at two chord endpoints. The specified value must be less than PI/2. This parameter currently has a default value chosen based on the complexity of the parts being tessellated. |  |
**chord_tolerance** | Option<**f64**> | Chord tolerance (in meters). This specifies the limit on the maximum deviation of a tessellation chord from the true surface/edge. This parameter currently has a default value chosen based on the size and complexity of the parts being tessellated. |  |
**max_facet_width** | Option<**f64**> | Max facet width. This specifies the limit on the size of any side of a tessellation facet. |  |
**min_facet_width** | Option<**f64**> | Max facet width. This specifies the limit on the size of any side of a tessellation facet. |  |
**configuration** | Option<**String**> | Configuration string. |  |
**link_document_id** | Option<**String**> | Id of document that links to the document being accessed. This may provide additional access rights to the document. Allowed only with version (v) path parameter. |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_body_details2

> crate::models::BtExportModelBodiesResponse get_body_details2(did, wvm, wvmid, eid, configuration, link_document_id)
Array of body information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**configuration** | Option<**String**> | Configuration string. |  |
**link_document_id** | Option<**String**> | Id of document that links to the document being accessed. This may provide additional access rights to the document. Allowed only with version (v) path parameter. |  |

### Return type

[**crate::models::BtExportModelBodiesResponse**](BTExportModelBodiesResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bounding_boxes2

> crate::models::BtBoundingBox get_bounding_boxes2(did, wvm, wvmid, eid, include_hidden, include_wire_bodies, configuration, link_document_id)
Mass properties of parts or a PartStudio.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**include_hidden** | Option<**bool**> | Whether or not to include bounding boxes for hidden parts. |  |[default to false]
**include_wire_bodies** | Option<**bool**> | Whether to include wire bodies in the bounding box. |  |[default to true]
**configuration** | Option<**String**> | Configuration string. |  |
**link_document_id** | Option<**String**> | Id of document that links to the document being accessed. This may provide additional access rights to the document. Allowed only with version (v) path parameter. |  |

### Return type

[**crate::models::BtBoundingBox**](BTBoundingBox.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_configuration1

> get_configuration1(did, wvm, wvmid, eid)
Get Configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_edges2

> crate::models::BtExportTessellatedEdgesResponse get_edges2(did, wvm, wvmid, eid, angle_tolerance, chord_tolerance, part_id, edge_id, configuration, link_document_id)
Tesselated edges from a PartStudio.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**angle_tolerance** | Option<**f64**> |  |  |
**chord_tolerance** | Option<**f64**> |  |  |
**part_id** | Option<[**Vec<String>**](String.md)> |  |  |
**edge_id** | Option<[**Vec<String>**](String.md)> |  |  |
**configuration** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> |  |  |

### Return type

[**crate::models::BtExportTessellatedEdgesResponse**](BTExportTessellatedEdgesResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_faces2

> crate::models::BtExportTessellatedFacesResponse get_faces2(did, wvm, wvmid, eid, angle_tolerance, chord_tolerance, max_facet_width, output_vertex_normals, output_facet_normals, output_texture_coordinates, output_index_table, part_id, face_id, output_error_faces, configuration, link_document_id)
Tesselated faces of the parts in the Part Studio.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**angle_tolerance** | Option<**f64**> | Angle tolerance (in radians). This specifies the limit on the sum of the angular deviations of a tessellation chord from the tangent vectors at two chord endpoints. The specified value must be less than PI/2. This parameter currently has a default value chosen based on the complexity of the parts being tessellated. |  |
**chord_tolerance** | Option<**f64**> | Chord tolerance (in meters). This specifies the limit on the maximum deviation of a tessellation chord from the true surface/edge. This parameter currently has a default value chosen based on the size and complexity of the parts being tessellated. |  |
**max_facet_width** | Option<**f64**> | Max facet width. This specifies the limit on the size of any side of a tessellation facet. |  |
**output_vertex_normals** | Option<**bool**> | If true, output vertex normals corresponding to surface normals at facet vertex points. |  |[default to false]
**output_facet_normals** | Option<**bool**> | Output facet normals. |  |[default to true]
**output_texture_coordinates** | Option<**bool**> | Output texture coordinates. |  |[default to false]
**output_index_table** | Option<**bool**> | Output index table. |  |[default to false]
**part_id** | Option<[**Vec<String>**](String.md)> | IDs of the parts to retrieve. Repeat query param to add more than one (i.e. partId=JHK&partId=JHD). May not be combined with other ID filters |  |
**face_id** | Option<[**Vec<String>**](String.md)> | IDs of the faces to tessellate (repeat query param to add more than one, i.e. faceId=JHK&faceId=JHD) |  |
**output_error_faces** | Option<**bool**> | Whether or not to output faces that cause an error |  |[default to false]
**configuration** | Option<**String**> | Configuration string. |  |
**link_document_id** | Option<**String**> | Id of document that links to the document being accessed. This may provide additional access rights to the document. Allowed only with version (v) path parameter. |  |

### Return type

[**crate::models::BtExportTessellatedFacesResponse**](BTExportTessellatedFacesResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feature_specs1

> get_feature_specs1(did, wvm, wvmid, eid)
Get Feature Specs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_features1

> crate::models::BtFeatureListResponse get_features1(did, wvm, wvmid, eid, feature_id, link_document_id, no_sketch_geometry, body)
Get Feature List

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**feature_id** | Option<[**Vec<String>**](String.md)> | ID of a feature; repeat query param to add more than one |  |
**link_document_id** | Option<**String**> | Id of document that links to the document being accessed. This may provide additional access rights to the document. Allowed only with version (v) path parameter. |  |
**no_sketch_geometry** | Option<**bool**> | Whether or not to output simple sketch info without geometry |  |[default to false]
**body** | Option<**String**> |  |  |

### Return type

[**crate::models::BtFeatureListResponse**](BTFeatureListResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v2+json;charset=UTF-8;qs=0.2

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mass_properties1

> crate::models::BtMassPropertiesBulkInfo get_mass_properties1(did, wvm, wvmid, eid, part_id, mass_as_group, configuration, link_document_id)
Mass properties of parts or a PartStudio.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**part_id** | Option<[**Vec<String>**](String.md)> | IDs of the parts to retrieve. Repeat query param to add more than one (i.e. partId=JHK&partId=JHD). May not be combined with other ID filters |  |
**mass_as_group** | Option<**bool**> | If true, specified parts will be evaluated as a single object instead of individually |  |[default to true]
**configuration** | Option<**String**> | Configuration string. |  |
**link_document_id** | Option<**String**> | Id of document that links to the document being accessed. This may provide additional access rights to the document. Allowed only with version (v) path parameter. |  |

### Return type

[**crate::models::BtMassPropertiesBulkInfo**](BTMassPropertiesBulkInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_named_views1

> crate::models::BtNamedViewsInfo get_named_views1(did, eid, skip_perspective)
Get Named Views

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**skip_perspective** | Option<**bool**> | Whether views with a perspective projection should be omitted. |  |[default to true]

### Return type

[**crate::models::BtNamedViewsInfo**](BTNamedViewsInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shaded_views2

> crate::models::BtShadedRenderDocumentResponse get_shaded_views2(did, wvm, wvmid, eid, view_matrix, output_height, output_width, pixel_size, edges, show_all_parts, include_surfaces, use_anti_aliasing, configuration, link_document_id)
Get Shaded Views

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**view_matrix** | Option<**String**> | 12-number view matrix (comma-separated), or one of the following named views: top, bottom, front, back, left, right The 12 entries in the view matrix form three rows and four columns, which is a linear transformation applied to the model itself. The matrix's first three columns maps the coordinate axes of the model to the coordinate axes of the view, and the fourth column translates the origin (in meters). The view coordinates have x pointing right, y pointing up, and z pointing towards the viewer, while a front view of the model has x pointing right, y pointing away from the viewer, and z pointing up. For example, the identity matrix viewMatrix=1,0,0,0,0,1,0,0,0,0,1,0 corresponds to the top view, and viewMatrix=0.612,0.612,0,0,-0.354,0.354,0.707,0,0.707,-0.707,0.707,0 corresponds (approximately) to the isometric view. The first three columns of the view matrix should be orthonormal and have a positive determinant.  If this is not the case, view behavior may be undefined. |  |[default to front]
**output_height** | Option<**i32**> | Output image height (in pixels) |  |[default to 500]
**output_width** | Option<**i32**> | Output image width (in pixels) |  |[default to 500]
**pixel_size** | Option<**f64**> | Height and width represented by each pixel (in meters). If the value is 0, the display will be sized to fit the output image dimensions. |  |[default to 0.003]
**edges** | Option<**String**> | The treatment to be applied to edges in the display. Options are show: show visible edges, hide: hide visible edges. |  |[default to show]
**show_all_parts** | Option<**bool**> | Whether or not all parts should be shown in the element, regardless of user setting. If false, the visibility setting made by the user will be reflected in the image. If true, all parts will be shown. |  |[default to false]
**include_surfaces** | Option<**bool**> | Whether or not surfaces should be shown in the element. It is applicable only when showAllParts is true. If false, surfaces will be excluded. If true, all surfaces will be shown. |  |[default to false]
**use_anti_aliasing** | Option<**bool**> | If true, an anti-aliasing factor will be used to smooth model boundaries in the final image result. If false, the image will be rasterized at the given resolution. Setting to true can have negative performance implications with respect to rendering time and memory usage. If a high-resolution image is requested and anti-aliasing is turned on, the server may not be able to fulfill the request. |  |[default to false]
**configuration** | Option<**String**> | Configuration string. |  |
**link_document_id** | Option<**String**> | Id of document that links to the document being accessed. This may provide additional access rights to the document. Allowed only with version (v) path parameter. |  |

### Return type

[**crate::models::BtShadedRenderDocumentResponse**](BTShadedRenderDocumentResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_translator_formats4

> Vec<crate::models::BtModelFormatInfo> get_translator_formats4(did, wid, eid, check_content)
Get Translation Formats

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wid** | **String** | Workspace ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**check_content** | Option<**bool**> | Whether the current content or lack thereof should be considered when determining the available formats. Empty part studios cannot be translated into any format. |  |[default to true]

### Return type

[**Vec<crate::models::BtModelFormatInfo>**](BTModelFormatInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## translate_format5

> crate::models::BtTranslationRequestInfo translate_format5(did, wv, wvid, eid, bt_translate_format_params)
Create Part Studio translation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wv** | **String** | One of w or v corresponding to whether a workspace or version was specified. | [required] |
**wvid** | **String** | Workspace (w) or Version (v) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**bt_translate_format_params** | [**BtTranslateFormatParams**](BtTranslateFormatParams.md) |  | [required] |

### Return type

[**crate::models::BtTranslationRequestInfo**](BTTranslationRequestInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## translate_ids

> crate::models::BtIdTranslationInfo translate_ids(did, wvm, wvmid, eid, bt_id_translation_params)
Id Translations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**bt_id_translation_params** | [**BtIdTranslationParams**](BtIdTranslationParams.md) |  | [required] |

### Return type

[**crate::models::BtIdTranslationInfo**](BTIdTranslationInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_configuration1

> update_configuration1(did, wvm, wvmid, eid, body)
Update Configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_feature1

> update_feature1(did, wid, eid, fid, body)
Update Feature

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wid** | **String** | Workspace ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**fid** | **String** | The id of the feature being updated. This id should be URL encoded and must match the featureId found in the serialized structure | [required] |
**body** | Option<**String**> | feature The serialized feature definition |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_features

> update_features(did, wid, eid, body)
Update Features

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wid** | **String** | Workspace ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**body** | Option<**String**> | feature The serialized feature definition |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_rollback

> update_rollback(did, wid, eid, body)
Update Feature Rollback

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wid** | **String** | Workspace ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**body** | Option<**String**> | The index at which the rollback index should be placed. Features  with entry index (0-based) higher than or equal to the value are rolled back. Value of -1 is treated  as an alias for \"end of feature list\". Otherwise the value must be in the range 0 to the number of  entries in the feature list |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

