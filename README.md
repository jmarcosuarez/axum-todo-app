# axum-todo.app

## To start:

### Start database

```
docker compose up -d --wait
```

### Start up the project

```
cargo watch -x run
```

# Previous setup:

## Seed initial data (see database/init.sql)

```
//?
```

```
docker compose up

// get interactive shell inside this container (run above)

docker compose exec database /bin/bash // where database is the service name

root@1313b5f6764a:/# psql -U postgres -d postgres //<username><database>

postgres=# \dt // shows list of tables
postgres=# select \* from tasks; // any regular sql allowed here - don't forget the semicolon

ctrl+c // to quit

//--------------------------------

docker compose down // removes database containers but doesnt remove volume

docker volume ls // to reset the db remove the value
docker volume remove intro-axum-todo_db-data
docker compose up -d --wait // deamon mode = run in the background - now will rerun the init script here: database/init.sql
                            // --wait=there's a healthcheck script in the docker-compose file

```

## To set up sea-orm

### Generate models

```
sea-orm-cli generate entity -o src/database
```

### Logs

```
docker compose logs database // name of the service
```

# Notes

- If `address already in use` shown when trying to start database do:

```
sudo lsof -i -P -n | grep  5432 // this shows the current process which is using the database
```

then

```
sudo kill <current process id>  // this will kill it
```
