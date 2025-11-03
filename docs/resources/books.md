# Book Object

## Book Structure

| Field        | Type      | Description                   |
|--------------|-----------|-------------------------------|
| id           | snowflake | The ID of the book            |
| isbn         | ?string   | The ISBN of the book          |
| title        | string    | The title of the book         |
| description  | string    | The description of the book   |
| author       | string    | The author of the book        |
| subjects     | ?string   | The subjects of the book      |
| pages        | ?integer  | The number of pages           |
| cover_url    | ?string   | The URL of the book cover     |
| publish_date | ?datetime | The publication date          |

## Book Request Structure

| Field            | Type        | Description                              |
|------------------|-------------|------------------------------------------|
| id               | snowflake   | The ID of the book request               |
| book_sharing_id  | snowflake   | The ID of the book sharing               |
| book_id          | snowflake   | The ID of the requested book             |
| borrower_id      | snowflake   | The ID of the user requesting the book   |
| is_accepted      | ?boolean    | Whether the request was accepted         |
| accepted_at      | ?datetime   | When the request was accepted            |
| borrowed_at      | ?datetime   | When the book was borrowed               |
| return_at        | ?datetime   | When the book should be returned         |

## Book Condition Enum

| Value          | Integer | Description                                                               |
|----------------|---------|---------------------------------------------------------------------------|
| MintCondition  | 0       | The book is in perfect condition, as if new                               |
| NearPerfect    | 1       | The book is nearly perfect, with minimal signs of use                    |
| GentlyUsed     | 2       | The book has been gently used, showing minimal wear                      |
| ShowsSomeWear  | 3       | The book shows signs of use, such as abrasions or lightly torn pages     |
| HeavilyUsed    | 4       | The book is significantly damaged, with medium to heavy tears            |

## Book Sharing Structure

| Field        | Type          | Description                          |
|--------------|---------------|--------------------------------------|
| id           | snowflake     | The ID of the book sharing           |
| book         | [Book](#book-structure) | The book being shared                |
| comment      | ?string       | Additional comments about the sharing|
| holder_id    | snowflake     | The ID of the current book holder    |
| condition    | [BookCondition](#book-condition-enum) | The condition of the shared book     |

# Endpoints

## Get Book By ID
```http
GET /books/{book.id}
```
Returns the [book](#book-structure) object for a given book ID.

## Get Unique Book Holders
```http
GET /books/{book.id}/holders
```
Returns a list of [user](./users.md#user-object) objects who are current holders of the specified book.

## Search Books
```http
GET /books/search?query=
```
Returns a list of [book](#book-structure) objects by their title (query).

## Change Request Status
```http
PATCH /books/requests/{request.id}
```
Returns a 204 (No Content) code on success.

### JSON Payload
| Field       | Type    | Description           |
|-------------|---------|-----------------------|
| is_accepted | boolean | The ID of the request |
