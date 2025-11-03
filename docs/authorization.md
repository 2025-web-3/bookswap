# Auth Session Object
Authenticating with the API is performed with the `Authorization` HTTP header in the format `Authorization: TOKEN`.
##### Example User Token Authorization
`Authorization: 3h3iWjA8YPPwxoBIxV5rxNKkGpUnLg`

# Auth
### Endpoints

#### Register Account
```http
POST /auth/register
```

##### JSON payload
| Field          | Type    | Description                   |
|----------------|---------|-------------------------------|
| `username`     | string  | The new account username.     |
| `email`        | string  | The new account email.        |
| `first_name`   | string  | The new user real first name. |
| `second_name`  | string  | The new user real second name.|
| `school_name`  | ?string | The new user current school.  |
| `password`     | string  | The new account password.     |

##### Response body
| Field   | Type                                     | Description                       |
|---------|------------------------------------------|-----------------------------------|
| `user`  | [User](./resources/users.md#user-object) | The user that session belongs to. |
| `token` | string                                   | The session token.                |

#### Login Account
```http
POST /auth/login
```

Email or username must be specified.

##### JSON payload
| Field      | Type    | Description                   |
|------------|---------|-------------------------------|
| `username` | ?string | The logging account username. |
| `email`    | ?string | The logging account email.    |
| `password` | string  | The logging account password. |

##### Response body
| Field   | Type                            | Description                       |
|---------|---------------------------------|-----------------------------------|
| `user`  | [User](./resources#user-object) | The user that session belongs to. |
| `token` | string                          | The session token.                |

