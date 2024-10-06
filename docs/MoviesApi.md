# \MoviesApi

All URIs are relative to *https://api4.thetvdb.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_movie**](MoviesApi.md#get_all_movie) | **GET** /movies | 
[**get_movie_base**](MoviesApi.md#get_movie_base) | **GET** /movies/{id} | 
[**get_movie_base_by_slug**](MoviesApi.md#get_movie_base_by_slug) | **GET** /movies/slug/{slug} | 
[**get_movie_extended**](MoviesApi.md#get_movie_extended) | **GET** /movies/{id}/extended | 
[**get_movie_translation**](MoviesApi.md#get_movie_translation) | **GET** /movies/{id}/translations/{language} | 
[**get_movies_filter**](MoviesApi.md#get_movies_filter) | **GET** /movies/filter | 



## get_all_movie

> models::GetAllMovie200Response get_all_movie(page)


returns list of movie base records

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f64**> | page number |  |

### Return type

[**models::GetAllMovie200Response**](getAllMovie_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_base

> models::GetMovieBase200Response get_movie_base(id)


Returns movie base record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** | id | [required] |

### Return type

[**models::GetMovieBase200Response**](getMovieBase_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_base_by_slug

> models::GetMovieBase200Response get_movie_base_by_slug(slug)


Returns movie base record search by slug

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** | slug | [required] |

### Return type

[**models::GetMovieBase200Response**](getMovieBase_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_extended

> models::GetMovieExtended200Response get_movie_extended(id, meta, short)


Returns movie extended record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **f64** | id | [required] |
**meta** | Option<**String**> | meta |  |
**short** | Option<**bool**> | reduce the payload and returns the short version of this record without characters, artworks and trailers. |  |

### Return type

[**models::GetMovieExtended200Response**](getMovieExtended_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_movie_translation

> models::GetEpisodeTranslation200Response get_movie_translation(id, language)


Returns movie translation record

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


## get_movies_filter

> models::GetMoviesFilter200Response get_movies_filter(country, lang, company, content_rating, genre, sort, status, year)


Search movies based on filter parameters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country** | **String** | country of origin | [required] |
**lang** | **String** | original language | [required] |
**company** | Option<**f64**> | production company |  |
**content_rating** | Option<**f64**> | content rating id base on a country |  |
**genre** | Option<**f64**> | genre |  |
**sort** | Option<**String**> | sort by results |  |
**status** | Option<**f64**> | status |  |
**year** | Option<**f64**> | release year |  |

### Return type

[**models::GetMoviesFilter200Response**](getMoviesFilter_200_response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

