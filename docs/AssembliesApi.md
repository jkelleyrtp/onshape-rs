# \AssembliesApi

All URIs are relative to *https://cad.onshape.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_feature**](AssembliesApi.md#add_feature) | **Post** /api/assemblies/d/{did}/{wvm}/{wvmid}/e/{eid}/features | 
[**create_assembly**](AssembliesApi.md#create_assembly) | **Post** /api/assemblies/d/{did}/w/{wid} | 
[**create_instance**](AssembliesApi.md#create_instance) | **Post** /api/assemblies/d/{did}/w/{wid}/e/{eid}/instances | 
[**delete_feature**](AssembliesApi.md#delete_feature) | **Delete** /api/assemblies/d/{did}/w/{wid}/e/{eid}/features/featureid/{fid} | 
[**delete_instance**](AssembliesApi.md#delete_instance) | **Delete** /api/assemblies/d/{did}/w/{wid}/e/{eid}/instance/nodeid/{nid} | 
[**get_assembly_definition**](AssembliesApi.md#get_assembly_definition) | **Get** /api/assemblies/d/{did}/{wvm}/{wvmid}/e/{eid} | 
[**get_bill_of_materials**](AssembliesApi.md#get_bill_of_materials) | **Get** /api/assemblies/d/{did}/{wvm}/{wvmid}/e/{eid}/bom | 
[**get_bounding_boxes**](AssembliesApi.md#get_bounding_boxes) | **Get** /api/assemblies/d/{did}/{wvm}/{wvmid}/e/{eid}/boundingboxes | 
[**get_feature_specs**](AssembliesApi.md#get_feature_specs) | **Get** /api/assemblies/d/{did}/{wvm}/{wvmid}/e/{eid}/featurespecs | 
[**get_features**](AssembliesApi.md#get_features) | **Get** /api/assemblies/d/{did}/{wvm}/{wvmid}/e/{eid}/features | 
[**get_named_views**](AssembliesApi.md#get_named_views) | **Get** /api/assemblies/d/{did}/e/{eid}/namedViews | 
[**get_or_create_bill_of_materials_element**](AssembliesApi.md#get_or_create_bill_of_materials_element) | **Post** /api/assemblies/d/{did}/w/{wid}/e/{eid}/bomelement | 
[**get_shaded_views**](AssembliesApi.md#get_shaded_views) | **Get** /api/assemblies/d/{did}/{wvm}/{wvmid}/e/{eid}/shadedviews | 
[**get_translator_formats**](AssembliesApi.md#get_translator_formats) | **Get** /api/assemblies/d/{did}/w/{wid}/e/{eid}/translationformats | 
[**insert_transformed_instances**](AssembliesApi.md#insert_transformed_instances) | **Post** /api/assemblies/d/{did}/w/{wid}/e/{eid}/transformedinstances | 
[**transform_occurrences**](AssembliesApi.md#transform_occurrences) | **Post** /api/assemblies/d/{did}/w/{wid}/e/{eid}/occurrencetransforms | 
[**translate_format**](AssembliesApi.md#translate_format) | **Post** /api/assemblies/d/{did}/{wv}/{wvid}/e/{eid}/translations | 
[**update_feature**](AssembliesApi.md#update_feature) | **Post** /api/assemblies/d/{did}/w/{wid}/e/{eid}/features/featureid/{fid} | 



## add_feature

> add_feature(did, wvm, wvmid, eid, body)


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
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_assembly

> create_assembly(did, wid, bt_model_element_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_model_element_params** | [**BtModelElementParams**](BtModelElementParams.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_instance

> create_instance(did, wid, eid, bt_assembly_instance_definition_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bt_assembly_instance_definition_params** | [**BtAssemblyInstanceDefinitionParams**](BtAssemblyInstanceDefinitionParams.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_feature

> delete_feature(did, wid, eid, fid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**fid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_instance

> delete_instance(did, eid, wid, nid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**nid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_assembly_definition

> get_assembly_definition(did, wvm, wvmid, eid, link_document_id, include_mate_features, include_non_solids, include_mate_connectors, configuration)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**link_document_id** | Option<**String**> |  |  |
**include_mate_features** | Option<**bool**> |  |  |
**include_non_solids** | Option<**bool**> |  |  |
**include_mate_connectors** | Option<**bool**> |  |  |
**configuration** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bill_of_materials

> get_bill_of_materials(did, wvm, wvmid, eid, metadata_workspace_id, bom_column_ids, indented, multi_level, generate_if_absent, link_document_id, configuration)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**metadata_workspace_id** | Option<**String**> |  |  |[default to ]
**bom_column_ids** | Option<[**Vec<String>**](String.md)> |  |  |
**indented** | Option<**bool**> |  |  |[default to true]
**multi_level** | Option<**bool**> |  |  |[default to false]
**generate_if_absent** | Option<**bool**> |  |  |[default to false]
**link_document_id** | Option<**String**> |  |  |
**configuration** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bounding_boxes

> get_bounding_boxes(did, wvm, wvmid, eid, link_document_id, include_hidden, display_state_id, configuration, exploded_view_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**link_document_id** | Option<**String**> |  |  |
**include_hidden** | Option<**bool**> |  |  |
**display_state_id** | Option<**String**> |  |  |
**configuration** | Option<**String**> |  |  |
**exploded_view_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feature_specs

> get_feature_specs(did, wvm, wvmid, eid)


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
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_features

> get_features(did, wvm, wvmid, eid, feature_id, link_document_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**feature_id** | Option<[**Vec<String>**](String.md)> |  |  |
**link_document_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_named_views

> get_named_views(did, eid, skip_perspective)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**skip_perspective** | Option<**bool**> |  |  |[default to true]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_or_create_bill_of_materials_element

> get_or_create_bill_of_materials_element(did, wid, eid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shaded_views

> get_shaded_views(did, wvm, wvmid, eid, link_document_id, view_matrix, output_height, output_width, pixel_size, edges, show_all_parts, include_surfaces, use_anti_aliasing, display_state_id, configuration, exploded_view_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**link_document_id** | Option<**String**> |  |  |
**view_matrix** | Option<**String**> |  |  |[default to front]
**output_height** | Option<**i32**> |  |  |[default to 500]
**output_width** | Option<**i32**> |  |  |[default to 500]
**pixel_size** | Option<**f64**> |  |  |[default to 0.003]
**edges** | Option<**String**> |  |  |[default to show]
**show_all_parts** | Option<**bool**> |  |  |[default to false]
**include_surfaces** | Option<**bool**> |  |  |[default to true]
**use_anti_aliasing** | Option<**bool**> |  |  |[default to false]
**display_state_id** | Option<**String**> |  |  |
**configuration** | Option<**String**> |  |  |
**exploded_view_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_translator_formats

> get_translator_formats(did, wid, eid, check_content)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**check_content** | Option<**bool**> |  |  |[default to true]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insert_transformed_instances

> insert_transformed_instances(did, eid, wid, bt_assembly_transformed_instances_definition_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_assembly_transformed_instances_definition_params** | [**BtAssemblyTransformedInstancesDefinitionParams**](BtAssemblyTransformedInstancesDefinitionParams.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transform_occurrences

> transform_occurrences(did, eid, wid, bt_assembly_transform_definition_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_assembly_transform_definition_params** | [**BtAssemblyTransformDefinitionParams**](BtAssemblyTransformDefinitionParams.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## translate_format

> translate_format(did, wv, wvid, eid, bt_translate_format_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wv** | **String** |  | [required] |
**wvid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bt_translate_format_params** | [**BtTranslateFormatParams**](BtTranslateFormatParams.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_feature

> update_feature(did, wid, eid, fid, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**fid** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

