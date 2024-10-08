# \SeasonsApi

All URIs are relative to *https://api4.thetvdb.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_seasons**](SeasonsApi.md#get_all_seasons) | **GET** /seasons | 
[**get_season_base**](SeasonsApi.md#get_season_base) | **GET** /seasons/{id} | 
[**get_season_extended**](SeasonsApi.md#get_season_extended) | **GET** /seasons/{id}/extended | 
[**get_season_translation**](SeasonsApi.md#get_season_translation) | **GET** /seasons/{id}/translations/{language} | 
[**get_season_types**](SeasonsApi.md#get_season_types) | **GET** /seasons/types | 



## get_all_seasons

> models::GetAllSeasons200Response get_all_seasons(page)


returns list of seasons base records

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f64**> | page number |  |

### Return type

[**models::GetAllSeasons200Response**](getAllSeasons_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_season_base

> models::GetSeasonBase200Response get_season_base(id)


Returns season base record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** | id | [required] |

### Return type

[**models::GetSeasonBase200Response**](getSeasonBase_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_season_extended

> models::GetSeasonExtended200Response get_season_extended(id)


Returns season extended record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** | id | [required] |

### Return type

[**models::GetSeasonExtended200Response**](getSeasonExtended_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_season_translation

> models::GetEpisodeTranslation200Response get_season_translation(id, language)


Returns season translation record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** | id | [required] |
**language** | **String** | language | [required] |

### Return type

[**models::GetEpisodeTranslation200Response**](getEpisodeTranslation_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_season_types

> models::GetSeasonTypes200Response get_season_types()


Returns season type records

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetSeasonTypes200Response**](getSeasonTypes_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

