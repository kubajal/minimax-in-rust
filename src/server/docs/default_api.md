# default_api

All URIs are relative to *https://localhost/v0*

Method | HTTP request | Description
------------- | ------------- | -------------
**computerMove**](default_api.md#computerMove) | **POST** /board | Computer move
**getCurrentBoardState**](default_api.md#getCurrentBoardState) | **GET** /board | Get board state


# **computerMove**
> serde_json::Value computerMove(board)
Computer move

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **board** | [**Board**](Board.md)| State of the board | 

### Return type

[**serde_json::Value**](object.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getCurrentBoardState**
> serde_json::Value getCurrentBoardState()
Get board state

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](object.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

