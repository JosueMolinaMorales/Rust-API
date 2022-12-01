# Rust-API
- [Rust-API](#rust-api)
- [Modules](#modules)
  - [Auth Module](#auth-module)
    - [POST /auth/login](#post-authlogin)
      - [Authorization](#authorization)
      - [Request Body](#request-body)
      - [Response Body](#response-body)
      - [Potentional Errors](#potentional-errors)
    - [POST /auth/register](#post-authregister)
      - [Authorization](#authorization-1)
      - [Request Body](#request-body-1)
      - [Response Body](#response-body-1)
      - [Potentional Errors](#potentional-errors-1)
  - [Record Module](#record-module)
    - [GET /record/\<user\_id\>/all](#get-recorduser_idall)
      - [Authorization](#authorization-2)
      - [Request Body](#request-body-2)
      - [Response Body](#response-body-2)
      - [Potentional Errors](#potentional-errors-2)
    - [GET /record/](#get-record)
      - [Authorization](#authorization-3)
      - [Request Body](#request-body-3)
      - [Response Body](#response-body-3)
      - [Potentional Errors](#potentional-errors-3)
    - [POST /record](#post-record)
      - [Authorization](#authorization-4)
      - [Request Body](#request-body-4)
      - [Response Body](#response-body-4)
      - [Potentional Errors](#potentional-errors-4)
    - [POST /record/](#post-record-1)
      - [Authorization](#authorization-5)
      - [Request Body](#request-body-5)
      - [Response Body](#response-body-5)
      - [Potentional Errors](#potentional-errors-5)
    - [DELETE /record/](#delete-record)
      - [Authorization](#authorization-6)
      - [Request Body](#request-body-6)
      - [Response Body](#response-body-6)
      - [Potentional Errors](#potentional-errors-6)
  - [Search Module](#search-module)
    - [GET /search/record/\<user\_id\>?\&\&](#get-searchrecorduser_id)
      - [Parameters](#parameters)
      - [Authorization](#authorization-7)
      - [Request Body](#request-body-7)
      - [Response Body](#response-body-7)
      - [Potentional Errors](#potentional-errors-7)

# Modules
* [Auth Module](#auth-module)
* [Records Module](#record-module)
* [Search Module](#search-module)

## Auth Module

### POST /auth/login
Route to log a user in, returns the bearer token and user object

#### Authorization
No Auth Required

#### Request Body
```
{
    email: String,
    password: String
}
```
#### Response Body
```
{
    token: String,
    user: {
        name: String,
        email: String,
        username: String
    }
}
```

#### Potentional Errors

| Error Code | Error Reason |
| ---------- | ------------ |
| 400 | Email or Password is incorrect |

### POST /auth/register
Route to register a user, returns a bearer token and user object

#### Authorization
No Auth Required

#### Request Body
```
{
    name: String,
    username: String
    email: String,
    password: String
}
```
#### Response Body
```
{
    token: String,
    user: {
        name: String,
        email: String,
        username: String
    }
}
```

#### Potentional Errors

| Error Code | Error Reason |
| ---------- | ------------ |
| 400 | Email Exists, Username Exists |

## Record Module

### GET /record/<user_id>/all
Get all the records for a user

#### Authorization
A valid bearer token is required

#### Request Body
None

#### Response Body
```
{
    [
        {
            record_type: Secret || Password,
            _id: String,
            user_id: String,
            key: Option<String>,
            secret: Option<String>,
            service: Option<String>,
            password: Option<String>,
            email: Option<String>,
            username: Option<String>,
        }
    ]
}
```
#### Potentional Errors

| Error Code | Error Reason |
| ---------- | ------------ |
| 400 | User id is not a valid object id |
| 401 | User id and Id in token do not match |


### GET /record/<id>
Get a specific record

#### Authorization
A valid bearer token is required

#### Request Body
None

#### Response Body
```
{
    records: {
        record_type: Secret || Password,
        _id: String,
        user_id: String,
        key: Option<String>,
        secret: Option<String>,
        service: Option<String>,
        password: Option<String>,
        email: Option<String>,
        username: Option<String>,
    }
}
```
#### Potentional Errors

| Error Code | Error Reason |
| ---------- | ------------ |
| 400 | User id is not a valid object id |
| 401 | User id and Id in token do not match |
| 404 | Record was not found |

### POST /record
Create a new record

#### Authorization
A valid bearer token is required

#### Request Body
```
{
    record_type: "Secret" || "Password",
    key: Option<String>,
    secret: Option<String>,
    service: Option<String>,
    password: Option<String>,
    email: Option<String>,
    username: Option<String>,
}
```

*Notes*

* A 400 will be thrown if record_type is Secret and key or secret is not in body
* A 400 will be thrown if record_type is Secret and any password fields are passed in
* A 400 will be thrown if record_type is Password and service, password, email or username are not in body
    * Email or Username can be passed in. Both do not need to be passed in
* A 400 will be thrown if record_type is Password and any secret fields are passed in

#### Response Body
```
{
    id: String
}
```
#### Potentional Errors

| Error Code | Error Reason |
| ---------- | ------------ |
| 400 | User id is not a valid object id |
| 401 | User id and Id in token do not match |

### POST /record/<id>
Update a record

#### Authorization
A valid bearer token is required

#### Request Body
```
{
    service: Option<String>,
    password: Option<String>,
    email: Option<String>,
    username: Option<String>,
    key: Option<String>,
    secret: Option<String>,
}
```

*Notes*

* A 400 will be thrown if record_type is Secret and any password fields are passed in
* A 400 will be thrown if record_type is Password and any secret fields are passed in

#### Response Body
No body but response Code: 204
#### Potentional Errors

| Error Code | Error Reason |
| ---------- | ------------ |
| 400 | User id is not a valid object id |
| 401 | User id and Id in token do not match |
| 404 | Record was not found |

### DELETE /record/<id>
Delete a record

#### Authorization
A valid bearer token is required

#### Request Body
None

#### Response Body
No Body but reponse code is 204
#### Potentional Errors

| Error Code | Error Reason |
| ---------- | ------------ |
| 400 | User id is not a valid object id |
| 401 | User id and Id in token do not match |
| 404 | Record was not found |

## Search Module

### GET /search/record/<user_id>?<page>&<limit>&<query>
Search a users record

#### Parameters

| Parameter Name | Description |
| -------------- | ----------- |
| Page | The page of the search, used for pagination |
| Limit | The amount of records to show |
| Query | A text query to search for records |

#### Authorization
A valid bearer token is required

#### Request Body
None

#### Response Body

```
{
    records: {
        record_type: Secret || Password,
        _id: String,
        user_id: String,
        key: Option<String>,
        secret: Option<String>,
        service: Option<String>,
        password: Option<String>,
        email: Option<String>,
        username: Option<String>,
    }
}
```

#### Potentional Errors

| Error Code | Error Reason |
| ---------- | ------------ |
| 400 | User id is not a valid object id |
| 401 | User id and Id in token do not match |
