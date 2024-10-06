# \EpisodesApi

All URIs are relative to *https://api4.thetvdb.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_episodes**](EpisodesApi.md#get_all_episodes) | **GET** /episodes | 
[**get_episode_base**](EpisodesApi.md#get_episode_base) | **GET** /episodes/{id} | 
[**get_episode_extended**](EpisodesApi.md#get_episode_extended) | **GET** /episodes/{id}/extended | 
[**get_episode_translation**](EpisodesApi.md#get_episode_translation) | **GET** /episodes/{id}/translations/{language} | 



## get_all_episodes

> models::GetAllEpisodes200Response get_all_episodes(page)


Returns a list of episodes base records with the basic attributes.<br> Note that all episodes are returned, even those that may not be included in a series' default season order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f64**> | page number |  |

### Return type

[**models::GetAllEpisodes200Response**](getAllEpisodes_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_episode_base

> models::GetEpisodeBase200Response get_episode_base(id)


Returns episode base record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** | id | [required] |

### Return type

[**models::GetEpisodeBase200Response**](getEpisodeBase_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_episode_extended

> models::GetEpisodeExtended200Response get_episode_extended(id, meta)


Returns episode extended record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** | id | [required] |
**meta** | Option<**String**> | meta |  |

### Return type

[**models::GetEpisodeExtended200Response**](getEpisodeExtended_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_episode_translation

> models::GetEpisodeTranslation200Response get_episode_translation(id, language)


Returns episode translation record

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

