# User management

A simple web application made with intention of learning Rust. Main frameworks used are: `actix-web` (and related) and `diesel`.
To avoid blocking `actix-web` and workarounds `diesel-async` was also used even though it is not recommended for production.
Security is done through `actix-web-grants` and JWT is used for authentication and authorization.

Before starting the application database needs to be started. For that a [docker-compose](support/runtime/docker-compose.yml) file is provided. It starts a Postgresql database on port 5432.
Database url, pool size and jwt secret are configured through .env file.
To be able to access the application an JWT token is needed. To generate the token an unsecured endpoint is provided:

POST `/security`, it accepts a username and a list of permissions. Available permission are: `UM_USER_FIND_ALL`, `UM_USER_FIND_BY_ID`, `UM_USER_SAVE`,
`UM_USER_UPDATE` and `UM_USER_DELETE`.

An example request to generate the token would look like this:

```shell

curl -v --location --request POST http://localhost:8081/security -H 'Content-Type: application/json' -d '{ "username": "username", "permissions": ["UM_USER_FIND_ALL"] }'

```

When token is retrieved it can be used to access [user_api](src/user/user_api.rs). User api provides standard methods for
finding, creating, updating and deleting users.
An example request for finding all users would look like this:

```shell

 curl -v --location --request GET http://localhost:8081/users\?limit\=10\&offset\=0 -H 'Content-Type: application/json' -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2NzMzODE0MjgsInBlcm1pc3Npb25zIjpbIlVNX1VTRVJfRklORF9BTEwiXSwidXNlcm5hbWUiOiJ1c2VybmFtZSJ9.RJR83lWoLuOb1kGsHhp5S5kec-rven_jY6DoMICOsrU"

```
