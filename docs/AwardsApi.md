# \AwardsApi

All URIs are relative to *https://api4.thetvdb.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_awards**](AwardsApi.md#get_all_awards) | **GET** /awards | 
[**get_award**](AwardsApi.md#get_award) | **GET** /awards/{id} | 
[**get_award_extended**](AwardsApi.md#get_award_extended) | **GET** /awards/{id}/extended | 



## get_all_awards

> models::GetAllAwards200Response get_all_awards()


Returns a list of award base records

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetAllAwards200Response**](getAllAwards_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_award

> models::GetAward200Response get_award(id)


Returns a single award base record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** | id | [required] |

### Return type

[**models::GetAward200Response**](getAward_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_award_extended

> models::GetAwardExtended200Response get_award_extended(id)


Returns a single award extended record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** | id | [required] |

### Return type

[**models::GetAwardExtended200Response**](getAwardExtended_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

