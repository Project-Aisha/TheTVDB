# \PeopleApi

All URIs are relative to *https://api4.thetvdb.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_people**](PeopleApi.md#get_all_people) | **GET** /people | 
[**get_people_base**](PeopleApi.md#get_people_base) | **GET** /people/{id} | 
[**get_people_extended**](PeopleApi.md#get_people_extended) | **GET** /people/{id}/extended | 
[**get_people_translation**](PeopleApi.md#get_people_translation) | **GET** /people/{id}/translations/{language} | 



## get_all_people

> models::GetAllPeople200Response get_all_people(page)


Returns a list of people base records with the basic attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f64**> | page number |  |

### Return type

[**models::GetAllPeople200Response**](getAllPeople_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_people_base

> models::GetPeopleBase200Response get_people_base(id)


Returns people base record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** | id | [required] |

### Return type

[**models::GetPeopleBase200Response**](getPeopleBase_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_people_extended

> models::GetPeopleExtended200Response get_people_extended(id, meta)


Returns people extended record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** | id | [required] |
**meta** | Option<**String**> | meta |  |

### Return type

[**models::GetPeopleExtended200Response**](getPeopleExtended_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_people_translation

> models::GetEpisodeTranslation200Response get_people_translation(id, language)


Returns people translation record

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

