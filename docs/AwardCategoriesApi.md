# \AwardCategoriesApi

All URIs are relative to *https://api4.thetvdb.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_award_category**](AwardCategoriesApi.md#get_award_category) | **GET** /awards/categories/{id} | 
[**get_award_category_extended**](AwardCategoriesApi.md#get_award_category_extended) | **GET** /awards/categories/{id}/extended | 



## get_award_category

> models::GetAwardCategory200Response get_award_category(id)


Returns a single award category base record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** | id | [required] |

### Return type

[**models::GetAwardCategory200Response**](getAwardCategory_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_award_category_extended

> models::GetAwardCategoryExtended200Response get_award_category_extended(id)


Returns a single award category extended record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** | id | [required] |

### Return type

[**models::GetAwardCategoryExtended200Response**](getAwardCategoryExtended_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

