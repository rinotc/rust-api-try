export DATABASE_URL=postgres://user:password@localhost:5432/myapp
sea-orm-cli migrate fresh -d .
sea-orm-cli generate entity -u $DATABASE_URL -o ../infra-orm/src/orm