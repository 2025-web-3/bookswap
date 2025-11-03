# JSON
#### JSON Error Codes
| Code  | Description            |
|-------|------------------------|
| 10000 | Unknown book.          |
| 10001 | Unknown sharing.       |
| 20000 | Invalid payload data.  |
| 20001 | Invalid path data.     |
| 20002 | Invalid query data.    |
| 20003 | Invalid header data.   |
| 20004 | Validation error.      |
| 20005 | Invalid credentials.   |
| 20007 | Database error.        |
| 20010 | Taken username error.  |
| 20011 | Taken email error.     |
| 30000 | Unauthorized.          |
| 30001 | Week password.         |
| 40000 | Missing access.        |

#### Example JSON Error Response
```json
{
  "code": 40000,
  "message": "Missing access"
}
```
