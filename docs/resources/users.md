# User Object

### User Structure

| Field          | Type        | Description                                  |
|----------------|-------------|----------------------------------------------|
| id             | snowflake   | The ID of the user                           |
| username       | string      | The username of the user                     |
| email          | string      | The email address of the user                |
| first_name     | string      | The first name of the user                   |
| second_name    | string      | The second name of the user                  |
| school_name    | ?string     | The name of the user's school                |
| permissions    | Permissions | The permissions of the user (excluded from serialization) |

# Endpoints

## Get Current User
```http
GET /users/@me
```
Returns the current [user](#user-object) object.

## Get User by ID
```http
GET /users/{user.id}
```
Returns the [user](#user-object) object by given ID.

## Get Current User's Requested Books
```http
GET /users/@me/requests
```
Returns a list of [book request](./books.md#book-request-structure) objects.

## Get Current User's Book Requests
```http
GET /users/@me/books/requests
```
Returns a list of [book request](./books.md#book-request-structure) objects.

## Add New Book to User Profile
```http
POST /users/@me/books
```
Returns a [book sharing](./books.md#book-sharing-structure) object.

##### JSON Payload

| Field     | Type    | Validation                                      | Description                          |
|-----------|---------|-------------------------------------------------|--------------------------------------|
| isbn      | string  | Length: 10-13 characters                        | The ISBN of the book                 |
| comment   | ?string | Length: 2-1024 characters (if provided)         | Additional comments about the book   |
| condition | integer | None                                            | The condition of the book            |

##### Possible Errors

| Error      | HTTP Status | Description |
|------------|-------------|-------------|
| Unauthorized | 401 Unauthorized | User is not authenticated |

## Get User Book Sharing
```http
GET /users/{user.id}/books/{book.id}
```
Returns a [book sharing](./books.md#book-sharing-structure) object.

## Get All Available Books from User
```http
GET /users/{user.id}/books
```
Returns a list of [book](./books.md#book-structure) objects.

## Request Book from User
```http
POST /users/{user.id}/books/{book.id}/request
```
Returns a [book request](./books.md#book-request-structure) object.

##### JSON Query

| Field       | Type      | Description                          |
|-------------|-----------|--------------------------------------|
| sharing_id  | snowflake | The ID of the sharing (if known)     |

##### Possible Errors

| Error      | HTTP Status | Description |
|------------|-------------|-------------|
| BookRequest | 403 Forbidden | 1. If sharing ID was not specified and there are more than 1 sharing in user profile<br>2. If book ID is in your sharings |
| UnknownBook | 404 Not Found | The specified book does not exist |
| UnknownSharing | 404 Not Found | The specified sharing does not exist |
